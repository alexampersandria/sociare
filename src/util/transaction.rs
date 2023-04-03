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
      deleted: false,
    }
  }
}
