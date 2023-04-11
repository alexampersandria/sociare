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

    let debt = sociare::util::Debt::new(&group.id, &from.id, &to.id, 100);
    create_debt(&mut conn, &debt).expect("failed creating debt");

    let gotten_debt = get_debt(&mut conn, &debt.id).expect("failed getting debt");

    assert_eq!(debt, gotten_debt);

    delete_user(&mut conn, &from.id).expect("failed deleting user");
    delete_user(&mut conn, &to.id).expect("failed deleting user");
    delete_group(&mut conn, &group.id).expect("failed deleting group");
    delete_debt(&mut conn, &debt.id).expect("failed deleting debt");
  }
}
