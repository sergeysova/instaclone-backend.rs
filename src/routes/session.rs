// use diesel::prelude::*;
// use diesel::QueryResult;
// use diesel::result::Error;

use rocket::http::{Cookie, Cookies};

use db::DbConn;
// use models::Session;
// use schema::sessions::dsl;
use super::{ApiJson, ApiJsonVec, ApiResponse, ApiKey};

#[derive(Serialize)]
pub struct KeyResponse {
  key: String,
}

#[get("/", format="application/json")]
pub fn get(_conn: DbConn, key: ApiKey) -> ApiJson<KeyResponse> {
  ApiResponse::json(KeyResponse {
    key: key.0.clone(),
  })
}

#[post("/", format="application/json")]
pub fn create(_conn: DbConn) {

}

#[delete("/", format="application/json")]
pub fn destroy(_conn: DbConn) {

}
