use super::unix_time;
use crate::schema;
use crate::util::Group;
use diesel::{Associations, Identifiable, Insertable, Queryable, Selectable};
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

/**
UserGroup is a junction table between users and groups.
*/
#[derive(
  Identifiable, Selectable, Associations, Insertable, Queryable, Debug, Clone, PartialEq,
)]
#[diesel(table_name = schema::users_groups)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Group))]
#[diesel(primary_key(user_id, group_id))]
pub struct UserGroup {
  pub id: String,
  pub user_id: String,
  pub group_id: String,
  pub nickname: String,
  pub is_admin: bool,
  pub active: bool,
  pub created_at: i64,
}

impl UserGroup {
  pub fn new(user_id: &str, group_id: &str) -> UserGroup {
    UserGroup {
      id: Uuid::new_v4().to_string(),
      user_id: user_id.to_string(),
      group_id: group_id.to_string(),
      nickname: "".to_string(),
      is_admin: false,
      active: true,
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
