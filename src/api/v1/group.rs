use crate::{api, schema, util};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use poem::{
  handler,
  web::{Json, Path},
  Request,
};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

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

#[derive(Deserialize, Serialize, Debug)]
pub struct GroupListing {
  pub group: util::Group,
  pub events: Vec<util::GroupEvent>,
  pub users: Vec<GroupMemberUserData>,
  pub debts: Vec<util::Debt>,
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
  pub users: Vec<GroupMemberUserData>,
  pub debts: Vec<util::Debt>,
}

pub fn get_group_listing(
  session: api::v1::user::PrivateUserData,
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
  #[validate(length(min = 1), length(max = 48))]
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
  #[validate(length(min = 1), length(max = 48))]
  pub name: Option<String>,
  #[validate(length(equal = 3))]
  pub currency: Option<String>,
  #[validate(custom(function = "validate_emoji"))]
  pub emoji: Option<String>,
  #[validate(length(min = 1), length(max = 96))]
  pub theme: Option<String>,
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
        let mut results = Vec::new();

        if let Some(name) = group.name {
          let changed_name = diesel::update(schema::groups::table)
            .filter(schema::groups::id.eq(&group_id))
            .set(schema::groups::name.eq(&name))
            .execute(&mut conn);

          if changed_name.is_ok() {
            results.push("name");
          }
        }

        if let Some(currency) = group.currency {
          let changed_currency = diesel::update(schema::groups::table)
            .filter(schema::groups::id.eq(&group_id))
            .set(schema::groups::currency.eq(&currency))
            .execute(&mut conn);

          if changed_currency.is_ok() {
            results.push("currency");
          }
        }

        if let Some(emoji) = group.emoji {
          let changed_emoji = diesel::update(schema::groups::table)
            .filter(schema::groups::id.eq(&group_id))
            .set(schema::groups::emoji.eq(&emoji))
            .execute(&mut conn);

          if changed_emoji.is_ok() {
            results.push("emoji");
          }
        }

        if let Some(theme) = group.theme {
          let changed_theme = diesel::update(schema::groups::table)
            .filter(schema::groups::id.eq(&group_id))
            .set(schema::groups::theme.eq(&theme))
            .execute(&mut conn);

          if changed_theme.is_ok() {
            results.push("theme");
          }
        }

        if results.is_empty() {
          "{\"error\": \"no_changes\"}".to_string()
        } else {
          serde_json::to_string_pretty(&results)
            .unwrap_or("{\"error\": \"internal_server_error\"}".to_string())
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
pub struct UserIdParams {
  #[validate(length(min = 1), length(max = 96))]
  pub user_id: String,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UserGroupParams {
  #[validate(length(min = 1), length(max = 96))]
  pub user_id: String,
  #[validate(length(min = 1), length(max = 96))]
  pub group_id: String,
}

#[handler]
pub fn add(
  req: &Request,
  Json(user_id_params): Json<UserIdParams>,
  Path(group_id): Path<String>,
) -> String {
  let user_group_params = UserGroupParams {
    user_id: user_id_params.user_id,
    group_id,
  };
  match user_group_params.validate() {
    Ok(_) => (),
    Err(_) => return "{\"error\": \"invalid_data\"}".to_string(),
  }

  let session = api::auth::from_request(req);

  if let Some(session) = session {
    let mut conn = crate::establish_connection();

    let user_groups = schema::users_groups::table
      .filter(schema::users_groups::active.eq(true))
      .filter(schema::users_groups::user_id.eq(&session.id))
      .filter(schema::users_groups::group_id.eq(&user_group_params.group_id))
      .select(util::UserGroup::as_select())
      .get_result::<util::UserGroup>(&mut conn);

    if user_groups.is_err() {
      return "{\"error\": \"invalid_group\"}".to_string();
    }

    if let Ok(user_groups) = user_groups {
      if !user_groups.is_admin {
        return "{\"error\": \"not_authorized\"}".to_string();
      }
    }

    let user_already_in_group = schema::users_groups::table
      .filter(schema::users_groups::user_id.eq(&user_group_params.user_id))
      .filter(schema::users_groups::group_id.eq(&user_group_params.group_id))
      .select(schema::users_groups::active)
      .get_result::<bool>(&mut conn);

    if let Ok(user_already_in_group) = user_already_in_group {
      if user_already_in_group {
        return "{\"error\": \"user_already_in_group\"}".to_string();
      } else {
        let set_user_group_active = diesel::update(
          schema::users_groups::table
            .filter(schema::users_groups::user_id.eq(&user_group_params.user_id))
            .filter(schema::users_groups::group_id.eq(&user_group_params.group_id)),
        )
        .set(schema::users_groups::active.eq(true))
        .get_result::<crate::util::UserGroup>(&mut conn);
        if let Ok(set_user_group_inactive) = set_user_group_active {
          let logged_event = api::event::log_simple(
            &session.id,
            &user_group_params.group_id,
            &format!("activated_user user_id:{}", &user_group_params.user_id),
          );

          if logged_event.is_err() {
            return "{\"error\": \"internal_server_error\"}".to_string();
          }

          return serde_json::to_string_pretty(&set_user_group_inactive)
            .unwrap_or("{\"error\": \"internal_server_error\"}".to_string());
        } else {
          return "{\"error\": \"internal_server_error\"}".to_string();
        }
      }
    }

    let user_group =
      crate::util::UserGroup::new(&user_group_params.user_id, &user_group_params.group_id);
    let added_user_to_group = crate::util::diesel::user::add_to_group(&mut conn, &user_group);
    if added_user_to_group.is_ok() {
      let logged_event = api::event::log_simple(
        &session.id,
        &user_group_params.group_id,
        &format!("added_user user_id:{}", &user_group_params.user_id),
      );

      if logged_event.is_err() {
        return "{\"error\": \"internal_server_error\"}".to_string();
      }

      serde_json::to_string_pretty(&user_group)
        .unwrap_or("{\"error\": \"internal_server_error\"}".to_string())
    } else {
      "{\"error\": \"internal_server_error\"}".to_string()
    }
  } else {
    "{\"error\": \"invalid_session\"}".to_string()
  }
}

#[handler]
pub fn remove(
  req: &Request,
  Json(user_id_params): Json<UserIdParams>,
  Path(group_id): Path<String>,
) -> String {
  let user_group_params = UserGroupParams {
    user_id: user_id_params.user_id,
    group_id,
  };
  match user_group_params.validate() {
    Ok(_) => (),
    Err(_) => return "{\"error\": \"invalid_data\"}".to_string(),
  }

  let session = api::auth::from_request(req);

  if let Some(session) = session {
    let mut conn = crate::establish_connection();

    let user_groups = schema::users_groups::table
      .filter(schema::users_groups::active.eq(true))
      .filter(schema::users_groups::user_id.eq(&session.id))
      .filter(schema::users_groups::group_id.eq(&user_group_params.group_id))
      .select(util::UserGroup::as_select())
      .get_result::<util::UserGroup>(&mut conn);

    if user_groups.is_err() {
      return "{\"error\": \"invalid_group\"}".to_string();
    }

    let user_in_group = schema::users_groups::table
      .filter(schema::users_groups::active.eq(true))
      .filter(schema::users_groups::user_id.eq(&user_group_params.user_id))
      .filter(schema::users_groups::group_id.eq(&user_group_params.group_id))
      .select(schema::users_groups::group_id)
      .get_result::<String>(&mut conn);

    if user_in_group.is_err() {
      return "{\"error\": \"user_not_in_group\"}".to_string();
    }

    let mut is_admin = false;
    if let Ok(user_groups) = user_groups {
      is_admin = user_groups.is_admin;
    }

    if session.id != user_group_params.user_id && !is_admin {
      return "{\"error\": \"not_authorized\"}".to_string();
    }

    let set_user_group_inactive = diesel::update(
      schema::users_groups::table
        .filter(schema::users_groups::user_id.eq(&user_group_params.user_id))
        .filter(schema::users_groups::group_id.eq(&user_group_params.group_id)),
    )
    .set(schema::users_groups::active.eq(false))
    .get_result::<crate::util::UserGroup>(&mut conn);

    if set_user_group_inactive.is_ok() {
      let logged_event = api::event::log_simple(
        &session.id,
        &user_group_params.group_id,
        &format!("deactivated_user user_id:{}", &user_group_params.user_id),
      );

      if logged_event.is_err() {
        return "{\"error\": \"internal_server_error\"}".to_string();
      }

      serde_json::to_string_pretty(&user_group_params)
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
