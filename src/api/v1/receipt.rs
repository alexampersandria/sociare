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
      let found_receipt = schema::receipts::table
        .filter(schema::receipts::id.eq(&id))
        .filter(schema::receipts::group_id.eq_any(user_groups))
        .select(util::Receipt::as_select())
        .first::<util::Receipt>(&mut conn);

      if let Ok(found_receipt) = found_receipt {
        serde_json::to_string_pretty(&found_receipt)
          .unwrap_or("{\"error\": \"internal_server_error\"}".to_string())
      } else {
        "{\"error\": \"receipt_not_found\"}".to_string()
      }
    } else {
      "{\"error\": \"internal_server_error\"}".to_string()
    }
  } else {
    "{\"error\": \"invalid_session\"}".to_string()
  }
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct NewReceipt {
  #[validate(length(min = 1), length(max = 96))]
  pub group_id: String,
  pub amount: i64,
  #[validate(length(min = 1), length(max = 768))]
  pub info: String,
}

#[handler]
pub fn create(req: &Request, Json(new_receipt): Json<NewReceipt>) -> String {
  match new_receipt.validate() {
    Ok(_) => (),
    Err(_) => return "{\"error\": \"invalid_data\"}".to_string(),
  }

  let session = api::auth::from_request(req);

  if let Some(session) = session {
    let mut conn = establish_connection();

    let user_groups = schema::users_groups::table
      .filter(schema::users_groups::user_id.eq(&session.id))
      .filter(schema::users_groups::group_id.eq(&new_receipt.group_id))
      .select(schema::users_groups::group_id)
      .get_results::<String>(&mut conn);

    if user_groups.is_ok() {
      let receipt = util::Receipt::new(
        &new_receipt.group_id,
        &session.id,
        new_receipt.amount,
        &new_receipt.info,
      );

      let result = diesel::insert_into(schema::receipts::table)
        .values(&receipt)
        .get_result::<util::Receipt>(&mut conn);

      if let Ok(result) = result {
        let updated_debts = api::v1::econ::update_debts(&mut conn, &result.group_id);

        if updated_debts.is_some() {
          let logged_receipt = api::v1::log_receipt(&session.id, &result.group_id, &receipt.id);
          if logged_receipt.is_err() {
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
      "{\"error\": \"internal_server_error\"}".to_string()
    }
  } else {
    "{\"error\": \"invalid_session\"}".to_string()
  }
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct EditReceipt {
  pub amount: Option<i64>,
  #[validate(length(min = 1), length(max = 768))]
  pub info: Option<String>,
  pub deleted: Option<bool>,
}

#[handler]
pub fn edit(
  req: &Request,
  Path(receipt): Path<String>,
  Json(edit_receipt): Json<EditReceipt>,
) -> String {
  match edit_receipt.validate() {
    Ok(_) => (),
    Err(_) => return "{\"error\": \"invalid_data\"}".to_string(),
  }

  let mut conn = establish_connection();

  let session = api::auth::from_request(req);

  if let Some(session) = session {
    let found_receipt = schema::receipts::table
      .filter(schema::receipts::id.eq(&receipt))
      .filter(schema::receipts::user_id.eq(&session.id))
      .select(util::Receipt::as_select())
      .first::<util::Receipt>(&mut conn);

    if let Ok(found_receipt) = found_receipt {
      let mut results = Vec::new();

      if let Some(amount) = edit_receipt.amount {
        let updated_receipt = util::diesel::receipt::set_amount(&mut conn, &receipt, &amount);
        if updated_receipt.is_ok() {
          results.push(format!(
            "update_receipt receipt_id:{} set_amount:{}",
            receipt, amount
          ));
        }
      }

      if let Some(info) = edit_receipt.info {
        let updated_receipt = util::diesel::receipt::set_info(&mut conn, &receipt, &info);
        if updated_receipt.is_ok() {
          results.push(format!("update_receipt receipt_id:{} set_info", receipt));
        }
      }

      if let Some(deleted) = edit_receipt.deleted {
        let updated_receipt = util::diesel::receipt::set_deleted(&mut conn, &receipt, &deleted);
        if updated_receipt.is_ok() {
          if deleted {
            results.push(format!(
              "update_receipt receipt_id:{} set_deleted:true",
              receipt
            ));
          } else {
            results.push(format!(
              "update_receipt receipt_id:{} set_deleted:false",
              receipt
            ));
          }
        }
      }

      if results.is_empty() {
        "{\"error\": \"no_changes\"}".to_string()
      } else {
        for result in results.iter() {
          let logged_receipt =
            api::v1::event::log_simple(&session.id, &found_receipt.group_id, result);
          if logged_receipt.is_err() {
            return "{\"error\": \"internal_server_error\"}".to_string();
          }
        }
        let updated_debts = api::v1::econ::update_debts(&mut conn, &found_receipt.group_id);
        if updated_debts.is_some() {
          serde_json::to_string_pretty(&results)
            .unwrap_or("{\"error\": \"internal_server_error\"}".to_string())
        } else {
          "{\"error\": \"internal_server_error\"}".to_string()
        }
      }
    } else {
      "{\"error\": \"receipt_not_found\"}".to_string()
    }
  } else {
    "{\"error\": \"invalid_session\"}".to_string()
  }
}
