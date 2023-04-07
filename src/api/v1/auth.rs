use crate::establish_connection;
use crate::schema;
use crate::util;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use poem::{handler, web::Path};

use super::PrivateUserData;

#[handler]
pub fn check(Path(session): Path<String>) -> String {
  let session = validate_session(&session);
  if let Some(session) = session {
    serde_json::to_string_pretty(&session).unwrap()
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
