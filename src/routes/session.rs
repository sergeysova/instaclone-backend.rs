use diesel::prelude::*;
use diesel::QueryResult;
use diesel::result::Error;

use rocket::http::{Cookie, Cookies};

use db::DbConn;
use models::Session;
use schema::sessions::dsl;
use super::{ApiJson, ApiJsonVec, ApiResponse, ApiKey};

#[get("/")]
pub fn get(conn: DbConn, key: ApiKey) -> String {
  key.0.clone()
}

#[post("/")]
pub fn create(conn: DbConn) {

}

#[delete("/")]
pub fn destroy(conn: DbConn) {

}
