use serde::Serialize;

#[derive(Queryable, Serialize)]
pub struct Account {
  pub id: i32,
  pub name: String,
  pub balance: f64,
  pub balance_history: Vec<f64>,
}

// #[derive(Insertable)]
// #[table_name="accounts"]
// pub struct NewAccount<'a> {
//     pub name: &'a str,
//     pub balance: &'a f64,
//     pub balance_history: &'a Vec<f64>,
// }

// impl<'a> NewAccount {
//   pub fn new(name: String, balance: f64) -> NewAccount<'a> {
//       NewAccount{name: &name, balance: &balance, balance_history: &vec![balance]}
//   }
// }
