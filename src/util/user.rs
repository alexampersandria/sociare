use uuid::Uuid;

use super::timestamp;

#[derive(Debug, PartialEq, Clone)]
pub struct User {
  pub id: Uuid,
  pub name: String,
  pub email: String,
  pub created_at: u64,
}

impl User {
  pub fn new(name: String, email: String) -> User {
    User {
      id: Uuid::new_v4(),
      name,
      email,
      created_at: timestamp::unix_timestamp(),
    }
  }
}
