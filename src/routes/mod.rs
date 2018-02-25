
use diesel::QueryResult;
use rocket_contrib::Json;
use diesel::prelude::*;
use diesel::result::Error;

use db::DbConn;
use models::User;
use schema::users::dsl;


#[get("/users")]
pub fn get_users(conn: DbConn) -> QueryResult<Json<Vec<User>>> {
  dsl::users
    .load::<User>(&*conn)
    .map(|users| Json(users))
}

#[get("/users/<user_id>")]
pub fn get_user(conn: DbConn, user_id: u32) -> QueryResult<Json<User>> {
  // match dsl::users.find(user_id as i32)
  //   .load::<User>(&*conn) {
  //     Ok(user) => match user.get(0) {
  //       Some(mut user) => Ok(Json(user)),
  //       None => Err(Error::NotFound),
  //     },
  //     Err(_) => Err(Error::NotFound),
  //   }
  dsl::users.find(user_id as i32)
    .load::<User>(&*conn)
    // .map(|user| user.get(0).ok_or(Error::NotFound, |user| user))
    .and_then(|list| list.get(0).ok_or_else(|| Error::NotFound))
    .map(|user| Json(*user))
}
