use sociare::util::mobilepay::betalingslink::betalingslink;

#[cfg(test)]
mod ci_integration {
  use super::*;

  #[test]
  fn generates_betalingslink() {
    let url = betalingslink("12345678".to_string(), 12345, "🤝 sociare".to_string());
    assert_eq!(url, "https://www.mobilepay.dk/erhverv/betalingslink/betalingslink-svar?phone=12345678&amount=123.45&comment=%F0%9F%A4%9D%20sociare")
  }
}
