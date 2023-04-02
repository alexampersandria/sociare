use super::timestamp;
use uuid::Uuid;

#[derive(Debug, PartialEq, Clone)]
pub struct Message {
  pub id: Uuid,
  pub user: Uuid,
  pub message: String,
  pub created_at: u64,
  pub edited_at: u64,
  pub deleted: bool,
}

#[allow(dead_code)]
impl Message {
  pub fn new(user: Uuid, message: String) -> Self {
    Message {
      id: Uuid::new_v4(),
      user,
      message,
      created_at: timestamp::unix_timestamp(),
      edited_at: 0,
      deleted: false,
    }
  }

  pub fn set_message(&mut self, message: String) {
    self.message = message;
    self.edited_at = timestamp::unix_timestamp();
  }

  pub fn delete(&mut self) {
    self.message = "".to_string();
    self.deleted = true;
    self.edited_at = timestamp::unix_timestamp();
  }
}
