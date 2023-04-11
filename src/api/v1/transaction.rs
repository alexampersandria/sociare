use crate::api;
use crate::establish_connection;
use crate::schema;
use crate::util;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use poem::web::Json;
use poem::Request;
use poem::{handler, web::Path};
use serde::Deserialize;
use serde::Serialize;
use validator::Validate;

#[handler]
pub fn get(req: &Request, Path(id): Path<String>) -> String {
  let session = api::auth::from_request(req);

  if let Some(session) = session {
    let mut conn = establish_connection();

    let user_groups = schema::users_groups::table
      .filter(schema::users_groups::user_id.eq(&session.id))
      .select(schema::users_groups::group_id)
      .get_results::<String>(&mut conn);

    if let Ok(user_groups) = user_groups {
      let found_transaction = schema::transactions::table
        .filter(schema::transactions::id.eq(&id))
        .filter(schema::transactions::group_id.eq_any(user_groups))
        .select(util::Transaction::as_select())
        .first::<util::Transaction>(&mut conn);

      if let Ok(found_transaction) = found_transaction {
        serde_json::to_string_pretty(&found_transaction)
          .unwrap_or("{\"error\": \"internal_server_error\"}".to_string())
      } else {
        "{\"error\": \"transaction_not_found\"}".to_string()
      }
    } else {
      "{\"error\": \"internal_server_error\"}".to_string()
    }
  } else {
    "{\"error\": \"invalid_session\"}".to_string()
  }
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct NewTransaction {
  #[validate(length(min = 1), length(max = 96))]
  pub group_id: String,
  #[validate(length(min = 1), length(max = 96))]
  pub to_id: String,
  pub amount: i64,
  #[validate(length(min = 1), length(max = 96))]
  pub method: String,
}

#[handler]
pub fn create(req: &Request, Json(new_transaction): Json<NewTransaction>) -> String {
  match new_transaction.validate() {
    Ok(_) => (),
    Err(_) => return "{\"error\": \"invalid_data\"}".to_string(),
  }

  let session = api::auth::from_request(req);

  if let Some(session) = session {
    let mut conn = establish_connection();

    let user_groups = schema::users_groups::table
      .filter(schema::users_groups::user_id.eq(&session.id))
      .filter(schema::users_groups::group_id.eq(&new_transaction.group_id))
      .select(schema::users_groups::group_id)
      .get_results::<String>(&mut conn);

    if user_groups.is_ok() {
      let transaction = util::Transaction::new(
        &new_transaction.group_id,
        &session.id,
        &new_transaction.to_id,
        new_transaction.amount,
        &new_transaction.method,
      );

      let result = diesel::insert_into(schema::transactions::table)
        .values(&transaction)
        .get_result::<util::Transaction>(&mut conn);

      if let Ok(result) = result {
        let logged_transaction =
          api::v1::log_transaction(&session.id, &result.group_id, &transaction.id);
        if logged_transaction.is_err() {
          return "{\"error\": \"internal_server_error\"}".to_string();
        }
        serde_json::to_string_pretty(&result)
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

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct EditTransaction {
  pub amount: Option<i64>,
  #[validate(length(min = 1), length(max = 96))]
  pub method: Option<String>,
  #[validate(length(min = 1), length(max = 96))]
  pub to_id: Option<String>,
  pub deleted: Option<bool>,
}

#[handler]
pub fn edit(
  req: &Request,
  Path(transaction): Path<String>,
  Json(edit_transaction): Json<EditTransaction>,
) -> String {
  match edit_transaction.validate() {
    Ok(_) => (),
    Err(_) => return "{\"error\": \"invalid_data\"}".to_string(),
  }

  let mut conn = establish_connection();

  let session = api::auth::from_request(req);

  if let Some(session) = session {
    let found_transaction = schema::transactions::table
      .filter(schema::transactions::id.eq(&transaction))
      .filter(schema::transactions::from_id.eq(&session.id))
      .filter(schema::transactions::confirmed.eq(false))
      .select(util::Transaction::as_select())
      .first::<util::Transaction>(&mut conn);

    if let Ok(found_transaction) = found_transaction {
      let mut results = Vec::new();

      if let Some(amount) = edit_transaction.amount {
        let updated_transaction =
          util::diesel::transaction::set_amount(&mut conn, &transaction, &amount);
        if updated_transaction.is_ok() {
          results.push(format!(
            "update_transaction transaction_id:{} set_amount:{}",
            transaction, amount
          ));
        }
      }

      if let Some(info) = edit_transaction.method {
        let updated_transaction =
          util::diesel::transaction::set_method(&mut conn, &transaction, &info);
        if updated_transaction.is_ok() {
          results.push(format!(
            "update_transaction transaction_id:{} set_info",
            transaction
          ));
        }
      }

      if let Some(deleted) = edit_transaction.deleted {
        let updated_transaction =
          util::diesel::transaction::set_deleted(&mut conn, &transaction, deleted);
        if updated_transaction.is_ok() {
          if deleted {
            results.push(format!(
              "update_transaction transaction_id:{} set_deleted:true",
              transaction
            ));
          } else {
            results.push(format!(
              "update_transaction transaction_id:{} set_deleted:false",
              transaction
            ));
          }
        }
      }

      if results.is_empty() {
        "{\"error\": \"no_changes\"}".to_string()
      } else {
        for result in results.iter() {
          let logged_transaction =
            api::v1::event::log_simple(&session.id, &found_transaction.group_id, result);
          if logged_transaction.is_err() {
            return "{\"error\": \"internal_server_error\"}".to_string();
          }
        }
        serde_json::to_string_pretty(&results)
          .unwrap_or("{\"error\": \"internal_server_error\"}".to_string())
      }
    } else {
      "{\"error\": \"transaction_not_found\"}".to_string()
    }
  } else {
    "{\"error\": \"invalid_session\"}".to_string()
  }
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct ConfirmTransaction {
  pub confirmed: bool,
}

#[handler]
pub fn confirm(
  req: &Request,
  Path(transaction): Path<String>,
  Json(confirm_transaction): Json<ConfirmTransaction>,
) -> String {
  match confirm_transaction.validate() {
    Ok(_) => (),
    Err(_) => return "{\"error\": \"invalid_data\"}".to_string(),
  }

  let mut conn = establish_connection();

  let session = api::auth::from_request(req);

  if let Some(session) = session {
    let found_transaction = schema::transactions::table
      .filter(schema::transactions::id.eq(&transaction))
      .filter(schema::transactions::to_id.eq(&session.id))
      .filter(schema::transactions::deleted.eq(false))
      .select(util::Transaction::as_select())
      .first::<util::Transaction>(&mut conn);

    if let Ok(found_transaction) = found_transaction {
      let mut results = Vec::new();

      let updated_transaction = util::diesel::transaction::set_confirmed(
        &mut conn,
        &transaction,
        confirm_transaction.confirmed,
      );
      if updated_transaction.is_ok() {
        results.push(format!(
          "update_transaction transaction_id:{} set_confirmed:{}",
          transaction, &confirm_transaction.confirmed
        ));
      }

      if results.is_empty() {
        "{\"error\": \"no_changes\"}".to_string()
      } else {
        for result in results.iter() {
          let logged_transaction =
            api::v1::event::log_simple(&session.id, &found_transaction.group_id, result);
          if logged_transaction.is_err() {
            return "{\"error\": \"internal_server_error\"}".to_string();
          }
        }
        let updated_debts = api::v1::econ::update_debts(&mut conn, &found_transaction.group_id);
        if updated_debts.is_some() {
          serde_json::to_string_pretty(&results)
            .unwrap_or("{\"error\": \"internal_server_error\"}".to_string())
        } else {
          "{\"error\": \"internal_server_error\"}".to_string()
        }
      }
    } else {
      "{\"error\": \"transaction_not_found\"}".to_string()
    }
  } else {
    "{\"error\": \"invalid_session\"}".to_string()
  }
}
