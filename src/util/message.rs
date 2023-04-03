use chrono::prelude::*;
use uuid::Uuid;

#[derive(Debug, PartialEq, Clone)]
pub struct Message {
  pub id: Uuid,
  pub user: Uuid,
  pub message: String,
  pub created_at: DateTime<Utc>,
  pub deleted: bool,
}

#[allow(dead_code)]
impl Message {
  pub fn new(user: Uuid, message: String) -> Self {
    Message {
      id: Uuid::new_v4(),
      user,
      message,
      created_at: Utc::now(),
      deleted: false,
    }
  }
}
