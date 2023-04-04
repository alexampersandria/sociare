use super::{unix_time, Message};
use crate::schema;
use crate::util::debt::Debt;
use crate::util::random_emoji::random_emoji;
use crate::util::receipt::Receipt;
use crate::util::transaction::Transaction;
use crate::util::user::User;
use diesel::{Identifiable, Insertable, Queryable, Selectable};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Identifiable, Selectable, Debug, PartialEq, Insertable, Queryable, Clone)]
#[diesel(table_name = schema::groups)]
pub struct Group {
  pub id: String,
  pub name: String,
  pub emoji: String,
  pub currency: String,
  pub created_at: i64,
}

impl Group {
  pub fn new(name: String, emoji: String, currency: String) -> Self {
    Group {
      id: Uuid::new_v4().to_string(),
      name,
      emoji,
      currency,
      created_at: unix_time(),
    }
  }
}

/**
A [Group] with all of its relations:
- users
- receipts
- transactions
- messages
 */
#[derive(Debug, PartialEq, Clone)]
pub struct FullGroup {
  pub group: Group,
  /// junction table between users and groups
  pub users: Vec<User>,
  pub receipts: Vec<Receipt>,
  pub transactions: Vec<Transaction>,
  pub messages: Vec<Message>,
  pub debts: Vec<Debt>,
}

#[allow(dead_code)]
impl FullGroup {
  pub fn new(name: String, users: Vec<User>, currency: String) -> FullGroup {
    let group = Group::new(name, random_emoji(), currency);
    FullGroup {
      group,
      users,
      receipts: vec![],
      transactions: vec![],
      messages: vec![],
      debts: vec![],
    }
  }

  pub fn add_receipt(&mut self, receipt: Receipt) {
    self.receipts.push(receipt);
  }

  pub fn find_receipt(&mut self, uuid: String) -> Option<&mut Receipt> {
    self.receipts.iter_mut().find(|r| r.id == uuid)
  }

  pub fn add_transaction(&mut self, transaction: Transaction) {
    self.transactions.push(transaction);
  }

  pub fn find_transaction(&mut self, uuid: String) -> Option<&mut Transaction> {
    self.transactions.iter_mut().find(|t| t.id == uuid)
  }

  pub fn add_user(&mut self, user: &User) {
    self.users.push(user.clone());
  }

  pub fn find_user(&mut self, uuid: String) -> Option<&mut User> {
    self.users.iter_mut().find(|u| u.id == uuid)
  }

  pub fn find_active_users(&self) -> Vec<String> {
    self.users.iter().map(|u| u.id.clone()).collect()
  }

  pub fn add_message(&mut self, message: Message) {
    self.messages.push(message);
  }

  pub fn find_message(&mut self, uuid: String) -> Option<&mut Message> {
    self.messages.iter_mut().find(|m| m.id == uuid)
  }

  pub fn add_debt(&mut self, debt: Debt) {
    self.debts.push(debt);
  }

  pub fn find_debt(&mut self, uuid: String) -> Option<&mut Debt> {
    self.debts.iter_mut().find(|d| d.id == uuid)
  }

  pub fn balance(&mut self) -> HashMap<String, i64> {
    let mut balance: HashMap<String, i64> = HashMap::new();
    for user in self.users.iter() {
      balance.insert(user.id.clone(), 0);
    }
    for receipt in self.receipts.iter() {
      if !receipt.deleted {
        balance.insert(
          receipt.user_id.clone(),
          balance.get(&receipt.user_id).unwrap_or(&0) + receipt.amount,
        );
      }
    }
    for transaction in self.transactions.iter() {
      if !transaction.deleted && transaction.confirmed {
        balance.insert(
          transaction.from_id.clone(),
          balance.get(&transaction.from_id).unwrap_or(&0) + transaction.amount,
        );
        balance.insert(
          transaction.to_id.clone(),
          balance.get(&transaction.to_id).unwrap_or(&0) - transaction.amount,
        );
      }
    }
    balance
  }

  pub fn total(&mut self) -> i64 {
    let mut total: i64 = 0;
    for balance in self.balance().values() {
      total += balance;
    }
    total
  }

  pub fn debts(&mut self) -> Vec<Debt> {
    let balance = self.balance();
    let mut payments: Vec<Debt> = vec![];

    for debtor in &balance {
      for creditor in &balance {
        if debtor.0 != creditor.0 {
          let transaction_amount: i64 = (*creditor.1 - *debtor.1) / (balance.len() as i64);
          if transaction_amount > 0 {
            payments.push(Debt::new(
              self.group.id.clone(),
              debtor.0.clone(),
              creditor.0.clone(),
              transaction_amount,
            ));
          }
        }
      }
    }

    // #TODO: Reduce payments to minimum

    payments
  }

  pub fn update_debts(&mut self) {
    self.debts = self.debts();
  }
}
