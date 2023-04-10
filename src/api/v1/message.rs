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
      serde_json::to_string_pretty(&result)
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

      if let Ok(created_message) = created_message {
        let logged_message =
          api::v1::event::log_message(&session.id, &message.group_id, &new_message.id);
        if logged_message.is_ok() {
          serde_json::to_string_pretty(&created_message)
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

    if found_message.is_ok() {
      let mut results = Vec::new();

      if let Some(content) = edit_message.content {
        let updated_message = util::diesel::message::set_content(&mut conn, &message, &content);

        if updated_message.is_ok() {
          results.push("updated_message set_content");
        }
      }

      if let Some(deleted) = edit_message.deleted {
        let updated_message = util::diesel::message::set_deleted(&mut conn, &message, deleted);

        if updated_message.is_ok() {
          if deleted {
            results.push("updated_message set_deleted:true");
          } else {
            results.push("updated_message set_deleted:false");
          }
        }
      }

      if results.is_empty() {
        "{\"error\": \"no_changes\"}".to_string()
      } else {
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
