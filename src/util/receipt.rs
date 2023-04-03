use super::unix_time;
use uuid::Uuid;

#[derive(Debug, PartialEq, Clone)]
pub struct Receipt {
  pub id: String,
  pub user: String,
  pub amount: i64,
  pub info: String,
  pub created_at: i64,
  pub deleted: bool,
}

#[allow(dead_code)]
impl Receipt {
  pub fn new(user: String, amount: i64, info: String) -> Self {
    Receipt {
      id: Uuid::new_v4().to_string(),
      user,
      amount,
      info,
      created_at: unix_time(),
      deleted: false,
    }
  }
}
