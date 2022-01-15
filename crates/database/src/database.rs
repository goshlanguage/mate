use diesel::{prelude::*, pg::PgConnection};
use std::env;

pub fn establish_connection() -> PgConnection {
  let conn_string = env::var("DATABASE_URL")
    .expect("DATABASE_URL must be set");
   PgConnection::establish(&conn_string)
    .unwrap_or_else(|_| panic!("Error connecting to the database"))
}
