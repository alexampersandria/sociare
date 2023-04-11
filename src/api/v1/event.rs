use crate::{schema, util};
use diesel::RunQueryDsl;
use uuid::Uuid;

pub fn log_event(
  user_id: &str,
  group_id: &str,
  event: &str,
  message_id: Option<String>,
  receipt_id: Option<String>,
  transaction_id: Option<String>,
) -> Result<usize, diesel::result::Error> {
  let group_event = util::GroupEvent {
    id: Uuid::new_v4().to_string(),
    user_id: user_id.to_string(),
    group_id: group_id.to_string(),
    event: event.to_string(),
    message_id,
    receipt_id,
    transaction_id,
    created_at: util::unix_ms(),
  };

  let mut conn = crate::establish_connection();

  diesel::insert_into(schema::group_events::table)
    .values(&group_event)
    .execute(&mut conn)
}

pub fn log_message(
  user_id: &str,
  group_id: &str,
  message_id: &str,
) -> Result<usize, diesel::result::Error> {
  log_event(
    user_id,
    group_id,
    "create_message",
    Some(message_id.to_string()),
    None,
    None,
  )
}

pub fn log_receipt(
  user_id: &str,
  group_id: &str,
  receipt_id: &str,
) -> Result<usize, diesel::result::Error> {
  log_event(
    user_id,
    group_id,
    "create_receipt",
    None,
    Some(receipt_id.to_string()),
    None,
  )
}

pub fn log_transaction(
  user_id: &str,
  group_id: &str,
  transaction_id: &str,
) -> Result<usize, diesel::result::Error> {
  log_event(
    user_id,
    group_id,
    "create_transaction",
    None,
    None,
    Some(transaction_id.to_string()),
  )
}

pub fn log_simple(
  user_id: &str,
  group_id: &str,
  event: &str,
) -> Result<usize, diesel::result::Error> {
  log_event(user_id, group_id, event, None, None, None)
}
