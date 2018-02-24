use diesel;
use diesel::QueryDsl;
use schema::users;
use schema::users::dsl::*;
use diesel::RunQueryDsl;
use diesel::result::Error;
use util::db;
use models::users::{User, NewUser};

pub fn create_user(new_user: NewUser) -> Result<User, Error> {
  let conn = db::establish_connection();

  let user = diesel::insert_into(users::table)
    .values(&new_user)
    .get_result(&conn);

  user
}

pub fn get_user(identifier: i32) -> Result<User, Error> {
  let conn = db::establish_connection();

  let user = users.find(identifier)
    .first(&conn);

  user
}
