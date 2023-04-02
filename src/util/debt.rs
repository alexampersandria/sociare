use uuid::Uuid;

#[derive(Debug, PartialEq, Clone)]
pub struct Debt {
  pub from: Uuid,
  pub to: Uuid,
  pub amount: i64,
}
