use crate::api;
use crate::establish_connection;
use crate::schema;
use crate::util;
use bcrypt::hash;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use poem::web::Json;
use poem::Request;
use poem::{handler, web::Path};
use serde::Deserialize;
use serde::Serialize;
use validator::Validate;

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
    Ok(result) => serde_json::to_string_pretty(&PublicUserData::new(result))
      .unwrap_or("{\"error\": \"internal_server_error\"}".to_string()),
    Err(_) => "{\"error\": \"user_not_found\"}".to_string(),
  }
}

#[handler]
pub fn me(req: &Request) -> String {
  let session = api::auth::from_request(req);
  if let Some(session) = session {
    serde_json::to_string_pretty(&session)
      .unwrap_or("{\"error\": \"internal_server_error\"}".to_string())
  } else {
    "{\"error\": \"invalid_session\"}".to_string()
  }
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct NewUser {
  #[validate(length(min = 3), length(max = 24))]
  username: String,
  #[validate(length(min = 1), length(max = 96))]
  name: String,
  #[validate(length(min = 7), length(max = 96))]
  password: String,
  #[validate(email)]
  email: String,
}

#[handler]
pub fn create(Json(user): Json<NewUser>) -> String {
  match user.validate() {
    Ok(_) => (),
    Err(_) => return "{\"error\": \"invalid_data\"}".to_string(),
  }

  let mut conn = establish_connection();

  let found_user = schema::users::table
    .filter(schema::users::username.eq(&user.username))
    .or_filter(schema::users::email.eq(&user.email))
    .select(schema::users::id)
    .first::<String>(&mut conn);

  if found_user.is_ok() {
    return "{\"error\": \"email_or_username_in_use\"}".to_string();
  }

  let password_hash = hash(&user.password, 10).unwrap_or("".to_string());
  if password_hash.is_empty() {
    return "{\"error\": \"internal_server_error\"}".to_string();
  }

  let user_object = util::User::new(&user.username, &password_hash, &user.name, &user.email);

  let created_user = util::diesel::create_user(&mut conn, &user_object);

  match created_user {
    Ok(_) => {
      let private_user_object = PrivateUserData {
        id: user_object.id,
        username: user_object.username,
        name: user_object.name,
        email: user_object.email,
        created_at: user_object.created_at,
        mobilepay: "".to_string(),
        paypal_me: "".to_string(),
      };
      serde_json::to_string_pretty(&private_user_object)
        .unwrap_or("{\"error\": \"internal_server_error\"}".to_string())
    }
    Err(_) => "{\"error\": \"internal_server_error\"}".to_string(),
  }
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct EditUser {
  #[validate(length(min = 3), length(max = 24))]
  username: Option<String>,
  #[validate(length(min = 1), length(max = 96))]
  name: Option<String>,
  #[validate(length(min = 7), length(max = 96))]
  password: Option<String>,
  #[validate(email)]
  email: Option<String>,
  #[validate(length(min = 1), length(max = 24))]
  mobilepay: Option<String>,
  #[validate(length(min = 1), length(max = 96))]
  paypal_me: Option<String>,
}

#[handler]
pub fn edit(req: &Request, Json(user): Json<EditUser>) -> String {
  match user.validate() {
    Ok(_) => (),
    Err(_) => return "{\"error\": \"invalid_data\"}".to_string(),
  }

  let session = api::auth::from_request(req);

  if let Some(session) = session {
    let mut conn = establish_connection();

    let mut results = Vec::new();

    if let Some(username) = user.username {
      let changed_username = diesel::update(schema::users::table)
        .filter(schema::users::id.eq(&session.id))
        .set(schema::users::username.eq(username))
        .execute(&mut conn);
      if changed_username.is_ok() {
        results.push("username");
      }
    }

    if let Some(name) = user.name {
      let changed_name = diesel::update(schema::users::table)
        .filter(schema::users::id.eq(&session.id))
        .set(schema::users::name.eq(name))
        .execute(&mut conn);
      if changed_name.is_ok() {
        results.push("name");
      }
    }

    if let Some(password) = user.password {
      let password_hash = hash(password, 10).unwrap_or("".to_string());
      if password_hash.is_empty() {
        return "{\"error\": \"internal_server_error\"}".to_string();
      }
      let changed_password = diesel::update(schema::users::table)
        .filter(schema::users::id.eq(&session.id))
        .set(schema::users::password.eq(password_hash))
        .execute(&mut conn);
      if changed_password.is_ok() {
        results.push("password");
      }
    }

    if let Some(email) = user.email {
      let changed_email = diesel::update(schema::users::table)
        .filter(schema::users::id.eq(&session.id))
        .set(schema::users::email.eq(email))
        .execute(&mut conn);
      if changed_email.is_ok() {
        results.push("email");
      }
    }

    if let Some(mobilepay) = user.mobilepay {
      let changed_mobilepay = diesel::update(schema::users::table)
        .filter(schema::users::id.eq(&session.id))
        .set(schema::users::mobilepay.eq(mobilepay))
        .execute(&mut conn);
      if changed_mobilepay.is_ok() {
        results.push("mobilepay");
      }
    }

    if let Some(paypal_me) = user.paypal_me {
      let changed_paypal_me = diesel::update(schema::users::table)
        .filter(schema::users::id.eq(&session.id))
        .set(schema::users::paypal_me.eq(paypal_me))
        .execute(&mut conn);
      if changed_paypal_me.is_ok() {
        results.push("paypal_me");
      }
    }

    if results.is_empty() {
      "{\"error\": \"internal_server_error\"}".to_string()
    } else {
      serde_json::to_string_pretty(&results)
        .unwrap_or("{\"error\": \"internal_server_error\"}".to_string())
    }
  } else {
    "{\"error\": \"invalid_session\"}".to_string()
  }
}

#[handler]
pub fn delete(req: &Request, Json(user): Json<AuthUser>) -> String {
  match user.validate() {
    Ok(_) => (),
    Err(_) => return "{\"error\": \"invalid_data\"}".to_string(),
  }

  let session = api::auth::from_request(req);
  if let Some(session) = session {
    let mut conn = establish_connection();
    let found_user = schema::users::table
      .filter(schema::users::id.eq(&session.id))
      .select((schema::users::id, schema::users::password))
      .first::<(String, String)>(&mut conn);

    if let Ok(found_user) = found_user {
      let valid = bcrypt::verify(user.password, &found_user.1);
      if valid.is_ok() {
        let deleted = util::diesel::delete_user(&mut conn, &session.id);
        if deleted.is_ok() {
          "{\"deleted\": true}".to_string()
        } else {
          "{\"error\": \"internal_server_error\"}".to_string()
        }
      } else {
        "{\"error\": \"invalid_password\"}".to_string()
      }
    } else {
      "{\"error\": \"user_not_found\"}".to_string()
    }
  } else {
    "{\"error\": \"invalid_session\"}".to_string()
  }
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct AuthUser {
  #[validate(length(min = 3), length(max = 24))]
  username: String,
  #[validate(length(min = 7), length(max = 96))]
  password: String,
}

#[handler]
pub fn login(req: &Request, Json(user): Json<AuthUser>) -> String {
  match user.validate() {
    Ok(_) => (),
    Err(_) => return "{\"error\": \"invalid_data\"}".to_string(),
  }

  let headers = req.headers();

  let ip_address_value = headers.get("X-Forwarded-For");
  let ip_address = match ip_address_value {
    Some(ip_address) => ip_address.to_str().unwrap_or(""),
    None => "",
  };

  let user_agent_value = headers.get("User-Agent");
  let user_agent = match user_agent_value {
    Some(user_agent) => user_agent.to_str().unwrap_or(""),
    None => "",
  };

  let session = api::auth::create_session(&user.username, &user.password, ip_address, user_agent);

  match session {
    Some(session) => serde_json::to_string_pretty(&session)
      .unwrap_or("{\"error\": \"internal_server_error\"}".to_string()),
    None => "{\"error\": \"invalid_credentials\"}".to_string(),
  }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PublicUserData {
  pub id: String,
  pub username: String,
  pub created_at: i64,
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

#[derive(Deserialize, Serialize, Debug)]
pub struct PrivateUserData {
  pub id: String,
  pub username: String,
  pub name: String,
  pub email: String,
  pub mobilepay: String,
  pub paypal_me: String,
  pub created_at: i64,
}

impl PrivateUserData {
  pub fn new(
    result: (
      String,
      String,
      String,
      String,
      Option<String>,
      Option<String>,
      i64,
    ),
  ) -> Self {
    Self {
      id: result.0,
      username: result.1,
      name: result.2,
      email: result.3,
      mobilepay: result.4.unwrap_or("".to_string()),
      paypal_me: result.5.unwrap_or("".to_string()),
      created_at: result.6,
    }
  }
}
