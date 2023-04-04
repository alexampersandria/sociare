use crate::{
  schema::{self, users::name},
  util::{self, Group, User, UserGroup},
};
use diesel::BelongingToDsl;
use diesel::ExpressionMethods;
use diesel::PgConnection;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel::SelectableHelper;

pub fn create_user(conn: &mut PgConnection, user: &User) -> Result<usize, diesel::result::Error> {
  diesel::insert_into(schema::users::table)
    .values(user)
    .execute(conn)
}

pub fn set_username(
  conn: &mut PgConnection,
  id: &String,
  new_username: &String,
) -> Result<usize, diesel::result::Error> {
  diesel::update(schema::users::table.find(id))
    .set(schema::users::username.eq(new_username))
    .execute(conn)
}

pub fn set_name(
  conn: &mut PgConnection,
  id: &String,
  new_name: &String,
) -> Result<usize, diesel::result::Error> {
  diesel::update(schema::users::table.find(id))
    .set(name.eq(new_name))
    .execute(conn)
}

pub fn set_email(
  conn: &mut PgConnection,
  id: &String,
  new_email: &String,
) -> Result<usize, diesel::result::Error> {
  diesel::update(schema::users::table.find(id))
    .set(schema::users::email.eq(new_email))
    .execute(conn)
}

pub fn set_phone(
  conn: &mut PgConnection,
  id: &String,
  new_phone: &String,
) -> Result<usize, diesel::result::Error> {
  diesel::update(schema::users::table.find(id))
    .set(schema::users::phone.eq(new_phone))
    .execute(conn)
}

pub fn set_password(
  conn: &mut PgConnection,
  id: &String,
  new_password: &String,
) -> Result<usize, diesel::result::Error> {
  diesel::update(schema::users::table.find(id))
    .set(schema::users::password.eq(new_password))
    .execute(conn)
}

pub fn delete_user(conn: &mut PgConnection, id: &String) -> Result<usize, diesel::result::Error> {
  diesel::delete(schema::users::table.find(id)).execute(conn)
}

pub fn get_user(conn: &mut PgConnection, id: &String) -> Result<User, diesel::result::Error> {
  schema::users::table.find(id).get_result(conn)
}

pub fn add_to_group(
  conn: &mut PgConnection,
  user_group: &UserGroup,
) -> Result<usize, diesel::result::Error> {
  diesel::insert_into(schema::users_groups::table)
    .values(user_group)
    .execute(conn)
}

pub fn remove_from_group(
  conn: &mut PgConnection,
  user_id: &String,
  group_id: &String,
) -> Result<usize, diesel::result::Error> {
  diesel::delete(
    schema::users_groups::table
      .filter(schema::users_groups::user_id.eq(user_id))
      .filter(schema::users_groups::group_id.eq(group_id)),
  )
  .execute(conn)
}

pub fn get_groups(
  conn: &mut PgConnection,
  user_id: &String,
) -> Result<Vec<Group>, diesel::result::Error> {
  let gotten_user = get_user(conn, user_id)?;

  util::user::UserGroup::belonging_to(&gotten_user)
    .inner_join(schema::groups::table)
    .select(util::Group::as_select())
    .load(conn)
}
