use crate::{schema, util};
use diesel::{Associations, Identifiable, Insertable, Queryable, Selectable};
use uuid::Uuid;

#[derive(
  Associations, Clone, Debug, Identifiable, Insertable, PartialEq, Queryable, Selectable,
)]
#[diesel(belongs_to(util::Group))]
#[diesel(table_name = schema::receipts)]
pub struct Receipt {
  pub id: String,
  pub group_id: String,
  pub user_id: String,
  pub amount: i64,
  pub info: String,
  pub created_at: i64,
  pub deleted: bool,
}

#[allow(dead_code)]
impl Receipt {
  pub fn new(group_id: &str, user_id: &str, amount: i64, info: &str) -> Self {
    Receipt {
      id: Uuid::new_v4().to_string(),
      group_id: group_id.to_string(),
      user_id: user_id.to_string(),
      amount,
      info: info.to_string(),
      created_at: util::unix_time(),
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
    let user_id = String::from("test_user");
    let amount = 100;
    let info = String::from("Test receipt");
    let receipt = Receipt::new(&group_id, &user_id, amount, &info);
    assert_eq!(receipt.group_id, group_id);
    assert_eq!(receipt.user_id, user_id);
    assert_eq!(receipt.amount, amount);
    assert_eq!(receipt.info, info);
    assert!(!receipt.deleted);
    assert_ne!(receipt.id, "");
    assert!(receipt.created_at > 0);
  }
}
