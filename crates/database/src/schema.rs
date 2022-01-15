table! {
  accounts (id) {
      id -> Int4,
      name -> Varchar,
      balance -> Nullable<Float8>,
      balance_history -> Nullable<Array<Float8>>,
  }
}
