use super::unix_time;
use crate::schema;
use diesel::{Insertable, Queryable};
use uuid::Uuid;

#[derive(Insertable, Queryable, Debug, Clone)]
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
