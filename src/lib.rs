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

pub mod schema;
pub mod models;

pub mod routes;
pub mod db;



