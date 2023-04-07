#![allow(dead_code)]
// not actually benchmarking anything 🤷‍♀️

use rand::{seq::SliceRandom, Rng};
use sociare::util;

fn benchmark() {
  let start = util::unix_ms();
  println!("starting benchmark... @ {}", start);
  let mut group = util::FullGroup::new("Test Group", vec![], "🎉", "USD");

  for i in 0..100 {
    let user =
      util::User::new_with_mobilepay(&format!("user_{}", i), "hunter", "test", "test", "test");
    group.add_user(user);
  }

  for _i in 0..10000 {
    let receipt = util::Receipt::new(
      &group.group.id,
      &group.users.choose(&mut rand::thread_rng()).unwrap().id,
      rand::thread_rng().gen_range(1..1000000),
      "",
    );
    group.add_receipt(receipt);
  }

  group.update_debts();
  let debts = group.debts;

  println!(
    "added 100 users, 10000 receipts and calculated {} debts in {}ms",
    debts.len(),
    util::unix_ms() - start
  );
}

// # some examples:
// before optimizations:
// starting benchmark... @ 1680740173425
// added 100 users, 10000 receipts and calculated 99 debts in 26028ms
// after optimizations:
// starting benchmark... @ 1680740312882
// added 100 users, 10000 receipts and calculated 99 debts in 2189 ms
