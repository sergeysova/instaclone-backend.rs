use rocket;
use diesel::QueryResult;
use diesel::prelude::*;
use diesel::result::Error;

use db::DbConn;
use models::User;
use schema::users::dsl;
use routes::{ApiJson, ApiJsonVec, ApiResponse};


/// Get users list
#[get("/", format="application/json")]
fn index(conn: DbConn) -> QueryResult<ApiJsonVec<User>> {
    dsl::users
        .load::<User>(&*conn)
        .map(ApiResponse::json_vec)
}

/// Get specific user by ID
#[get("/<user_id>", format="application/json")]
fn get(conn: DbConn, user_id: u32) -> QueryResult<ApiJson<User>> {
    dsl::users.find(user_id as i32)
        .load::<User>(&*conn)
        .and_then(|list| list.first().cloned().ok_or(Error::NotFound))
        .map(ApiResponse::json)
}

#[inline]
pub fn mount() -> Vec<rocket::Route> {
    routes![
        index,
        get,
    ]
}
