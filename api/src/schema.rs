table! {
    account_histories (id) {
        id -> Int4,
        account_id -> Int4,
        balance -> Float8,
        updated -> Timestamp,
    }
}

table! {
    accounts (id) {
        id -> Int4,
        name -> Varchar,
        vendor -> Varchar,
        client_key -> Varchar,
        client_secret -> Varchar,
        created -> Timestamp,
        updated -> Nullable<Timestamp>,
    }
}

table! {
    users (id) {
        id -> Int4,
        first_name -> Text,
        last_name -> Text,
        email -> Text,
        created -> Timestamp,
    }
}

joinable!(account_histories -> accounts (account_id));

allow_tables_to_appear_in_same_query!(
    account_histories,
    accounts,
    users,
);
