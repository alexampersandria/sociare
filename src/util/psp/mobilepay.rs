use urlencoding::encode;

pub fn betalingslink(mobilepay: &str, amount: i64, comment: &str) -> String {
  format!(
    "https://www.mobilepay.dk/erhverv/betalingslink/betalingslink-svar?phone={}&amount={:.2}&comment={}",
    encode(mobilepay),
    amount as f64 / 100.0,
    encode(comment)
  )
}

#[cfg(test)]
mod ci_unit {
  use crate::util::betalingslink;

  #[test]
  fn generates_betalingslink() {
    let url = betalingslink("12345678", 12345, "🤝 sociare");
    assert_eq!(url, "https://www.mobilepay.dk/erhverv/betalingslink/betalingslink-svar?phone=12345678&amount=123.45&comment=%F0%9F%A4%9D%20sociare")
  }
}
