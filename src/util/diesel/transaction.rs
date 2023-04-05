use crate::schema;
use crate::util;
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};

pub fn create_transaction(
  conn: &mut PgConnection,
  transaction: &util::Transaction,
) -> Result<usize, diesel::result::Error> {
  diesel::insert_into(schema::transactions::table)
    .values(transaction)
    .execute(conn)
}

pub fn set_from(
  conn: &mut PgConnection,
  id: &String,
  new_from: &String,
) -> Result<usize, diesel::result::Error> {
  diesel::update(schema::transactions::table.find(id))
    .set(schema::transactions::from_id.eq(new_from))
    .execute(conn)
}

pub fn set_to(
  conn: &mut PgConnection,
  id: &String,
  new_to: &String,
) -> Result<usize, diesel::result::Error> {
  diesel::update(schema::transactions::table.find(id))
    .set(schema::transactions::to_id.eq(new_to))
    .execute(conn)
}

pub fn set_amount(
  conn: &mut PgConnection,
  id: &String,
  new_amount: &i64,
) -> Result<usize, diesel::result::Error> {
  diesel::update(schema::transactions::table.find(id))
    .set(schema::transactions::amount.eq(new_amount))
    .execute(conn)
}

pub fn set_method(
  conn: &mut PgConnection,
  id: &String,
  new_method: &String,
) -> Result<usize, diesel::result::Error> {
  diesel::update(schema::transactions::table.find(id))
    .set(schema::transactions::method.eq(new_method))
    .execute(conn)
}

pub fn set_confirmed(
  conn: &mut PgConnection,
  id: &String,
  new_confirmed: bool,
) -> Result<usize, diesel::result::Error> {
  diesel::update(schema::transactions::table.find(id))
    .set(schema::transactions::confirmed.eq(new_confirmed))
    .execute(conn)
}

pub fn set_deleted(
  conn: &mut PgConnection,
  id: &String,
  deleted: bool,
) -> Result<usize, diesel::result::Error> {
  diesel::update(schema::transactions::table.find(id))
    .set(schema::transactions::deleted.eq(deleted))
    .execute(conn)
}

pub fn get_transaction(
  conn: &mut PgConnection,
  id: &String,
) -> Result<util::Transaction, diesel::result::Error> {
  schema::transactions::table
    .find(id)
    .first::<util::Transaction>(conn)
}

/**
# ⚠️ WARNING ⚠️
Use only for tests. In production, use `set_deleted` instead.
*/
pub fn delete_transaction(
  conn: &mut PgConnection,
  id: &String,
) -> Result<usize, diesel::result::Error> {
  diesel::delete(schema::transactions::table.find(id)).execute(conn)
}
