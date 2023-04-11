use crate::schema;
use crate::schema::users_groups;
use crate::util;
use diesel::PgConnection;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

pub fn update_debts(conn: &mut PgConnection, group_id: &str) -> Option<(usize, usize)> {
  let user_groups = users_groups::table
    .filter(users_groups::group_id.eq(group_id))
    .filter(users_groups::active.eq(true))
    .load::<util::UserGroup>(conn);
  let receipts = schema::receipts::table
    .filter(schema::receipts::group_id.eq(group_id))
    .load::<util::Receipt>(conn);
  let transactions = schema::transactions::table
    .filter(schema::transactions::group_id.eq(group_id))
    .filter(schema::transactions::confirmed.eq(true))
    .load::<util::Transaction>(conn);

  if let (Ok(user_groups), Ok(receipts), Ok(transactions)) = (user_groups, receipts, transactions) {
    let balance = util::group::balance(&user_groups, &receipts, &transactions);
    let new_debts = util::group::debts(group_id, &balance);

    let _deleted_old_debts =
      diesel::delete(schema::debts::table.filter(schema::debts::group_id.eq(group_id)))
        .execute(conn);

    let inserted_new_debts = diesel::insert_into(schema::debts::table)
      .values(&new_debts)
      .execute(conn);

    if let Ok(inserted_new_debts) = inserted_new_debts {
      let total = util::group::total(&receipts);
      let updated_total =
        diesel::update(schema::groups::table.filter(schema::groups::id.eq(group_id)))
          .set(schema::groups::total.eq(total))
          .execute(conn);

      if let Ok(updated_total) = updated_total {
        return Some((inserted_new_debts, updated_total));
      }
    }
  }

  None
}
