use crate::schema::{self, debts::amount};
use crate::util::Debt;
use diesel::ExpressionMethods;
use diesel::{PgConnection, QueryDsl, RunQueryDsl};

pub fn create_debt(conn: &mut PgConnection, debt: &Debt) -> Result<usize, diesel::result::Error> {
  diesel::insert_into(schema::debts::table)
    .values(debt)
    .execute(conn)
}

pub fn set_from(
  conn: &mut PgConnection,
  id: &String,
  new_from: &String,
) -> Result<usize, diesel::result::Error> {
  diesel::update(schema::debts::table.find(id))
    .set(schema::debts::from_id.eq(new_from))
    .execute(conn)
}

pub fn set_to(
  conn: &mut PgConnection,
  id: &String,
  new_to: &String,
) -> Result<usize, diesel::result::Error> {
  diesel::update(schema::debts::table.find(id))
    .set(schema::debts::to_id.eq(new_to))
    .execute(conn)
}

pub fn set_amount(
  conn: &mut PgConnection,
  id: &String,
  new_amount: &i64,
) -> Result<usize, diesel::result::Error> {
  diesel::update(schema::debts::table.find(id))
    .set(amount.eq(new_amount))
    .execute(conn)
}

pub fn delete_debt(conn: &mut PgConnection, id: &String) -> Result<usize, diesel::result::Error> {
  diesel::delete(schema::debts::table.find(id)).execute(conn)
}
