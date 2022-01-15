table! {
    accounts (id) {
        id -> Int4,
        name -> Varchar,
        balance -> Float8,
        balance_history -> Array<Float8>,
    }
}
