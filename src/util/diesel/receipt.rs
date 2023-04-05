use crate::{schema, util};
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};

pub fn create_receipt(
  conn: &mut PgConnection,
  receipt: &util::Receipt,
) -> Result<usize, diesel::result::Error> {
  diesel::insert_into(schema::receipts::table)
    .values(receipt)
    .execute(conn)
}

pub fn set_user_id(
  conn: &mut PgConnection,
  id: &String,
  new_user_id: &String,
) -> Result<usize, diesel::result::Error> {
  diesel::update(schema::receipts::table.find(id))
    .set(schema::receipts::user_id.eq(new_user_id))
    .execute(conn)
}

pub fn set_amount(
  conn: &mut PgConnection,
  id: &String,
  new_amount: &i64,
) -> Result<usize, diesel::result::Error> {
  diesel::update(schema::receipts::table.find(id))
    .set(schema::receipts::amount.eq(new_amount))
    .execute(conn)
}

pub fn set_info(
  conn: &mut PgConnection,
  id: &String,
  new_info: &String,
) -> Result<usize, diesel::result::Error> {
  diesel::update(schema::receipts::table.find(id))
    .set(schema::receipts::info.eq(new_info))
    .execute(conn)
}

pub fn set_deleted(
  conn: &mut PgConnection,
  id: &String,
  new_deleted: &bool,
) -> Result<usize, diesel::result::Error> {
  diesel::update(schema::receipts::table.find(id))
    .set(schema::receipts::deleted.eq(new_deleted))
    .execute(conn)
}

pub fn get_receipt(
  conn: &mut PgConnection,
  id: &String,
) -> Result<util::Receipt, diesel::result::Error> {
  schema::receipts::table
    .find(id)
    .first::<util::Receipt>(conn)
}

/**
# ⚠️ WARNING ⚠️
Use only for tests. In production, use `set_deleted` instead.
*/
pub fn delete_receipt(
  conn: &mut PgConnection,
  id: &String,
) -> Result<usize, diesel::result::Error> {
  diesel::delete(schema::receipts::table.find(id)).execute(conn)
}
