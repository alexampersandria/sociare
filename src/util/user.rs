use crate::schema;
use crate::util;
use diesel::{Identifiable, Insertable, Queryable, Selectable};
use uuid::Uuid;

#[derive(Insertable, Identifiable, Selectable, Queryable, Debug, Clone, PartialEq)]
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
  pub fn new(username: &str, password: &str, name: &str, email: &str, phone: &str) -> User {
    User {
      id: Uuid::new_v4().to_string(),
      username: username.to_string(),
      name: name.to_string(),
      email: email.to_string(),
      phone: phone.to_string(),
      password: password.to_string(),
      created_at: util::unix_time(),
    }
  }
}

#[cfg(test)]
mod ci_unit {
  use super::*;

  #[test]
  fn new() {
    let username = "testuser".to_string();
    let password = "password123".to_string();
    let name = "Test User".to_string();
    let email = "testuser@example.com".to_string();
    let phone = "123-456-7890".to_string();

    let user = User::new(&username, &password, &name, &email, &phone);

    assert_eq!(user.username, username);
    assert_eq!(user.password, password);
    assert_eq!(user.name, name);
    assert_eq!(user.email, email);
    assert_eq!(user.phone, phone);
  }
}
