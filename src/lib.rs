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
extern crate r2d2;
extern crate r2d2_diesel;

pub mod db;

mod schema;
mod models;
mod auth;
mod routes;

pub fn create_app() -> rocket::Rocket {
    rocket::ignite()
        .mount("/users", routes::users::mount())
        .mount("/session", routes::session::mount())
        .catch(routes::catch::errors())
}