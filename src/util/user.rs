use super::unix_time;
use crate::schema;
use diesel::{Insertable, Queryable};
use uuid::Uuid;

#[derive(Insertable, Queryable, Debug, Clone, PartialEq)]
#[diesel(table_name = schema::users)]
pub struct User {
  pub id: String,
  pub username: String,
  pub name: String,
  pub email: String,
  pub phone: String,
  pub password: String,
  pub created_at: i64,
}

impl User {
  pub fn new(
    username: String,
    password: String,
    name: String,
    email: String,
    phone: String,
  ) -> User {
    User {
      id: Uuid::new_v4().to_string(),
      username,
      name,
      email,
      phone,
      password,
      created_at: unix_time(),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn new() {
    let username = "testuser".to_string();
    let password = "password123".to_string();
    let name = "Test User".to_string();
    let email = "testuser@example.com".to_string();
    let phone = "123-456-7890".to_string();

    let user = User::new(
      username.clone(),
      password.clone(),
      name.clone(),
      email.clone(),
      phone.clone(),
    );

    assert_eq!(user.username, username);
    assert_eq!(user.password, password);
    assert_eq!(user.name, name);
    assert_eq!(user.email, email);
    assert_eq!(user.phone, phone);
  }
}
