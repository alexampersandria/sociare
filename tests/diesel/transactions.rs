use sociare::util::*;

#[cfg(test)]
mod diesel_integration {
  use super::*;

  #[test]
  fn create_delete() {
    let mut conn = sociare::establish_connection();
    let from = sociare::util::User::new_with_mobilepay(
      "username",
      "password",
      "Display Name 🦀",
      "test@example.com",
      "12345678",
    );
    let to = sociare::util::User::new_with_mobilepay(
      "other user",
      "password",
      "Lobster Man 🦞",
      "test2@example.com",
      "004512345678",
    );
    create_user(&mut conn, &from).expect("failed creating user");
    create_user(&mut conn, &to).expect("failed creating user");

    let group = sociare::util::Group::new("groupname", "Group Name 🦀", "USD");
    create_group(&mut conn, &group).expect("failed creating group");

    let transaction = sociare::util::Transaction::new(&group.id, &from.id, &to.id, 100, "test");
    create_transaction(&mut conn, &transaction).expect("failed creating transaction");

    let gotten_transaction =
      get_transaction(&mut conn, &transaction.id).expect("failed getting transaction");

    assert_eq!(transaction, gotten_transaction);

    delete_user(&mut conn, &from.id).expect("failed deleting user");
    delete_user(&mut conn, &to.id).expect("failed deleting user");
    delete_group(&mut conn, &group.id).expect("failed deleting group");
    delete_transaction(&mut conn, &transaction.id).expect("failed deleting transaction");

    let after_deletion_from = get_user(&mut conn, &from.id);
    let after_deletion_to = get_user(&mut conn, &to.id);
    let after_deletion_group = get_group(&mut conn, &group.id);
    let after_deletion_transaction = get_transaction(&mut conn, &transaction.id);

    assert!(after_deletion_from.is_err());
    assert!(after_deletion_to.is_err());
    assert!(after_deletion_group.is_err());
    assert!(after_deletion_transaction.is_err());
  }
}
