use crate::schema;
use crate::util;
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};

pub fn create_message(
  conn: &mut PgConnection,
  message: &util::Message,
) -> Result<usize, diesel::result::Error> {
  diesel::insert_into(schema::messages::table)
    .values(message)
    .execute(conn)
}

pub fn set_content(
  conn: &mut PgConnection,
  id: &String,
  new_content: &String,
) -> Result<usize, diesel::result::Error> {
  diesel::update(schema::messages::table.find(id))
    .set(schema::messages::content.eq(new_content))
    .execute(conn)
}

pub fn set_deleted(
  conn: &mut PgConnection,
  id: &String,
  deleted: bool,
) -> Result<usize, diesel::result::Error> {
  diesel::update(schema::messages::table.find(id))
    .set(schema::messages::deleted.eq(deleted))
    .execute(conn)
}

/**
# ⚠️ WARNING ⚠️
Use only for tests. In production, use `set_deleted` instead.
*/
pub fn delete_message(
  conn: &mut PgConnection,
  id: &String,
) -> Result<usize, diesel::result::Error> {
  diesel::delete(schema::messages::table.find(id)).execute(conn)
}
