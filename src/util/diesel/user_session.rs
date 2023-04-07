use crate::{schema, util};
use diesel::{PgConnection, RunQueryDsl};

pub fn create_user_session(
  conn: &mut PgConnection,
  user_session: &util::UserSession,
) -> Result<usize, diesel::result::Error> {
  diesel::insert_into(schema::user_sessions::table)
    .values(user_session)
    .execute(conn)
}
