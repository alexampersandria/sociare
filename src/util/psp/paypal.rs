use urlencoding::encode;

pub fn shortlink(paypal_me_handle: &str) -> String {
  format!("https://www.paypal.me/{}", encode(paypal_me_handle))
}

pub fn with_amount(paypal_me_handle: &str, amount: i64, currency: &str) -> String {
  format!(
    "{}/{}{}",
    shortlink(paypal_me_handle),
    amount as f64 / 100.0,
    currency
  )
}

#[cfg(test)]
mod ci_unit {
  use crate::util;

  #[test]
  fn generates_paypal_me_url() {
    let url = util::psp::paypal::shortlink("testuser");
    assert_eq!(url, "https://www.paypal.me/testuser")
  }

  #[test]
  fn generates_with_amount() {
    let url = util::psp::paypal::with_amount("testuser", 2534, "DKK");
    assert_eq!(url, "https://www.paypal.me/testuser/25.34DKK")
  }
}
