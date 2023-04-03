use std::collections::HashMap;

use rusty_money::iso;
use uuid::Uuid;

use crate::util::debt::Debt;
use crate::util::message::Message;
use crate::util::receipt::Receipt;
use crate::util::transaction::Transaction;
use crate::util::user::User;

use crate::util::random_emoji::random_emoji;

#[derive(Debug, PartialEq, Clone)]
pub struct GroupUser {
  pub id: Uuid,
  pub nickname: String,
  pub active: bool,
}

impl GroupUser {
  pub fn new(id: Uuid) -> Self {
    GroupUser {
      id,
      nickname: "".to_string(),
      active: true,
    }
  }
  pub fn set_nickname(&mut self, nickname: &str) {
    self.nickname = nickname.to_string();
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Group {
  pub id: Uuid,
  pub name: String,
  pub emoji: String,
  pub users: Vec<GroupUser>,
  pub created_at: i64,
  pub receipts: Vec<Receipt>,
  pub transactions: Vec<Transaction>,
  pub messages: Vec<Message>,
  pub currency: iso::Currency,
}

#[allow(dead_code)]
impl Group {
  pub fn new(name: String, users: &[User], currency: iso::Currency) -> Group {
    let group_users = users.iter().map(|u| GroupUser::new(u.id)).collect();
    Group {
      id: Uuid::new_v4(),
      name,
      emoji: random_emoji(),
      users: group_users,
      created_at: 0,
      receipts: vec![],
      transactions: vec![],
      messages: vec![],
      currency,
    }
  }

  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn set_emoji(&mut self, emoji: String) {
    if emojis::get(&emoji).is_some() {
      self.emoji = emoji;
    }
  }

  pub fn add_receipt(&mut self, receipt: Receipt) {
    self.receipts.push(receipt);
  }

  pub fn get_receipt(&mut self, uuid: Uuid) -> Option<&mut Receipt> {
    self.receipts.iter_mut().find(|r| r.id == uuid)
  }

  pub fn add_transaction(&mut self, transaction: Transaction) {
    self.transactions.push(transaction);
  }

  pub fn get_transaction(&mut self, uuid: Uuid) -> Option<&mut Transaction> {
    self.transactions.iter_mut().find(|t| t.id == uuid)
  }

  pub fn add_user(&mut self, user: &User) {
    if let Some(found_user) = self.get_user(user.id) {
      found_user.active = true;
    } else {
      self.users.push(GroupUser::new(user.id));
    }
  }

  pub fn get_user(&mut self, uuid: Uuid) -> Option<&mut GroupUser> {
    self.users.iter_mut().find(|u| u.id == uuid)
  }

  pub fn get_active_users(&self) -> Vec<Uuid> {
    self
      .users
      .iter()
      .filter(|u| u.active)
      .map(|u| u.id)
      .collect()
  }

  pub fn add_message(&mut self, message: Message) {
    self.messages.push(message);
  }

  pub fn get_message(&mut self, uuid: Uuid) -> Option<&mut Message> {
    self.messages.iter_mut().find(|m| m.id == uuid)
  }

  pub fn balance(&mut self) -> HashMap<Uuid, i64> {
    let mut balance: HashMap<Uuid, i64> = HashMap::new();
    for user in self.users.iter() {
      if user.active {
        balance.insert(user.id, 0);
      }
    }
    for receipt in self.receipts.iter() {
      if !receipt.deleted {
        balance.insert(receipt.user, balance[&receipt.user] + receipt.amount);
      }
    }
    for transaction in self.transactions.iter() {
      if !transaction.deleted && transaction.confirmed {
        balance.insert(
          transaction.from,
          balance[&transaction.from] + transaction.amount,
        );
        balance.insert(
          transaction.to,
          balance[&transaction.to] - transaction.amount,
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
            payments.push(Debt {
              from: *debtor.0,
              to: *creditor.0,
              amount: transaction_amount,
            });
          }
        }
      }
    }

    // #TODO: Reduce payments to minimum

    payments
  }
}
