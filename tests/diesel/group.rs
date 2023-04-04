use sociare::util::*;

#[test]
pub fn full_group() {
  let mut conn = sociare::establish_connection();

  let group = Group::new("Big Chat".to_string(), "🦀".to_string(), "USD".to_string());
  let other_group = Group::new("Smol Chat".to_string(), "🦞".to_string(), "EUR".to_string());
  let user = sociare::util::User::new(
    "momo".to_string(),
    "hunter2".to_string(),
    "Momo 🙈".to_string(),
    "monkey@example.com".to_string(),
    "555-555-5555".to_string(),
  );
  let other_user = sociare::util::User::new(
    "gigi".to_string(),
    "hunter2".to_string(),
    "Gigi 🦒".to_string(),
    "giraffe@example.com".to_string(),
    "666-666-6666".to_string(),
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

  let message = sociare::util::Message::new(
    group.id.clone(),
    user.id.clone(),
    "test_message".to_string(),
  );
  let other_message = sociare::util::Message::new(
    group.id.clone(),
    other_user.id.clone(),
    "other_test_message".to_string(),
  );
  let third_message =
    sociare::util::Message::new(group.id.clone(), user.id.clone(), "🐓".to_string());
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
