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
pub struct KeyResponse {
  key: String,
}

#[get("/", format="application/json")]
pub fn get(_conn: DbConn, auth: Auth) -> ApiJson<KeyResponse> {
  ApiResponse::json(KeyResponse {
    key: auth.user.username.clone(),
  })
}

#[post("/", format="application/json")]
pub fn create(_conn: DbConn) {

}

#[delete("/", format="application/json")]
pub fn destroy(_conn: DbConn) {

}
