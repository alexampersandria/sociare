use super::unix_time;
use uuid::Uuid;

#[derive(Debug, PartialEq, Clone)]
pub struct Message {
  pub id: String,
  pub user: String,
  pub message: String,
  pub created_at: i64,
  pub deleted: bool,
}

#[allow(dead_code)]
impl Message {
  pub fn new(user: String, message: String) -> Self {
    Message {
      id: Uuid::new_v4().to_string(),
      user,
      message,
      created_at: unix_time(),
      deleted: false,
    }
  }
}
