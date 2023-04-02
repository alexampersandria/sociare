use chrono::prelude::*;
use uuid::Uuid;

#[derive(Debug, PartialEq, Clone)]
pub struct Receipt {
  pub id: Uuid,
  pub user: Uuid,
  pub amount: i64,
  pub info: String,
  pub created_at: DateTime<Utc>,
  pub edited_at: DateTime<Utc>,
  pub deleted: bool,
}

#[allow(dead_code)]
impl Receipt {
  pub fn new(user: Uuid, amount: i64, info: String) -> Self {
    Receipt {
      id: Uuid::new_v4(),
      user,
      amount,
      info,
      created_at: Utc::now(),
      edited_at: Utc::now(),
      deleted: false,
    }
  }

  pub fn set_amount(&mut self, amount: i64) {
    if !self.deleted {
      self.amount = amount;
      self.edited_at = Utc::now();
    }
  }

  pub fn set_info(&mut self, info: String) {
    if !self.deleted {
      self.info = info;
      self.edited_at = Utc::now();
    }
  }

  pub fn delete(&mut self) {
    self.amount = 0;
    self.info = "".to_string();
    self.deleted = true;
    self.edited_at = Utc::now();
  }
}
