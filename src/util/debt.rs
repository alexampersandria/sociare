#[derive(Debug, PartialEq, Clone)]
pub struct Debt {
  pub from: String,
  pub to: String,
  pub amount: i64,
}
