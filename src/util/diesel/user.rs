use crate::{
  schema::{self, users::name},
  util::User,
};
use diesel::ExpressionMethods;
use diesel::{PgConnection, QueryDsl, RunQueryDsl};

pub fn create_user(conn: &mut PgConnection, user: User) -> Result<usize, diesel::result::Error> {
  diesel::insert_into(schema::users::table)
    .values(&user)
    .execute(conn)
}

pub fn edit_user(conn: &mut PgConnection, user: User) -> Result<usize, diesel::result::Error> {
  diesel::update(schema::users::table.find(user.id))
    .set(name.eq(user.name))
    .execute(conn)
}

pub fn delete_user(conn: &mut PgConnection, id: String) -> Result<usize, diesel::result::Error> {
  diesel::delete(schema::users::table.find(id)).execute(conn)
}

pub fn get_user(conn: &mut PgConnection, id: String) -> Result<User, diesel::result::Error> {
  schema::users::table.find(id).get_result(conn)
}
