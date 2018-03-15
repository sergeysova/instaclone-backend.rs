// use diesel::prelude::*;
// use diesel::QueryResult;
// use diesel::result::Error;

use rocket;
use rocket::http::{Cookie, Cookies};
use rocket_contrib::{Json};

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
fn get(auth: Auth) -> ApiJson<SessionResponse> {
    ApiResponse::json(SessionResponse {
        id: auth.user.id,
        username: auth.user.username,
    })
}


#[derive(Deserialize, Serialize)]
pub struct LoginBody {
    username: String,
    password: String,
}

#[post("/", format="application/json", data="<data>")]
fn create(_conn: DbConn, data: Json<LoginBody>) -> ApiJson<LoginBody> {
    ApiResponse::json(data.0)
}

#[delete("/", format="application/json")]
fn destroy(_auth: Auth) {

}

#[inline]
pub fn mount() -> Vec<rocket::Route> {
    routes![
        get,
        create,
        destroy,
    ]
}
