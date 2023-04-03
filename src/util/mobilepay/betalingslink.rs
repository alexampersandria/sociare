use urlencoding::encode;

pub fn betalingslink(phone: String, amount: i64, comment: String) -> String {
  format!(
    "https://www.mobilepay.dk/erhverv/betalingslink/betalingslink-svar?phone={}&amount={:.2}&comment={}",
    encode(&phone),
    amount as f64 / 100.0,
    encode(&comment)
  )
}
