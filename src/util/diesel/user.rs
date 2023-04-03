use diesel::{PgConnection, RunQueryDsl};

use crate::{schema, util::User};

pub fn create_user(conn: &mut PgConnection, user: User) -> Result<usize, diesel::result::Error> {
  diesel::insert_into(schema::users::table)
    .values(&user)
    .execute(conn)
}
