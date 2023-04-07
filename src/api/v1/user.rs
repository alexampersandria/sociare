use crate::establish_connection;
use crate::schema;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use poem::{handler, web::Path};
use serde::Deserialize;
use serde::Serialize;

#[handler]
pub fn get(Path(id): Path<String>) -> String {
  let mut conn = establish_connection();
  let result = schema::users::table
    .filter(schema::users::id.eq(&id))
    .or_filter(schema::users::username.eq(&id))
    .select((
      schema::users::id,
      schema::users::username,
      schema::users::created_at,
    ))
    .first::<(String, String, i64)>(&mut conn);
  match result {
    Ok(result) => serde_json::to_string_pretty(&PublicUserData::new(result)).unwrap(),
    Err(_) => "{\"error\": \"user_not_found\"}".to_string(),
  }
}

#[derive(Deserialize, Serialize)]
struct PublicUserData {
  id: String,
  username: String,
  created_at: i64,
}

impl PublicUserData {
  pub fn new(result: (String, String, i64)) -> Self {
    Self {
      id: result.0,
      username: result.1,
      created_at: result.2,
    }
  }
}
