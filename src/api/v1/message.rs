use crate::api;
use crate::establish_connection;
use crate::schema;
use crate::schema::users_groups;
use crate::util;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use poem::web::Json;
use poem::Request;
use poem::{handler, web::Path};
use serde::Deserialize;
use serde::Serialize;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize)]
pub struct MessageDetails {
  pub message: util::Message,
  pub group_events: Option<Vec<util::GroupEvent>>,
}

#[handler]
pub fn get(req: &Request, Path(message): Path<String>) -> String {
  let mut conn = establish_connection();

  let session = api::auth::from_request(req);

  if let Some(session) = session {
    let users_groups = users_groups::table
      .filter(users_groups::user_id.eq(&session.id))
      .select(users_groups::group_id)
      .first::<String>(&mut conn);

    let result = schema::messages::table
      .filter(schema::messages::id.eq(&message))
      .filter(schema::messages::group_id.eq_any(users_groups))
      .select(util::Message::as_select())
      .first::<util::Message>(&mut conn);

    if let Ok(result) = result {
      let mut message_details = MessageDetails {
        message: result,
        group_events: None,
      };

      let message_group_events = schema::group_events::table
        .filter(schema::group_events::group_id.eq(&message_details.message.group_id))
        .filter(schema::group_events::message_id.eq(&message_details.message.id))
        .select(util::GroupEvent::as_select())
        .get_results::<util::GroupEvent>(&mut conn);

      if let Ok(message_group_events) = message_group_events {
        message_details.group_events = Some(message_group_events);
      }

      serde_json::to_string_pretty(&message_details)
        .unwrap_or("{\"error\": \"internal_server_error\"}".to_string())
    } else {
      "{\"error\": \"message_not_found\"}".to_string()
    }
  } else {
    "{\"error\": \"invalid_session\"}".to_string()
  }
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct NewMessage {
  #[validate(length(min = 1), length(max = 4096))]
  content: String,
  #[validate(length(min = 1), length(max = 96))]
  group_id: String,
}

#[handler]
pub fn create(req: &Request, Json(message): Json<NewMessage>) -> String {
  match message.validate() {
    Ok(_) => (),
    Err(_) => return "{\"error\": \"invalid_data\"}".to_string(),
  }

  let mut conn = establish_connection();

  let session = api::auth::from_request(req);

  if let Some(session) = session {
    let user_group = users_groups::table
      .filter(users_groups::user_id.eq(&session.id))
      .filter(users_groups::group_id.eq(&message.group_id))
      .select(users_groups::group_id)
      .first::<String>(&mut conn);

    if user_group.is_ok() {
      let new_message = util::Message::new(&message.group_id, &session.id, &message.content);

      let created_message = util::diesel::message::create_message(&mut conn, &new_message);

      if created_message.is_ok() {
        let logged_message =
          api::v1::event::log_message(&session.id, &message.group_id, &new_message.id);
        if logged_message.is_ok() {
          serde_json::to_string_pretty(&new_message)
            .unwrap_or("{\"error\": \"internal_server_error\"}".to_string())
        } else {
          "{\"error\": \"internal_server_error\"}".to_string()
        }
      } else {
        "{\"error\": \"internal_server_error\"}".to_string()
      }
    } else {
      "{\"error\": \"invalid_group\"}".to_string()
    }
  } else {
    "{\"error\": \"invalid_session\"}".to_string()
  }
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct EditMessage {
  #[validate(length(min = 1), length(max = 4096))]
  content: Option<String>,
  deleted: Option<bool>,
}

#[handler]
pub fn edit(
  req: &Request,
  Path(message): Path<String>,
  Json(edit_message): Json<EditMessage>,
) -> String {
  match edit_message.validate() {
    Ok(_) => (),
    Err(_) => return "{\"error\": \"invalid_data\"}".to_string(),
  }

  let mut conn = establish_connection();

  let session = api::auth::from_request(req);

  if let Some(session) = session {
    let found_message = schema::messages::table
      .filter(schema::messages::id.eq(&message))
      .filter(schema::messages::user_id.eq(&session.id))
      .select(util::Message::as_select())
      .first::<util::Message>(&mut conn);

    if let Ok(found_message) = found_message {
      let mut results = Vec::new();

      if let Some(content) = edit_message.content {
        let updated_message = util::diesel::message::set_content(&mut conn, &message, &content);

        if updated_message.is_ok() {
          results.push("update_message set_content");
        }
      }

      if let Some(deleted) = edit_message.deleted {
        let updated_message = util::diesel::message::set_deleted(&mut conn, &message, deleted);

        if updated_message.is_ok() {
          if deleted {
            results.push("update_message set_deleted:true");
          } else {
            results.push("update_message set_deleted:false");
          }
        }
      }

      if results.is_empty() {
        "{\"error\": \"no_changes\"}".to_string()
      } else {
        for result in results.iter() {
          let logged_message = api::v1::event::log_event(
            &session.id,
            &found_message.group_id,
            result,
            Some(found_message.id.clone()),
            None,
            None,
          );
          if logged_message.is_err() {
            return "{\"error\": \"internal_server_error\"}".to_string();
          }
        }
        serde_json::to_string_pretty(&results)
          .unwrap_or("{\"error\": \"internal_server_error\"}".to_string())
      }
    } else {
      "{\"error\": \"message_not_found\"}".to_string()
    }
  } else {
    "{\"error\": \"invalid_session\"}".to_string()
  }
}
