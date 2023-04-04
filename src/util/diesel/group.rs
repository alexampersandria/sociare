use crate::schema::{self};
use crate::util::{self, Group, User};
use diesel::BelongingToDsl;
use diesel::ExpressionMethods;
use diesel::PgConnection;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel::SelectableHelper;

pub fn create_group(
  conn: &mut PgConnection,
  group: &Group,
) -> Result<usize, diesel::result::Error> {
  diesel::insert_into(schema::groups::table)
    .values(group)
    .execute(conn)
}

pub fn set_name(
  conn: &mut PgConnection,
  id: &String,
  new_name: &String,
) -> Result<usize, diesel::result::Error> {
  diesel::update(schema::groups::table.find(id))
    .set(schema::groups::name.eq(new_name))
    .execute(conn)
}

pub fn set_emoji(
  conn: &mut PgConnection,
  id: &String,
  new_emoji: &String,
) -> Result<usize, diesel::result::Error> {
  diesel::update(schema::groups::table.find(id))
    .set(schema::groups::emoji.eq(new_emoji))
    .execute(conn)
}

pub fn set_currency(
  conn: &mut PgConnection,
  id: &String,
  new_currency: &String,
) -> Result<usize, diesel::result::Error> {
  diesel::update(schema::groups::table.find(id))
    .set(schema::groups::currency.eq(new_currency))
    .execute(conn)
}

pub fn get_group(conn: &mut PgConnection, id: &String) -> Result<Group, diesel::result::Error> {
  schema::groups::table.find(id).first(conn)
}

pub fn get_users(conn: &mut PgConnection, id: &String) -> Result<Vec<User>, diesel::result::Error> {
  let gotten_group = get_group(conn, id)?;

  util::user::UserGroup::belonging_to(&gotten_group)
    .inner_join(schema::users::table)
    .select(util::User::as_select())
    .load(conn)
}

pub fn get_messages(
  conn: &mut PgConnection,
  id: &String,
) -> Result<Vec<util::Message>, diesel::result::Error> {
  schema::messages::table
    .inner_join(schema::groups::table)
    .filter(schema::groups::id.eq(id))
    .select(util::Message::as_select())
    .load(conn)
}

pub fn get_receipts(
  conn: &mut PgConnection,
  id: &String,
) -> Result<Vec<util::Receipt>, diesel::result::Error> {
  schema::receipts::table
    .inner_join(schema::groups::table)
    .filter(schema::groups::id.eq(id))
    .select(util::Receipt::as_select())
    .load(conn)
}

pub fn get_transactions(
  conn: &mut PgConnection,
  id: &String,
) -> Result<Vec<util::Transaction>, diesel::result::Error> {
  schema::transactions::table
    .inner_join(schema::groups::table)
    .filter(schema::groups::id.eq(id))
    .select(util::Transaction::as_select())
    .load(conn)
}

pub fn get_debts(
  conn: &mut PgConnection,
  id: &String,
) -> Result<Vec<util::Debt>, diesel::result::Error> {
  schema::debts::table
    .inner_join(schema::groups::table)
    .filter(schema::groups::id.eq(id))
    .select(util::Debt::as_select())
    .load(conn)
}

pub fn get_full_group(
  conn: &mut PgConnection,
  id: &String,
) -> Result<util::FullGroup, diesel::result::Error> {
  let group = get_group(conn, id)?;
  let users = get_users(conn, id)?;
  let messages = get_messages(conn, id)?;
  let receipts = get_receipts(conn, id)?;
  let transactions = get_transactions(conn, id)?;
  let debts = get_debts(conn, id)?;

  Ok(util::FullGroup {
    group,
    users,
    messages,
    receipts,
    transactions,
    debts,
  })
}
