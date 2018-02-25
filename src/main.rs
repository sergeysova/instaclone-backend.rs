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

use dotenv::dotenv;

pub mod schema;
pub mod models;

pub mod routes;
pub mod db;


fn main() {
    dotenv().ok();

    rocket::ignite()
        .manage(db::establish_connection())
        .mount("/", routes![routes::get_users, routes::get_user])
        // .catch(errors![error_404])
        .launch();
}
