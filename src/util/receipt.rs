use super::timestamp;
use uuid::Uuid;

#[derive(Debug, PartialEq, Clone)]
pub struct Receipt {
  pub id: Uuid,
  pub user: Uuid,
  pub amount: u64,
  pub info: String,
  pub created_at: u64,
  pub edited_at: u64,
  pub deleted: bool,
}

#[allow(dead_code)]
impl Receipt {
  pub fn new(user: Uuid, amount: u64, info: String) -> Self {
    Receipt {
      id: Uuid::new_v4(),
      user,
      amount,
      info,
      created_at: timestamp::unix_timestamp(),
      edited_at: 0,
      deleted: false,
    }
  }

  pub fn set_amount(&mut self, amount: u64) {
    if !self.deleted {
      self.amount = amount;
      self.edited_at = timestamp::unix_timestamp();
    }
  }

  pub fn set_info(&mut self, info: String) {
    if !self.deleted {
      self.info = info;
      self.edited_at = timestamp::unix_timestamp();
    }
  }

  pub fn delete(&mut self) {
    self.amount = 0;
    self.info = "".to_string();
    self.deleted = true;
    self.edited_at = timestamp::unix_timestamp();
  }
}
