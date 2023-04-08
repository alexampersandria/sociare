use crate::establish_connection;
use crate::schema;
use crate::util;
use crate::util::UserSession;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use poem::Request;
use poem::{handler, web::Path};

use super::PrivateUserData;

#[handler]
pub fn check(Path(session): Path<String>) -> String {
  let session = validate_session(&session);
  if let Some(session) = session {
    serde_json::to_string_pretty(&session)
      .unwrap_or("{\"error\": \"internal_server_error\"}".to_string())
  } else {
    "{\"error\": \"invalid_session\"}".to_string()
  }
}

#[handler]
pub fn delete(Path(session): Path<String>) -> String {
  let deleted = delete_session(&session);
  if deleted {
    "{\"deleted\": true}".to_string()
  } else {
    "{\"error\": \"invalid_session\"}".to_string()
  }
}

#[handler]
pub fn list(req: &Request) -> String {
  let session = from_request(req);
  if let Some(session) = session {
    let mut conn = establish_connection();
    let all_sessions = schema::user_sessions::table
      .filter(schema::user_sessions::user_id.eq(&session.id))
      .load::<UserSession>(&mut conn);
    if let Ok(all_sessions) = all_sessions {
      serde_json::to_string_pretty(&all_sessions)
        .unwrap_or("{\"error\": \"internal_server_error\"}".to_string())
    } else {
      "{\"error\": \"no_active_sessions\"}".to_string()
    }
  } else {
    "{\"error\": \"invalid_session\"}".to_string()
  }
}

pub fn create_session(
  username: &str,
  password: &str,
  ip_address: &str,
  user_agent: &str,
) -> Option<util::UserSession> {
  let mut conn = establish_connection();
  let user = schema::users::table
    .filter(schema::users::username.eq(&username))
    .select((schema::users::id, schema::users::password))
    .first::<(String, String)>(&mut conn);

  if let Ok(user) = user {
    let valid = bcrypt::verify(password, &user.1);
    if valid.is_ok() {
      let session = util::UserSession::new(&user.0, ip_address, user_agent);
      let created_session = util::diesel::create_user_session(&mut conn, &session);
      if created_session.is_ok() {
        Some(session)
      } else {
        None
      }
    } else {
      None
    }
  } else {
    None
  }
}

pub fn delete_session(session: &str) -> bool {
  let mut conn = establish_connection();
  let deleted_session = util::diesel::delete_user_session(&mut conn, session);
  deleted_session.is_ok()
}

pub fn validate_session(session: &str) -> Option<PrivateUserData> {
  let mut conn = establish_connection();
  let user_id = schema::user_sessions::table
    .find(&session)
    .select(schema::user_sessions::user_id)
    .first::<String>(&mut conn);

  if let Ok(user_id) = user_id {
    let user = schema::users::table
      .find(&user_id)
      .select((
        schema::users::id,
        schema::users::username,
        schema::users::name,
        schema::users::email,
        schema::users::mobilepay,
        schema::users::paypal_me,
        schema::users::created_at,
      ))
      .first::<(
        String,
        String,
        String,
        String,
        Option<String>,
        Option<String>,
        i64,
      )>(&mut conn);

    if let Ok(user) = user {
      Some(PrivateUserData::new(user))
    } else {
      None
    }
  } else {
    None
  }
}

pub fn from_request(req: &Request) -> Option<PrivateUserData> {
  let session = req.headers().get("Authorization");
  if let Some(session) = session {
    let session = session.to_str().unwrap_or("");
    if !session.starts_with("Bearer ") {
      return None;
    }
    let session = session.replace("Bearer ", "");
    validate_session(&session)
  } else {
    None
  }
}
