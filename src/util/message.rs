use crate::{schema, util};
use diesel::{Associations, Identifiable, Insertable, Queryable, Selectable};
use uuid::Uuid;

#[derive(
  Associations, Clone, Debug, Identifiable, Insertable, PartialEq, Queryable, Selectable,
)]
#[diesel(belongs_to(util::Group))]
#[diesel(table_name = schema::messages)]
pub struct Message {
  pub id: String,
  pub group_id: String,
  pub user_id: String,
  pub content: String,
  pub created_at: i64,
  pub deleted: bool,
}

#[allow(dead_code)]
impl Message {
  pub fn new(group_id: &str, user_id: &str, content: &str) -> Self {
    Message {
      id: Uuid::new_v4().to_string(),
      group_id: group_id.to_string(),
      user_id: user_id.to_string(),
      content: content.to_string(),
      created_at: util::unix_ms(),
      deleted: false,
    }
  }
}

#[cfg(test)]
mod ci_unit {
  use super::*;

  #[test]
  fn new() {
    let group_id = String::from("test_group");
    let user_id = Uuid::new_v4().to_string();
    let message = String::from("Hello, world!");
    let new_message = Message::new(&group_id, &user_id, &message);
    assert_eq!(new_message.group_id, group_id);
    assert_eq!(new_message.user_id, user_id);
    assert_eq!(new_message.content, message);
    assert!(!new_message.deleted);
    assert_ne!(new_message.id, "");
    assert!(new_message.created_at > 0);
  }
}
