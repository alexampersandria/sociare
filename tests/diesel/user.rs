use sociare::util::*;

#[cfg(test)]
mod diesel_integration {
  use super::*;

  #[test]
  fn create_delete() {
    let mut conn = sociare::establish_connection();
    let user = sociare::util::User::new_with_phone(
      "username",
      "password",
      "Display Name 🦀",
      "test@example.com",
      "12345678",
    );
    create_user(&mut conn, &user).expect("failed creating user");
    let gotten_user = get_user(&mut conn, &user.id).expect("failed getting user");
    assert_eq!(user, gotten_user);

    delete_user(&mut conn, &user.id).expect("failed deleting user");
    let after_deletion_user = get_user(&mut conn, &user.id);
    assert!(after_deletion_user.is_err());
  }
}
