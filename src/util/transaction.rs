use super::unix_time;
use uuid::Uuid;

#[derive(Debug, PartialEq, Clone)]
pub struct Transaction {
  pub id: String,
  pub from: String,
  pub to: String,
  pub amount: i64,
  pub method: String,
  pub confirmed: bool,
  pub created_at: i64,
  pub deleted: bool,
}

#[allow(dead_code)]
impl Transaction {
  pub fn new(from: String, to: String, amount: i64, method: String) -> Self {
    Transaction {
      id: Uuid::new_v4().to_string(),
      from,
      to,
      amount,
      method,
      confirmed: false,
      created_at: unix_time(),
      deleted: false,
    }
  }
}
