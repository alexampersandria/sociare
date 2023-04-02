use uuid::Uuid;

use chrono::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub struct User {
  pub id: Uuid,
  pub name: String,
  pub email: String,
  pub created_at: DateTime<Utc>,
}

impl User {
  pub fn new(name: String, email: String) -> User {
    User {
      id: Uuid::new_v4(),
      name,
      email,
      created_at: Utc::now(),
    }
  }
}
