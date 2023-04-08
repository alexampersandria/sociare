use crate::{api, schema, util};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use poem::{
  handler,
  web::{Json, Path},
  Request,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[handler]
pub fn get_all(req: &Request) -> String {
  let session = api::auth::from_request(req);

  if let Some(session) = session {
    let mut conn = crate::establish_connection();

    let groups = schema::users_groups::table
      .filter(schema::users_groups::active.eq(true))
      .filter(schema::users_groups::user_id.eq(&session.id))
      .inner_join(schema::groups::table)
      .select(util::Group::as_select())
      .get_results::<util::Group>(&mut conn);

    if let Ok(groups) = groups {
      serde_json::to_string_pretty(&groups)
        .unwrap_or("{\"error\": \"internal_server_error\"}".to_string())
    } else {
      "{\"error\": \"internal_server_error\"}".to_string()
    }
  } else {
    "{\"error\": \"invalid_session\"}".to_string()
  }
}

#[handler]
pub fn get(req: &Request, Path(group): Path<String>) -> String {
  let session = api::auth::from_request(req);

  if let Some(session) = session {
    let mut conn = crate::establish_connection();

    let user_groups = schema::users_groups::table
      .filter(schema::users_groups::active.eq(true))
      .filter(schema::users_groups::user_id.eq(&session.id))
      .filter(schema::users_groups::group_id.eq(&group))
      .select(schema::users_groups::group_id)
      .get_result::<String>(&mut conn);

    let group = schema::groups::table
      .filter(schema::groups::id.eq_any(user_groups))
      .first::<crate::util::Group>(&mut conn);

    if let Ok(group) = group {
      serde_json::to_string_pretty(&group)
        .unwrap_or("{\"error\": \"internal_server_error\"}".to_string())
    } else {
      "{\"error\": \"internal_server_error\"}".to_string()
    }
  } else {
    "{\"error\": \"invalid_session\"}".to_string()
  }
}

#[handler]
pub fn create(req: &Request, Json(group): Json<NewGroup>) -> String {
  match group.validate() {
    Ok(_) => (),
    Err(_) => return "{\"error\": \"invalid_data\"}".to_string(),
  }

  let session = api::auth::from_request(req);

  if let Some(session) = session {
    // do something
    let mut conn = crate::establish_connection();
    let group = crate::util::Group::new(&group.name, &group.currency);
    let created_group = crate::util::diesel::create_group(&mut conn, &group);
    if created_group.is_ok() {
      let user_group = crate::util::UserGroup::new(&session.id, &group.id);
      let added_user_to_group = crate::util::diesel::user::add_to_group(&mut conn, &user_group);
      if added_user_to_group.is_ok() {
        serde_json::to_string_pretty(&user_group)
          .unwrap_or("{\"error\": \"internal_server_error\"}".to_string())
      } else {
        "{\"error\": \"internal_server_error\"}".to_string()
      }
    } else {
      "{\"error\": \"internal_server_error\"}".to_string()
    }
  } else {
    "{\"error\": \"invalid_session\"}".to_string()
  }
}

#[handler]
pub fn add(req: &Request, Json(add_user_to_group): Json<AddUserToGroup>) -> String {
  match add_user_to_group.validate() {
    Ok(_) => (),
    Err(_) => return "{\"error\": \"invalid_data\"}".to_string(),
  }

  let session = api::auth::from_request(req);

  if let Some(session) = session {
    let mut conn = crate::establish_connection();

    let user_groups = schema::users_groups::table
      .filter(schema::users_groups::active.eq(true))
      .filter(schema::users_groups::user_id.eq(&session.id))
      .filter(schema::users_groups::group_id.eq(&add_user_to_group.group_id))
      .select(schema::users_groups::group_id)
      .get_result::<String>(&mut conn);

    if user_groups.is_err() {
      return "{\"error\": \"invalid_group\"}".to_string();
    }

    let user_group =
      crate::util::UserGroup::new(&add_user_to_group.user_id, &add_user_to_group.group_id);
    let added_user_to_group = crate::util::diesel::user::add_to_group(&mut conn, &user_group);
    if added_user_to_group.is_ok() {
      serde_json::to_string_pretty(&user_group)
        .unwrap_or("{\"error\": \"internal_server_error\"}".to_string())
    } else {
      "{\"error\": \"internal_server_error\"}".to_string()
    }
  } else {
    "{\"error\": \"invalid_session\"}".to_string()
  }
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct NewGroup {
  #[validate(length(min = 1), length(max = 24))]
  pub name: String,
  #[validate(length(equal = 3))]
  pub currency: String,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct AddUserToGroup {
  #[validate(length(min = 1), length(max = 96))]
  pub user_id: String,
  #[validate(length(min = 1), length(max = 96))]
  pub group_id: String,
}
