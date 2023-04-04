use super::unix_time;
use crate::schema;
use crate::util::Group;
use diesel::{Associations, Identifiable, Insertable, Queryable, Selectable};
use uuid::Uuid;

#[derive(
  Insertable, Queryable, Selectable, Identifiable, Associations, Debug, Clone, PartialEq,
)]
#[diesel(belongs_to(Group))]
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
  pub fn new(group_id: String, from_id: String, to_id: String, amount: i64) -> Self {
    Debt {
      id: Uuid::new_v4().to_string(),
      group_id,
      from_id,
      to_id,
      amount,
      created_at: unix_time(),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn new() {
    let group = String::from("group1");
    let from = String::from("user1");
    let to = String::from("user2");
    let amount = 100;
    let debt = Debt::new(group.clone(), from.clone(), to.clone(), amount);
    assert_eq!(debt.group_id, group);
    assert_eq!(debt.from_id, from);
    assert_eq!(debt.to_id, to);
    assert_eq!(debt.amount, amount);
  }
}
