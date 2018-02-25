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
use dotenv::dotenv;

pub mod schema;
pub mod models;

mod routes;
mod db;


fn main() {
    dotenv().ok();


    rocket::ignite()
        .manage(db::establish_connection())
        .mount("/", routes![routes::get_index])
        // .catch(errors![error_404])
        .launch();
}
