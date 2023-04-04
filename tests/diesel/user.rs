use sociare::util::*;

#[test]
fn create() {
  let mut conn = sociare::establish_connection();
  let user = sociare::util::User::new(
    "username".to_string(),
    "password".to_string(),
    "Display Name 🦀".to_string(),
    "test@example.com".to_string(),
    "12345678".to_string(),
  );
  create_user(&mut conn, user.clone()).expect("failed creating user");
  let gotten_user = get_user(&mut conn, user.clone().id).expect("failed getting user");
  assert_eq!(user, gotten_user);
  delete_user(&mut conn, user.id).expect("failed deleting user");
}

#[test]
fn edit() {
  let mut conn = sociare::establish_connection();
  let user = sociare::util::User::new(
    "username".to_string(),
    "password".to_string(),
    "Display Name 🦀".to_string(),
    "test@example.com".to_string(),
    "12345678".to_string(),
  );
  create_user(&mut conn, user.clone()).expect("failed creating user");
  let mutated_user = sociare::util::User {
    name: "New Name 🦀".to_string(),
    ..user.clone()
  };
  edit_user(&mut conn, mutated_user).expect("failed editing user");
  let gotten_user = get_user(&mut conn, user.clone().id).expect("failed getting user");
  assert_ne!(user, gotten_user);
  delete_user(&mut conn, user.id).expect("failed deleting user");
}

#[test]
fn delete() {
  let mut conn = sociare::establish_connection();
  let user = sociare::util::User::new(
    "username".to_string(),
    "password".to_string(),
    "Display Name 🦀".to_string(),
    "test@example.com".to_string(),
    "12345678".to_string(),
  );
  create_user(&mut conn, user.clone()).expect("failed creating user");
  delete_user(&mut conn, user.clone().id).expect("failed deleting user");
  let result = get_user(&mut conn, user.id);
  assert!(result.is_err());
}
