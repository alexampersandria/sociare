use chrono::prelude::*;
use uuid::Uuid;

#[derive(Debug, PartialEq, Clone)]
pub struct Transaction {
  pub id: Uuid,
  pub from: Uuid,
  pub to: Uuid,
  pub amount: i64,
  pub method: String,
  pub confirmed: bool,
  pub created_at: DateTime<Utc>,
  pub edited_at: DateTime<Utc>,
  pub confirmed_at: DateTime<Utc>,
  pub deleted: bool,
}

#[allow(dead_code)]
impl Transaction {
  pub fn new(from: Uuid, to: Uuid, amount: i64, method: String) -> Self {
    Transaction {
      id: Uuid::new_v4(),
      from,
      to,
      amount,
      method,
      confirmed: false,
      created_at: Utc::now(),
      edited_at: Utc::now(),
      confirmed_at: Utc::now(),
      deleted: false,
    }
  }

  pub fn set_amount(&mut self, amount: i64) {
    if !self.confirmed && !self.deleted {
      self.amount = amount;
      self.edited_at = Utc::now();
    }
  }

  pub fn set_method(&mut self, method: String) {
    if !self.confirmed && !self.deleted {
      self.method = method;
      self.edited_at = Utc::now();
    }
  }

  pub fn delete(&mut self) {
    if !self.confirmed && !self.deleted {
      self.amount = 0;
      self.method = "".to_string();
      self.deleted = true;
      self.edited_at = Utc::now();
    }
  }

  pub fn confirm(&mut self) {
    self.confirmed = true;
    self.confirmed_at = Utc::now();
  }
}
