use sociare::util::group::Group;
use sociare::util::receipt::Receipt;
use sociare::util::transaction::Transaction;
use sociare::util::user::User;

use rusty_money::{iso, Money};

#[allow(dead_code)]
fn group() -> Group {
  let user_1 = User::new("user_1".to_string(), "user_1@example.com".to_string());
  let user_2 = User::new("user_2".to_string(), "user_2@example.com".to_string());
  let user_3 = User::new("user_3".to_string(), "user_3@example.com".to_string());

  let data: Vec<Receipt> = vec![
    Receipt::new(user_2.id, 13605, "shop_1".to_string()),
    Receipt::new(user_2.id, 14785, "shop_2".to_string()),
    Receipt::new(user_2.id, 10735, "shop_1".to_string()),
    Receipt::new(user_2.id, 9715, "shop_2".to_string()),
    Receipt::new(user_1.id, 25095, "shop_2".to_string()),
    Receipt::new(user_1.id, 15885, "shop_2".to_string()),
    Receipt::new(user_2.id, 8320, "shop_2".to_string()),
    Receipt::new(user_2.id, 14495, "shop_1".to_string()),
    Receipt::new(user_3.id, 25200, "takeaway".to_string()),
    Receipt::new(user_1.id, 13830, "shop_2".to_string()),
    Receipt::new(user_3.id, 13135, "shop_1".to_string()),
    Receipt::new(user_3.id, 32330, "shop_1 + shop_2".to_string()),
    Receipt::new(user_3.id, 16110, "shop_2".to_string()),
    Receipt::new(user_2.id, 10580, "shop_2".to_string()),
    Receipt::new(user_3.id, 25830, "shop_2".to_string()),
    Receipt::new(user_1.id, 13890, "shop_2".to_string()),
    Receipt::new(user_3.id, 14790, "shop_2".to_string()),
    Receipt::new(user_3.id, 15780, "shop_1".to_string()),
    Receipt::new(user_1.id, 31360, "shop_2".to_string()),
    Receipt::new(user_2.id, 14900, "shop_2".to_string()),
    Receipt::new(user_2.id, 16765, "shop_2".to_string()),
    Receipt::new(user_1.id, 28500, "takeaway".to_string()),
    Receipt::new(user_2.id, 11880, "shop_2".to_string()),
  ];

  let mut group = Group::new(
    "LTT FEB 2023".to_string(),
    &vec![user_1, user_2, user_3],
    iso::DKK.to_owned(),
  );

  for receipt in data {
    group.add_receipt(receipt);
  }

  group
}

#[allow(dead_code)]
fn settle_debts(group: &mut Group) {
  let settle = group.debts();
  for debt in settle {
    group.add_transaction(Transaction::new(
      debt.from,
      debt.to,
      debt.amount.unsigned_abs(),
      "Settlement".to_string(),
    ));
  }
  for transaction in &mut group.transactions {
    transaction.confirm();
  }
}

#[allow(dead_code)]
fn format_value(value: i64, currency: &iso::Currency) -> String {
  return Money::from_minor(value, currency).to_string();
}

#[test]
fn total_remains_the_same() {
  let mut group = group();
  let before_total = group.total();
  settle_debts(&mut group);
  let after_total = group.total();
  assert_eq!(before_total, after_total);
}

#[test]
fn balance_changed() {
  let mut group = group();
  let before_balance = group.balance();
  settle_debts(&mut group);
  let after_balance = group.balance();
  assert_ne!(before_balance, after_balance);
}

#[test]
fn total_user_1() {
  let mut group = group();
  let balance = group.balance();
  let user_1 = group
    .users
    .iter()
    .find(|u| u.display_name == "user_1")
    .unwrap();
  let user_1_total = balance.get(&user_1.id).unwrap();
  assert_eq!(format_value(*user_1_total, &group.currency), "1.285,60kr.");
}

#[test]
fn total_user_2() {
  let mut group = group();
  let balance = group.balance();
  let user_2 = group
    .users
    .iter()
    .find(|u| u.display_name == "user_2")
    .unwrap();
  let user_2_total = balance.get(&user_2.id).unwrap();
  assert_eq!(format_value(*user_2_total, &group.currency), "1.257,80kr.");
}

#[test]
fn total_user_3() {
  let mut group = group();
  let balance = group.balance();
  let thorifnn = group
    .users
    .iter()
    .find(|u| u.display_name == "user_3")
    .unwrap();
  let thorifnn_total = balance.get(&thorifnn.id).unwrap();
  assert_eq!(
    format_value(*thorifnn_total, &group.currency),
    "1.431,75kr."
  );
}

#[test]
#[ignore] // not yet implemented
fn minimal_transactions() {
  let mut group = group();
  let settle = group.debts();
  assert_eq!(settle.len(), 2);
}
