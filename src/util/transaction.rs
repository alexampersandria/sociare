use super::timestamp;
use uuid::Uuid;

#[derive(Debug, PartialEq, Clone)]
pub struct Transaction {
  pub id: Uuid,
  pub from: Uuid,
  pub to: Uuid,
  pub amount: u64,
  pub method: String,
  pub confirmed: bool,
  pub created_at: u64,
  pub edited_at: u64,
  pub confirmed_at: u64,
  pub deleted: bool,
}

#[allow(dead_code)]
impl Transaction {
  pub fn new(from: Uuid, to: Uuid, amount: u64, method: String) -> Self {
    Transaction {
      id: Uuid::new_v4(),
      from,
      to,
      amount,
      method,
      confirmed: false,
      created_at: timestamp::unix_timestamp(),
      edited_at: 0,
      confirmed_at: 0,
      deleted: false,
    }
  }

  pub fn set_amount(&mut self, amount: u64) {
    if !self.confirmed && !self.deleted {
      self.amount = amount;
      self.edited_at = timestamp::unix_timestamp();
    }
  }

  pub fn set_method(&mut self, method: String) {
    if !self.confirmed && !self.deleted {
      self.method = method;
      self.edited_at = timestamp::unix_timestamp();
    }
  }

  pub fn delete(&mut self) {
    if !self.confirmed && !self.deleted {
      self.amount = 0;
      self.method = "".to_string();
      self.deleted = true;
      self.edited_at = timestamp::unix_timestamp();
    }
  }

  pub fn confirm(&mut self) {
    self.confirmed = true;
    self.confirmed_at = timestamp::unix_timestamp();
  }
}
