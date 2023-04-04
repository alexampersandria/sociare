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
    group_id: String,
    from_id: String,
    to_id: String,
    debt_id: String,
    amount: i64,
    method: String,
  ) -> Self {
    Transaction {
      id: Uuid::new_v4().to_string(),
      group_id,
      from_id,
      to_id,
      debt_id,
      amount,
      method,
      confirmed: false,
      created_at: unix_time(),
      deleted: false,
    }
  }
}
