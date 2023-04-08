use sociare::util::*;

#[cfg(test)]
mod diesel_integration {
  use super::*;

  #[test]
  fn create() {
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

    let group = sociare::util::Group::new_with_emoji("groupname", "Group Name 🦀", "USD");
    create_group(&mut conn, &group).expect("failed creating group");

    let receipt = sociare::util::Receipt::new(&group.id, &from.id, 100, "test");
    create_receipt(&mut conn, &receipt).expect("failed creating receipt");

    let gotten_receipt = get_receipt(&mut conn, &receipt.id).expect("failed getting receipt");

    assert_eq!(receipt, gotten_receipt);

    delete_user(&mut conn, &from.id).expect("failed deleting user");
    delete_user(&mut conn, &to.id).expect("failed deleting user");
    delete_group(&mut conn, &group.id).expect("failed deleting group");
    delete_receipt(&mut conn, &receipt.id).expect("failed deleting receipt");
  }
}
