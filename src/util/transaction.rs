use super::unix_time;
use crate::schema;
use crate::util::Group;
use diesel::{Associations, Identifiable, Insertable, Queryable, Selectable};
use uuid::Uuid;

#[derive(
  Insertable, Queryable, Selectable, Identifiable, Associations, Debug, Clone, PartialEq,
)]
#[diesel(belongs_to(Group))]
#[diesel(table_name = schema::transactions)]
pub struct Transaction {
  pub id: String,
  pub group_id: String,
  pub from_id: String,
  pub to_id: String,
  pub debt_id: String,
  pub amount: i64,
  pub method: String,
  pub confirmed: bool,
  pub deleted: bool,
  pub created_at: i64,
}

#[allow(dead_code)]
impl Transaction {
  pub fn new(
    group_id: &str,
    from_id: &str,
    to_id: &str,
    debt_id: &str,
    amount: i64,
    method: &str,
  ) -> Self {
    Transaction {
      id: Uuid::new_v4().to_string(),
      group_id: group_id.to_string(),
      from_id: from_id.to_string(),
      to_id: to_id.to_string(),
      debt_id: debt_id.to_string(),
      amount,
      method: method.to_string(),
      confirmed: false,
      created_at: unix_time(),
      deleted: false,
    }
  }
}

#[cfg(test)]
mod unit {
  use super::*;

  #[test]
  fn new() {
    let group_id = "group_id";
    let from_id = "from_id";
    let to_id = "to_id";
    let debt_id = "debt_id";
    let amount = 100;
    let method = "method";
    let transaction = Transaction::new(group_id, from_id, to_id, debt_id, amount, method);
    assert_eq!(transaction.group_id, group_id);
    assert_eq!(transaction.from_id, from_id);
    assert_eq!(transaction.to_id, to_id);
    assert_eq!(transaction.debt_id, debt_id);
    assert_eq!(transaction.amount, amount);
    assert_eq!(transaction.method, method);
    assert!(!transaction.confirmed);
    assert!(!transaction.deleted);
  }
}
