use crate::{api, schema, util};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use poem::{
  handler,
  web::{Json, Path},
  Request,
};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

use super::PrivateUserData;

#[derive(Deserialize, Serialize, Debug)]
pub struct GroupListing {
  pub group: util::Group,
  pub events: Vec<util::GroupEvent>,
  pub users: Vec<GroupMemberUserData>,
  pub debts: Vec<util::Debt>,
}

#[handler]
pub fn get_all(req: &Request) -> String {
  let session = api::auth::from_request(req);

  let limit = req
    .params::<GetGroupParamsLimit>()
    .unwrap_or(GetGroupParamsLimit { limit: 100 });
  let offset = req
    .params::<GetGroupParamsOffset>()
    .unwrap_or(GetGroupParamsOffset { offset: 0 });

  if let Some(session) = session {
    get_group_listing(session, limit.limit, offset.offset, None)
  } else {
    "{\"error\": \"invalid_session\"}".to_string()
  }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GroupEventDetails {
  pub event: util::GroupEvent,
  pub message: Option<util::Message>,
  pub receipt: Option<util::Receipt>,
  pub transaction: Option<util::Transaction>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GroupListingDetails {
  pub group: util::Group,
  pub events: Vec<GroupEventDetails>,
}

#[handler]
pub fn get(req: &Request, Path(group): Path<String>) -> String {
  let session = api::auth::from_request(req);

  let limit = req
    .params::<GetGroupParamsLimit>()
    .unwrap_or(GetGroupParamsLimit { limit: 10 });
  let offset = req
    .params::<GetGroupParamsOffset>()
    .unwrap_or(GetGroupParamsOffset { offset: 0 });

  if let Some(session) = session {
    get_group_listing(session, limit.limit, offset.offset, Some(group))
  } else {
    "{\"error\": \"invalid_session\"}".to_string()
  }
}

#[derive(Deserialize)]
struct GetGroupParamsLimit {
  limit: i64,
}

#[derive(Deserialize)]
struct GetGroupParamsOffset {
  offset: i64,
}

pub fn get_group_listing(
  session: PrivateUserData,
  limit: i64,
  offset: i64,
  group_id: Option<String>,
) -> String {
  let mut conn = crate::establish_connection();

  let groups: Result<_, _>;

  if let Some(group_id) = group_id {
    groups = schema::users_groups::table
      .filter(schema::users_groups::active.eq(true))
      .filter(schema::users_groups::user_id.eq(&session.id))
      .filter(schema::users_groups::group_id.eq(&group_id))
      .inner_join(schema::groups::table)
      .select(util::Group::as_select())
      .get_results::<util::Group>(&mut conn);
  } else {
    groups = schema::users_groups::table
      .filter(schema::users_groups::active.eq(true))
      .filter(schema::users_groups::user_id.eq(&session.id))
      .inner_join(schema::groups::table)
      .select(util::Group::as_select())
      .get_results::<util::Group>(&mut conn);
  }

  if let Ok(groups) = groups {
    let users = schema::users_groups::table
      .filter(schema::users_groups::active.eq(true))
      .filter(schema::users_groups::group_id.eq_any(groups.iter().map(|g| g.id.clone())))
      .inner_join(schema::users::table)
      .select((
        schema::users_groups::group_id,
        schema::users::id,
        schema::users::username,
        schema::users::name,
        schema::users::mobilepay,
        schema::users::paypal_me,
        schema::users_groups::nickname,
        schema::users_groups::is_admin,
        schema::users_groups::active,
        schema::users::created_at,
      ))
      .get_results::<(
        String,
        String,
        String,
        String,
        Option<String>,
        Option<String>,
        Option<String>,
        bool,
        bool,
        i64,
      )>(&mut conn);

    let debts = schema::debts::table
      .filter(schema::debts::group_id.eq_any(groups.iter().map(|g| g.id.clone())))
      .get_results::<util::Debt>(&mut conn);

    if let (Ok(users), Ok(debts)) = (users, debts) {
      let mut group_listings: Vec<GroupListing> = Vec::new();

      for group in groups {
        let group_events = schema::group_events::table
          .filter(schema::group_events::group_id.eq(&group.id))
          .order(schema::group_events::created_at.desc())
          .limit(limit)
          .offset(offset)
          .get_results::<util::GroupEvent>(&mut conn);

        let mut group_listing = GroupListing {
          group,
          events: group_events.unwrap_or(Vec::new()),
          users: Vec::new(),
          debts: Vec::new(),
        };

        for user in &users {
          if user.0 == group_listing.group.id {
            group_listing.users.push(GroupMemberUserData {
              id: user.1.clone(),
              username: user.2.clone(),
              name: user.3.clone(),
              mobilepay: user.4.clone(),
              paypal_me: user.5.clone(),
              nickname: user.6.clone(),
              is_admin: user.7,
              active: user.8,
              created_at: user.9,
            });
          }
        }

        for debt in &debts {
          if debt.group_id == group_listing.group.id {
            group_listing.debts.push(debt.clone());
          }
        }

        group_listings.push(group_listing);
      }

      serde_json::to_string(&group_listings).unwrap()
    } else {
      "{\"error\": \"internal_server_error\"}".to_string()
    }
  } else {
    "{\"error\": \"internal_server_error\"}".to_string()
  }
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct NewGroup {
  #[validate(length(min = 1), length(max = 24))]
  pub name: String,
  #[validate(length(equal = 3))]
  pub currency: String,
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

fn validate_emoji(value: &str) -> Result<(), ValidationError> {
  let is_emoji = emojis::get(value);
  if is_emoji.is_some() {
    Ok(())
  } else {
    Err(ValidationError::new("invalid_emoji"))
  }
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct EditGroup {
  #[validate(length(min = 1), length(max = 24))]
  pub name: String,
  #[validate(length(equal = 3))]
  pub currency: String,
  #[validate(custom(function = "validate_emoji"))]
  pub emoji: String,
  #[validate(length(min = 1), length(max = 96))]
  pub theme: String,
}

#[handler]
pub fn edit(req: &Request, Json(group): Json<EditGroup>, Path(group_id): Path<String>) -> String {
  match group.validate() {
    Ok(_) => (),
    Err(_) => return "{\"error\": \"invalid_data\"}".to_string(),
  }

  let session = api::auth::from_request(req);

  if let Some(session) = session {
    let mut conn = crate::establish_connection();

    let user_group = schema::users_groups::table
      .filter(schema::users_groups::user_id.eq(&session.id))
      .filter(schema::users_groups::group_id.eq(&group_id))
      .get_result::<util::UserGroup>(&mut conn);

    if let Ok(user_group) = user_group {
      if user_group.is_admin {
        let updated_group = diesel::update(schema::groups::table.find(&group_id))
          .set((
            schema::groups::name.eq(&group.name),
            schema::groups::currency.eq(&group.currency),
            schema::groups::emoji.eq(&group.emoji),
            schema::groups::theme.eq(&group.theme),
          ))
          .get_result::<util::Group>(&mut conn);

        if let Ok(updated_group) = updated_group {
          serde_json::to_string_pretty(&updated_group)
            .unwrap_or("{\"error\": \"internal_server_error\"}".to_string())
        } else {
          "{\"error\": \"internal_server_error\"}".to_string()
        }
      } else {
        "{\"error\": \"invalid_session\"}".to_string()
      }
    } else {
      "{\"error\": \"internal_server_error\"}".to_string()
    }
  } else {
    "{\"error\": \"invalid_session\"}".to_string()
  }
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct AddUserToGroup {
  #[validate(length(min = 1), length(max = 96))]
  pub user_id: String,
  #[validate(length(min = 1), length(max = 96))]
  pub group_id: String,
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

#[derive(Deserialize, Serialize, Debug)]
pub struct GroupMemberUserData {
  pub id: String,
  pub username: String,
  pub name: String,
  pub mobilepay: Option<String>,
  pub paypal_me: Option<String>,
  pub nickname: Option<String>,
  pub is_admin: bool,
  pub active: bool,
  pub created_at: i64,
}

#[allow(clippy::complexity)]
impl GroupMemberUserData {
  pub fn new(
    result: (
      String,
      String,
      String,
      Option<String>,
      Option<String>,
      Option<String>,
      bool,
      bool,
      i64,
    ),
  ) -> Self {
    Self {
      id: result.0,
      username: result.1,
      name: result.2,
      mobilepay: result.3,
      paypal_me: result.4,
      nickname: result.5,
      is_admin: result.6,
      active: result.7,
      created_at: result.8,
    }
  }
}
