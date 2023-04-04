use super::unix_time;
use crate::schema;
use crate::util::Group;
use diesel::{Associations, Identifiable, Insertable, Queryable, Selectable};
use uuid::Uuid;

#[derive(
  Insertable, Queryable, Selectable, Identifiable, Associations, Debug, Clone, PartialEq,
)]
#[diesel(belongs_to(Group))]
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
  pub fn new(group_id: String, user_id: String, content: String) -> Self {
    Message {
      id: Uuid::new_v4().to_string(),
      group_id,
      user_id,
      content,
      created_at: unix_time(),
      deleted: false,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn new() {
    let group_id = String::from("test_group");
    let user_id = Uuid::new_v4().to_string();
    let message = String::from("Hello, world!");
    let new_message = Message::new(group_id.clone(), user_id.clone(), message.clone());
    assert_eq!(new_message.group_id, group_id);
    assert_eq!(new_message.user_id, user_id);
    assert_eq!(new_message.content, message);
    assert!(!new_message.deleted);
    assert_ne!(new_message.id, "");
    assert!(new_message.created_at > 0);
  }
}
