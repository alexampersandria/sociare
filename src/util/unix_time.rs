use std::time::{SystemTime, UNIX_EPOCH};

pub fn unix_ms() -> i64 {
  let duration_since_unix_epoch = SystemTime::now().duration_since(UNIX_EPOCH);
  if let Ok(duration) = duration_since_unix_epoch {
    duration.as_millis() as i64
  } else {
    0
  }
}

pub fn unix_s() -> i64 {
  unix_ms() / 1000
}

#[cfg(test)]
mod ci_unit {
  use super::*;

  #[test]
  fn unix_ms_positive() {
    let unix_time = unix_ms();

    assert!(unix_time > 0, "unix_time should be positive");
  }

  #[test]
  fn unix_s_positive() {
    let unix_time = unix_s();

    assert!(unix_time > 0, "unix_time should be positive");
  }

  #[test]
  fn unix_ms_after_2023_04_06_00_00_01() {
    // approximate date of writing this test
    let unix_time = unix_ms();

    assert!(
      unix_time > 1680732001000,
      "unix_time should be after 2023-04-06 00:00:01"
    );
  }

  #[test]
  fn unix_s_after_2023_04_06_00_00_01() {
    // approximate date of writing this test
    let unix_time = unix_ms();

    assert!(
      unix_time > 1680732001,
      "unix_time should be after 2023-04-06 00:00:01"
    );
  }

  #[test]
  fn unix_s_less_than_unix_ms() {
    let unix_time_s = unix_s();
    let unix_time_ms = unix_ms();

    assert!(
      unix_time_s < unix_time_ms,
      "unix_time_s should be less than unix_time_ms"
    );
  }
}
