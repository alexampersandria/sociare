use chrono::prelude::*;
use uuid::Uuid;

#[derive(Debug, PartialEq, Clone)]
pub struct Receipt {
  pub id: Uuid,
  pub user: Uuid,
  pub amount: i64,
  pub info: String,
  pub created_at: DateTime<Utc>,
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
      deleted: false,
    }
  }
}
