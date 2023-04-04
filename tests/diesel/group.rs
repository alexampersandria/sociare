use sociare::util::*;

#[test]
pub fn full_group() {
  let mut conn = sociare::establish_connection();

  let group = Group::new("Big Chat", "🦀", "USD");
  let other_group = Group::new("Smol Chat", "🦞", "EUR");
  let user = sociare::util::User::new(
    "momo",
    "hunter2",
    "Momo 🙈",
    "monkey@example.com",
    "555-555-5555",
  );
  let other_user = sociare::util::User::new(
    "gigi",
    "hunter2",
    "Gigi 🦒",
    "giraffe@example.com",
    "666-666-6666",
  );

  sociare::util::diesel::create_group(&mut conn, &group).expect("Failed to create group");
  sociare::util::diesel::create_group(&mut conn, &other_group).expect("Failed to create group");
  sociare::util::diesel::create_user(&mut conn, &user).expect("Failed to create user");
  sociare::util::diesel::add_to_group(&mut conn, &UserGroup::new(&user.id, &group.id))
    .expect("Failed to add user to group");
  sociare::util::diesel::add_to_group(&mut conn, &UserGroup::new(&user.id, &other_group.id))
    .expect("Failed to add user to group");
  sociare::util::diesel::create_user(&mut conn, &other_user).expect("Failed to create user");
  sociare::util::diesel::add_to_group(&mut conn, &UserGroup::new(&other_user.id, &group.id))
    .expect("Failed to add user to group");

  let message = sociare::util::Message::new(&group.id, &user.id, "test_message");
  let other_message = sociare::util::Message::new(&group.id, &other_user.id, "other_test_message");
  let third_message = sociare::util::Message::new(&group.id, &user.id, "🐓");
  sociare::util::diesel::create_message(&mut conn, &message).expect("Failed to create message");
  sociare::util::diesel::create_message(&mut conn, &other_message)
    .expect("Failed to create message");
  sociare::util::diesel::create_message(&mut conn, &third_message)
    .expect("Failed to create message");

  let user_groups = get_groups(&mut conn, &user.id);
  let group_users = get_users(&mut conn, &group.id);

  assert_eq!(user_groups.unwrap().len(), 2);
  assert_eq!(group_users.unwrap().len(), 2);
}
