use crate::{schema, util};
use diesel::{Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
  Clone, Debug, Deserialize, Identifiable, Insertable, PartialEq, Queryable, Selectable, Serialize,
)]
#[diesel(table_name = schema::users)]
pub struct User {
  pub id: String,
  pub username: String,
  pub name: String,
  pub email: String,
  pub password: String,
  pub created_at: i64,
  pub mobilepay: Option<String>,
  pub paypal_me: Option<String>,
}

impl User {
  pub fn new(username: &str, password: &str, name: &str, email: &str) -> User {
    User {
      id: Uuid::new_v4().to_string(),
      username: username.to_string(),
      name: name.to_string(),
      email: email.to_string(),
      password: password.to_string(),
      created_at: util::unix_ms(),
      mobilepay: None,
      paypal_me: None,
    }
  }
  pub fn new_with_mobilepay(
    username: &str,
    password: &str,
    name: &str,
    email: &str,
    mobilepay: &str,
  ) -> User {
    User {
      id: Uuid::new_v4().to_string(),
      username: username.to_string(),
      name: name.to_string(),
      email: email.to_string(),
      password: password.to_string(),
      created_at: util::unix_ms(),
      mobilepay: Option::from(mobilepay.to_string()),
      paypal_me: None,
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

    let user = User::new(&username, &password, &name, &email);

    assert_eq!(user.username, username);
    assert_eq!(user.password, password);
    assert_eq!(user.name, name);
    assert_eq!(user.email, email);
    assert!(user.mobilepay.is_none());
    assert!(user.paypal_me.is_none());
  }

  #[test]
  fn new_with_mobilepay() {
    let username = "testuser".to_string();
    let password = "password123".to_string();
    let name = "Test User".to_string();
    let email = "testuser@example.com".to_string();
    let mobilepay = "123-456-7890".to_string();

    let user = User::new_with_mobilepay(&username, &password, &name, &email, &mobilepay);

    assert_eq!(user.username, username);
    assert_eq!(user.password, password);
    assert_eq!(user.name, name);
    assert_eq!(user.email, email);
    assert_eq!(user.mobilepay.unwrap(), mobilepay);
    assert!(user.paypal_me.is_none());
  }
}
