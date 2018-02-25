
use diesel::QueryResult;
use rocket_contrib::Json;
use diesel::prelude::*;

use db::DbConn;
use models::User;


#[get("/")]
pub fn get_index(conn: DbConn) -> QueryResult<Json<Vec<User>>> {
  use schema::users::dsl;

  dsl::users
    .load::<User>(&*conn)
    .map(|users| Json(users))
}
