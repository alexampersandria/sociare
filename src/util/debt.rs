use crate::{schema, util};
use diesel::{Associations, Identifiable, Insertable, Queryable, Selectable};
use uuid::Uuid;

#[derive(
  Associations, Clone, Debug, Identifiable, Insertable, PartialEq, Queryable, Selectable,
)]
#[diesel(belongs_to(util::Group))]
#[diesel(table_name = schema::debts)]
pub struct Debt {
  pub id: String,
  pub group_id: String,
  pub from_id: String,
  pub to_id: String,
  pub amount: i64,
  pub created_at: i64,
}

impl Debt {
  pub fn new(group_id: &str, from_id: &str, to_id: &str, amount: i64) -> Self {
    Debt {
      id: Uuid::new_v4().to_string(),
      group_id: group_id.to_string(),
      from_id: from_id.to_string(),
      to_id: to_id.to_string(),
      amount,
      created_at: util::unix_ms(),
    }
  }
}

#[cfg(test)]
mod ci_unit {
  use super::*;

  #[test]
  fn new() {
    let group = String::from("group1");
    let from = String::from("user1");
    let to = String::from("user2");
    let amount = 100;
    let debt = Debt::new(&group, &from, &to, amount);
    assert_eq!(debt.group_id, group);
    assert_eq!(debt.from_id, from);
    assert_eq!(debt.to_id, to);
    assert_eq!(debt.amount, amount);
  }
}
