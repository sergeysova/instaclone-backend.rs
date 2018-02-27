#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate dotenv;
extern crate rocket;
extern crate instaclone_backend;

use dotenv::dotenv;
use instaclone_backend::{db, routes};

fn main() {
    dotenv().ok();

    rocket::ignite()
        .manage(db::establish_connection())
        .mount("/users", routes![
            routes::users::index,
            routes::users::get,
        ])
        .mount("/session", routes![
            routes::session::create,
            routes::session::get,
            routes::session::destroy,
        ])
        // .catch(errors![error_404])
        .launch();
}
