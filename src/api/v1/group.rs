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

#[derive(Deserialize)]
struct GetGroupParams {
  limit: i64,
  offset: i64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GroupDetails {
  pub group: util::Group,
  pub users: Vec<UserDetails>,
  pub messages: Vec<util::Message>,
  pub receipts: Vec<util::Receipt>,
  pub transactions: Vec<util::Transaction>,
  pub debts: Vec<util::Debt>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserDetails {
  pub id: String,
  pub username: String,
  pub name: String,
  pub mobilepay: Option<String>,
  pub paypal_me: Option<String>,
  pub created_at: i64,
  pub nickname: Option<String>,
}

#[handler]
pub fn get(req: &Request, Path(group): Path<String>) -> String {
  let session = api::auth::from_request(req);
  let params = req.params::<GetGroupParams>();

  let mut limit = 100;
  let mut offset = 0;

  if let Ok(params) = params {
    limit = params.limit;
    offset = params.offset;
  }

  if let Some(session) = session {
    let mut conn = crate::establish_connection();

    let user_groups = schema::users_groups::table
      .filter(schema::users_groups::active.eq(true))
      .filter(schema::users_groups::user_id.eq(&session.id))
      .filter(schema::users_groups::group_id.eq(&group))
      .select(schema::users_groups::group_id)
      .get_result::<String>(&mut conn);

    let found_group = schema::groups::table
      .filter(schema::groups::id.eq_any(user_groups))
      .first::<crate::util::Group>(&mut conn);

    if let Ok(found_group) = found_group {
      let users: Result<
        Vec<(
          util::UserGroup,
          String,
          String,
          String,
          Option<String>,
          Option<String>,
          i64,
        )>,
        diesel::result::Error,
      > = schema::users_groups::table
        .filter(schema::users_groups::active.eq(true))
        .filter(schema::users_groups::group_id.eq(&group))
        .inner_join(schema::users::table)
        .select((
          util::UserGroup::as_select(),
          schema::users::id,
          schema::users::username,
          schema::users::name,
          schema::users::mobilepay,
          schema::users::paypal_me,
          schema::users::created_at,
        ))
        .get_results::<(
          util::UserGroup,
          String,
          String,
          String,
          Option<String>,
          Option<String>,
          i64,
        )>(&mut conn);

      if let Ok(users) = users {
        let user_details = users
          .iter()
          .map(|user| UserDetails {
            id: user.1.clone(),
            username: user.2.clone(),
            name: user.3.clone(),
            mobilepay: user.4.clone(),
            paypal_me: user.5.clone(),
            created_at: user.6,
            nickname: user.0.nickname.clone(),
          })
          .collect::<Vec<UserDetails>>();

        let messages = schema::messages::table
          .filter(schema::messages::group_id.eq(&group))
          .limit(limit)
          .offset(offset)
          .order(schema::messages::created_at.desc())
          .get_results::<crate::util::Message>(&mut conn);

        if let Ok(messages) = messages {
          let receipts = schema::receipts::table
            .filter(schema::receipts::group_id.eq(&group))
            .limit(limit)
            .offset(offset)
            .order(schema::receipts::created_at.desc())
            .get_results::<crate::util::Receipt>(&mut conn);

          if let Ok(receipts) = receipts {
            let transactions = schema::transactions::table
              .filter(schema::transactions::group_id.eq(&group))
              .limit(limit)
              .offset(offset)
              .order(schema::transactions::created_at.desc())
              .get_results::<crate::util::Transaction>(&mut conn);

            if let Ok(transactions) = transactions {
              let debts = schema::debts::table
                .filter(schema::debts::group_id.eq(&group))
                .limit(limit)
                .offset(offset)
                .order(schema::debts::created_at.desc())
                .get_results::<crate::util::Debt>(&mut conn);

              if let Ok(debts) = debts {
                let mut group_details = GroupDetails {
                  group: found_group,
                  users: user_details,
                  messages,
                  receipts,
                  transactions,
                  debts,
                };

                serde_json::to_string_pretty(&group_details)
                  .unwrap_or("{\"error\": \"internal_server_error\"}".to_string())
              } else {
                "{\"error\": \"internal_server_error\"}".to_string()
              }
            } else {
              "{\"error\": \"internal_server_error\"}".to_string()
            }
          } else {
            "{\"error\": \"internal_server_error\"}".to_string()
          }
        } else {
          "{\"error\": \"internal_server_error\"}".to_string()
        }
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
      let user_group = crate::util::UserGroup::new_admin(&session.id, &group.id);
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

    let user_already_in_group = schema::users_groups::table
      .filter(schema::users_groups::active.eq(true))
      .filter(schema::users_groups::user_id.eq(&add_user_to_group.user_id))
      .filter(schema::users_groups::group_id.eq(&add_user_to_group.group_id))
      .select(schema::users_groups::group_id)
      .get_result::<String>(&mut conn);

    if user_already_in_group.is_ok() {
      return "{\"error\": \"user_already_in_group\"}".to_string();
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
