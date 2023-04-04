pub fn unix_time() -> i64 {
  use std::time::{SystemTime, UNIX_EPOCH};
  SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_secs() as i64
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn positive() {
    let unix_time = unix_time();

    assert!(unix_time > 0, "unix_time should be positive");
  }
}
