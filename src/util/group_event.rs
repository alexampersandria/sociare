use crate::{schema, util};
use diesel::{Associations, Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
  Associations,
  Clone,
  Debug,
  Deserialize,
  Identifiable,
  Insertable,
  PartialEq,
  Queryable,
  Selectable,
  Serialize,
)]
#[diesel(table_name = schema::group_events)]
#[diesel(belongs_to(util::Group))]
#[diesel(belongs_to(util::User))]
#[diesel(belongs_to(util::Message))]
#[diesel(belongs_to(util::Receipt))]
#[diesel(belongs_to(util::Transaction))]
#[diesel(primary_key(user_id, group_id))]
pub struct GroupEvent {
  pub id: String,
  pub user_id: String,
  pub group_id: String,
  pub event: String,
  pub message_id: Option<String>,
  pub receipt_id: Option<String>,
  pub transaction_id: Option<String>,
  pub created_at: i64,
}

impl GroupEvent {
  pub fn new(user_id: &str, group_id: &str, event: &str) -> GroupEvent {
    GroupEvent {
      id: Uuid::new_v4().to_string(),
      user_id: user_id.to_string(),
      group_id: group_id.to_string(),
      event: event.to_string(),
      message_id: None,
      receipt_id: None,
      transaction_id: None,
      created_at: util::unix_ms(),
    }
  }

  pub fn new_message(user_id: &str, group_id: &str, message_id: &str) -> GroupEvent {
    GroupEvent {
      id: Uuid::new_v4().to_string(),
      user_id: user_id.to_string(),
      group_id: group_id.to_string(),
      event: "message".to_string(),
      message_id: Some(message_id.to_string()),
      receipt_id: None,
      transaction_id: None,
      created_at: util::unix_ms(),
    }
  }

  pub fn new_receipt(user_id: &str, group_id: &str, receipt_id: &str) -> GroupEvent {
    GroupEvent {
      id: Uuid::new_v4().to_string(),
      user_id: user_id.to_string(),
      group_id: group_id.to_string(),
      event: "receipt".to_string(),
      message_id: None,
      receipt_id: Some(receipt_id.to_string()),
      transaction_id: None,
      created_at: util::unix_ms(),
    }
  }

  pub fn new_transaction(user_id: &str, group_id: &str, transaction_id: &str) -> GroupEvent {
    GroupEvent {
      id: Uuid::new_v4().to_string(),
      user_id: user_id.to_string(),
      group_id: group_id.to_string(),
      event: "transaction".to_string(),
      message_id: None,
      receipt_id: None,
      transaction_id: Some(transaction_id.to_string()),
      created_at: util::unix_ms(),
    }
  }
}
