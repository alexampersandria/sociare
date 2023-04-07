use sociare::util::*;

#[cfg(test)]
mod diesel_integration {
  use super::*;

  #[test]
  pub fn create_delete() {
    let mut conn = sociare::establish_connection();

    let group = Group::new("Crab Chat", "🦀", "USD");
    let user = sociare::util::User::new_with_mobilepay(
      "momo",
      "hunter2",
      "Momo 🙈",
      "monkey@example.com",
      "555-555-5555",
    );
    let message = Message::new(&group.id, &user.id, "Hello World!");

    sociare::util::diesel::create_group(&mut conn, &group).expect("Failed to create group");
    sociare::util::diesel::create_user(&mut conn, &user).expect("Failed to create user");
    sociare::util::diesel::add_to_group(&mut conn, &UserGroup::new(&user.id, &group.id))
      .expect("Failed to add user to group");
    sociare::util::diesel::create_message(&mut conn, &message).expect("Failed to create message");

    let user_groups = get_groups(&mut conn, &user.id);
    let group_users = get_users(&mut conn, &group.id);
    let group_messages = get_messages(&mut conn, &group.id);

    assert_eq!(user_groups.unwrap().len(), 1);
    assert_eq!(group_users.unwrap().len(), 1);
    assert_eq!(group_messages.unwrap().len(), 1);

    sociare::util::diesel::delete_group(&mut conn, &group.id).expect("Failed to delete group");
    sociare::util::diesel::delete_user(&mut conn, &user.id).expect("Failed to delete user");
    sociare::util::diesel::delete_message(&mut conn, &message.id)
      .expect("Failed to delete message");

    let after_deletion_user_groups = get_groups(&mut conn, &user.id);
    let after_deletion_group_users = get_users(&mut conn, &group.id);
    let after_deletion_group_messages = get_messages(&mut conn, &group.id);

    assert!(after_deletion_user_groups.is_err());
    assert!(after_deletion_group_users.is_err());
    assert_eq!(after_deletion_group_messages.unwrap().len(), 0);
  }
}
