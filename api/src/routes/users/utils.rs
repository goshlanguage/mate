use crate::diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use crate::establish_connection;
use crate::models::*;
use actix_web::{Error, HttpResponse, Responder};
use log::error;

// CREATE ID
/// Returns user associated with the given id
pub fn create_user(new: &NewUserPayload) -> User {
    use crate::schema::users;

    let connection = establish_connection();
    let now = chrono::Utc::now().naive_utc();

    let new_user = NewUser {
        first_name: &new.first_name,
        last_name: &new.last_name,
        email: &new.email,
        created: now,
    };

    // TODO
    // catch and return error here perhaps
    diesel::insert_into(users::table)
        .values(new_user)
        .get_result::<User>(&connection)
        .expect("Error creating new user")
}

// READ ID
/// Returns user associated with the given id
pub fn get_user_by_id(user_id: i32) -> Option<User> {
    use crate::schema::users::dsl::*;

    let connection = establish_connection();

    let result = users.filter(id.eq(user_id)).first(&connection);

    match result {
        Ok(user) => Some(user),
        Err(_) => None,
    }
}

// DELETE 1
/// Delete users
pub fn delete(user_id: i32) -> String {
    use crate::schema::users::dsl::*;

    let connection = establish_connection();

    // TODO
    // gross hack alert
    diesel::delete(users.filter(id.eq(user_id)))
        .execute(&connection)
        .unwrap();
    format!("ok")
}
