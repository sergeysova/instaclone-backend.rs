extern crate dotenv;
extern crate rocket;
extern crate instaclone_backend;

use dotenv::dotenv;
use instaclone_backend::{db, create_app};

fn main() {
    dotenv().ok();

    create_app()
        .manage(db::establish_connection())
        .launch();
}
