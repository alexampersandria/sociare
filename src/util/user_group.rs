use crate::schema;
use crate::util;
use diesel::{Associations, Identifiable, Insertable, Queryable, Selectable};
use uuid::Uuid;

#[derive(
  Identifiable, Selectable, Associations, Insertable, Queryable, Debug, Clone, PartialEq,
)]
#[diesel(table_name = schema::users_groups)]
#[diesel(belongs_to(util::User))]
#[diesel(belongs_to(util::Group))]
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
      created_at: util::unix_time(),
    }
  }
}
