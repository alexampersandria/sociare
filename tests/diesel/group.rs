use sociare::util::*;

#[cfg(test)]
mod diesel_integration {
  use super::*;

  #[test]
  pub fn create() {
    let mut conn = sociare::establish_connection();

    let group = Group::new_with_emoji("Crab Chat", "🦀", "USD");
    let user = sociare::util::User::new_with_mobilepay(
      "momo",
      "hunter2",
      "Momo 🙈",
      "monkey@example.com",
      "555-555-5555",
    );

    sociare::util::diesel::create_group(&mut conn, &group).expect("Failed to create group");
    sociare::util::diesel::create_user(&mut conn, &user).expect("Failed to create user");
    sociare::util::diesel::add_to_group(&mut conn, &UserGroup::new(&user.id, &group.id))
      .expect("Failed to add user to group");

    let user_groups = get_groups(&mut conn, &user.id);
    let group_users = get_users(&mut conn, &group.id);

    assert_eq!(user_groups.unwrap().len(), 1);
    assert_eq!(group_users.unwrap().len(), 1);

    sociare::util::diesel::delete_group(&mut conn, &group.id).expect("Failed to delete group");
    sociare::util::delete_user(&mut conn, &user.id).expect("Failed to delete user");
  }
}
