use sociare::util::receipt::Receipt;
use sociare::util::transaction::Transaction;
use sociare::util::user::User;
use sociare::util::{group::FullGroup, random_emoji};

use rusty_money::{iso, Money};

#[allow(dead_code)]
fn full_group() -> FullGroup {
  let user_1 = User::new("user_1", "hunter2", "Foo", "user_1@example.com", "1234");
  let user_2 = User::new("user_2", "hunter2", "Bar", "user_2@example.com", "5678");
  let user_3 = User::new(
    "user_3",
    "hunter2",
    "Keith",
    "user_3@example.com",
    "12345678",
  );

  let data: Vec<Receipt> = vec![
    Receipt::new("test_group", &user_2.id, 13605, "shop_1"),
    Receipt::new("test_group", &user_2.id, 14785, "shop_2"),
    Receipt::new("test_group", &user_2.id, 10735, "shop_1"),
    Receipt::new("test_group", &user_2.id, 9715, "shop_2"),
    Receipt::new("test_group", &user_1.id, 25095, "shop_2"),
    Receipt::new("test_group", &user_1.id, 15885, "shop_2"),
    Receipt::new("test_group", &user_2.id, 8320, "shop_2"),
    Receipt::new("test_group", &user_2.id, 14495, "shop_1"),
    Receipt::new("test_group", &user_3.id, 25200, "takeaway"),
    Receipt::new("test_group", &user_1.id, 13830, "shop_2"),
    Receipt::new("test_group", &user_3.id, 13135, "shop_1"),
    Receipt::new("test_group", &user_3.id, 32330, "shop_1 + shop_2"),
    Receipt::new("test_group", &user_3.id, 16110, "shop_2"),
    Receipt::new("test_group", &user_2.id, 10580, "shop_2"),
    Receipt::new("test_group", &user_3.id, 25830, "shop_2"),
    Receipt::new("test_group", &user_1.id, 13890, "shop_2"),
    Receipt::new("test_group", &user_3.id, 14790, "shop_2"),
    Receipt::new("test_group", &user_3.id, 15780, "shop_1"),
    Receipt::new("test_group", &user_1.id, 31360, "shop_2"),
    Receipt::new("test_group", &user_2.id, 14900, "shop_2"),
    Receipt::new("test_group", &user_2.id, 16765, "shop_2"),
    Receipt::new("test_group", &user_1.id, 28500, "takeaway"),
    Receipt::new("test_group", &user_2.id, 11880, "shop_2"),
  ];

  let mut group = FullGroup::new(
    "LTT FEB 2023",
    vec![user_1, user_2, user_3],
    &random_emoji(),
    "DKK",
  );

  for receipt in data {
    group.add_receipt(receipt);
  }

  group
}

#[allow(dead_code)]
fn settle_debts(group: &mut FullGroup) {
  let settle = group.debts();
  for debt in settle {
    group.add_transaction(Transaction::new(
      "test_group",
      &debt.from_id,
      &debt.to_id,
      "test_id",
      debt.amount,
      "test",
    ));
  }
  for transaction in &mut group.transactions {
    transaction.confirmed = true;
  }
}

#[allow(dead_code)]
fn format_value(value: i64, currency: &iso::Currency) -> String {
  return Money::from_minor(value, currency).to_string();
}

#[test]
fn total_remains_the_same() {
  let mut group = full_group();
  let before_total = group.total();
  settle_debts(&mut group);
  let after_total = group.total();
  assert_eq!(before_total, after_total);
}

#[test]
fn balance_changed() {
  let mut group = full_group();
  let before_balance = group.balance();
  settle_debts(&mut group);
  let after_balance = group.balance();
  assert_ne!(before_balance, after_balance);
}

#[test]
fn total_user_1() {
  let mut group = full_group();
  let balance = group.balance();
  let user_1 = group
    .users
    .iter()
    .find(|u| u.id == group.users.get(0).unwrap().id)
    .unwrap();
  let user_1_total = balance.get(&user_1.id).unwrap();
  let currency = iso::find(&group.group.currency).unwrap();
  assert_eq!(format_value(*user_1_total, currency), "1.285,60kr.");
}

#[test]
fn total_user_2() {
  let mut group = full_group();
  let balance = group.balance();
  let user_2 = group
    .users
    .iter()
    .find(|u| u.id == group.users.get(1).unwrap().id)
    .unwrap();
  let user_2_total = balance.get(&user_2.id).unwrap();
  let currency = iso::find(&group.group.currency).unwrap();
  assert_eq!(format_value(*user_2_total, currency), "1.257,80kr.");
}

#[test]
fn total_user_3() {
  let mut group = full_group();
  let balance = group.balance();
  let user_3 = group
    .users
    .iter()
    .find(|u| u.id == group.users.get(2).unwrap().id)
    .unwrap();
  let user_3_total = balance.get(&user_3.id).unwrap();
  let currency = iso::find(&group.group.currency).unwrap();
  assert_eq!(format_value(*user_3_total, currency), "1.431,75kr.");
}

#[test]
#[ignore] // #TODO: remove ignore when fixed
fn minimal_transactions() {
  let mut group = full_group();
  let settle = group.debts();
  assert_eq!(settle.len(), 2);
}
