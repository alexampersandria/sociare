use crate::{schema, util};
use diesel::{Identifiable, Insertable, Queryable, Selectable};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Clone, Debug, Identifiable, Insertable, PartialEq, Queryable, Selectable)]
#[diesel(table_name = schema::groups)]
pub struct Group {
  pub id: String,
  pub name: String,
  pub emoji: String,
  pub currency: String,
  pub created_at: i64,
}

impl Group {
  pub fn new(name: &str, emoji: &str, currency: &str) -> Self {
    Group {
      id: Uuid::new_v4().to_string(),
      name: name.to_string(),
      emoji: emoji.to_string(),
      currency: currency.to_string(),
      created_at: util::unix_ms(),
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
  pub users: Vec<util::User>,
  pub receipts: Vec<util::Receipt>,
  pub transactions: Vec<util::Transaction>,
  pub messages: Vec<util::Message>,
  pub debts: Vec<util::Debt>,
}

#[allow(dead_code)]
impl FullGroup {
  pub fn new(name: &str, users: Vec<util::User>, emoji: &str, currency: &str) -> FullGroup {
    let group = Group::new(name, emoji, currency);
    FullGroup {
      group,
      users,
      receipts: vec![],
      transactions: vec![],
      messages: vec![],
      debts: vec![],
    }
  }

  pub fn add_receipt(&mut self, receipt: util::Receipt) {
    self.receipts.push(receipt);
  }

  pub fn find_receipt(&mut self, uuid: String) -> Option<&mut util::Receipt> {
    self.receipts.iter_mut().find(|r| r.id == uuid)
  }

  pub fn add_transaction(&mut self, transaction: util::Transaction) {
    self.transactions.push(transaction);
  }

  pub fn find_transaction(&mut self, uuid: String) -> Option<&mut util::Transaction> {
    self.transactions.iter_mut().find(|t| t.id == uuid)
  }

  pub fn add_user(&mut self, user: util::User) {
    self.users.push(user);
  }

  pub fn find_user(&mut self, uuid: String) -> Option<&mut util::User> {
    self.users.iter_mut().find(|u| u.id == uuid)
  }

  pub fn find_active_users(&self) -> Vec<String> {
    self.users.iter().map(|u| u.id.clone()).collect()
  }

  pub fn add_message(&mut self, message: util::Message) {
    self.messages.push(message);
  }

  pub fn find_message(&mut self, uuid: String) -> Option<&mut util::Message> {
    self.messages.iter_mut().find(|m| m.id == uuid)
  }

  pub fn add_debt(&mut self, debt: util::Debt) {
    self.debts.push(debt);
  }

  pub fn find_debt(&mut self, uuid: String) -> Option<&mut util::Debt> {
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

  pub fn debts(&mut self) -> Vec<util::Debt> {
    let balance = self.balance();
    let mut payments = Vec::with_capacity(balance.len() * balance.len());

    for debtor in &balance {
      for creditor in &balance {
        if debtor.0 != creditor.0 {
          let transaction_amount = (*creditor.1 - *debtor.1) / (balance.len() as i64);
          if transaction_amount > 0 {
            payments.push(util::Debt::new(
              &self.group.id,
              debtor.0,
              creditor.0,
              transaction_amount,
            ));
          }
        }
      }
    }

    payments
  }

  pub fn update_debts(&mut self) {
    self.debts = self.debts();
  }
}

#[cfg(test)]
mod ci_unit {
  use super::*;

  #[test]
  fn new() {
    let group = Group::new("Test Group", "🎉", "USD");
    assert_eq!(group.name, "Test Group");
    assert_eq!(group.emoji, "🎉");
    assert_eq!(group.currency, "USD");
  }

  #[test]
  fn add_receipt() {
    let mut group = FullGroup::new("Test Group", vec![], "🎉", "USD");
    let receipt = util::Receipt::new(&group.group.id, "Test User", 100, "");
    group.add_receipt(receipt);
    assert_eq!(group.receipts.len(), 1);
    assert_eq!(group.receipts[0].user_id, "Test User");
  }

  #[test]
  fn find_receipt() {
    let mut group = FullGroup::new("Test Group", vec![], "🎉", "USD");
    let receipt = util::Receipt::new(&group.group.id, "User 1", 100, "");
    group.add_receipt(receipt.clone());

    // Find existing receipt
    let found_receipt = group.find_receipt(receipt.id);
    assert!(found_receipt.is_some());
    assert_eq!(found_receipt.unwrap().user_id, "User 1");

    // Find non-existing receipt
    let non_existent_receipt = group.find_receipt("nonexistentid".to_string());
    assert!(non_existent_receipt.is_none());
  }

  #[test]
  fn add_transaction() {
    let mut group = FullGroup::new("Test Group", vec![], "🎉", "USD");
    let transaction =
      util::Transaction::new(&group.group.id, "User 1", "User 2", 100, "Test Transaction");
    group.add_transaction(transaction);
    assert_eq!(group.transactions.len(), 1);
    assert_eq!(group.transactions[0].from_id, "User 1");
  }

  #[test]
  fn find_transaction() {
    let mut group = FullGroup::new("Test Group", vec![], "🎉", "USD");
    let transaction =
      util::Transaction::new(&group.group.id, "User 1", "User 2", 100, "Test Transaction");
    group.add_transaction(transaction.clone());

    // Find existing transaction
    let found_transaction = group.find_transaction(transaction.id);
    assert!(found_transaction.is_some());
    assert_eq!(found_transaction.unwrap().from_id, "User 1");

    // Find non-existing transaction
    let non_existent_transaction = group.find_transaction("nonexistentid".to_string());
    assert!(non_existent_transaction.is_none());
  }

  #[test]
  fn add_user() {
    let mut group = FullGroup::new("Test Group", vec![], "🎉", "USD");
    let user = util::User::new("Test User", "1", "2", "3", "4");
    group.add_user(user);
    assert_eq!(group.users.len(), 1);
    assert_eq!(group.users[0].username, "Test User");
  }

  #[test]
  fn find_user() {
    let mut group = FullGroup::new("Test Group", vec![], "🎉", "USD");
    let user = util::User::new("Test User", "1", "2", "3", "4");
    group.add_user(user.clone());

    // Find existing user
    let found_user = group.find_user(user.id);
    assert!(found_user.is_some());
    assert_eq!(found_user.unwrap().username, "Test User");

    // Find non-existing user
    let non_existent_user = group.find_user("nonexistentid".to_string());
    assert!(non_existent_user.is_none());
  }

  #[test]
  fn add_message() {
    let mut group = FullGroup::new("Test Group", vec![], "🎉", "USD");
    let message = util::Message::new(&group.group.id, "Test User", "Test Message");
    group.add_message(message);
    assert_eq!(group.messages.len(), 1);
    assert_eq!(group.messages[0].user_id, "Test User");
  }

  #[test]
  fn find_message() {
    let mut group = FullGroup::new("Test Group", vec![], "🎉", "USD");
    let message = util::Message::new(&group.group.id, "Test User", "Test Message");
    group.add_message(message.clone());

    // Find existing message
    let found_message = group.find_message(message.id);
    assert!(found_message.is_some());
    assert_eq!(found_message.unwrap().user_id, "Test User");

    // Find non-existing message
    let non_existent_message = group.find_message("nonexistentid".to_string());
    assert!(non_existent_message.is_none());
  }

  // Add more tests for other functions here...
}
