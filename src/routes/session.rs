// use diesel::prelude::*;
// use diesel::QueryResult;
// use diesel::result::Error;

use rocket::http::{Cookie, Cookies};

use db::DbConn;
// use models::Session;
// use schema::sessions::dsl;
use routes::{ApiJson, ApiJsonVec, ApiResponse};
use auth::Auth;

#[derive(Serialize)]
pub struct SessionResponse {
  id: i32,
  username: String,
}

#[get("/", format="application/json")]
pub fn get(auth: Auth) -> ApiJson<SessionResponse> {
  ApiResponse::json(SessionResponse {
    id: auth.user.id,
    username: auth.user.username,
  })
}

#[post("/", format="application/json")]
pub fn create(_conn: DbConn) {

}

#[delete("/", format="application/json")]
pub fn destroy(_auth: Auth) {

}
