#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate rocket_contrib;
extern crate rocket;

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;
use dotenv::dotenv;
use std::env;

mod routes;


type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

fn establish_connection() -> DbPool {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be present");

    let manager = ConnectionManager::<PgConnection>::new(db_url.as_str());

    r2d2::Pool::new(manager).expect(&format!("Can't connect to database pool {}", &db_url))
}

fn main() {
    rocket::ignite()
        .manage(establish_connection())
        .mount("/", routes![routes::get_index])
        // .catch(errors![error_404])
        .launch();
}
