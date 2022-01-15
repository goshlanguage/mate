#[derive(Queryable)]
pub struct Account {
  pub id: i32,
  pub name: String,
  pub balance: f64,
  pub balance_history: Vec<f64>,
}