
use std::env;
use dotenv::dotenv;
use diesel::prelude::*;

pub type InstacloneConnection = SqliteConnection;

pub fn establish_connection() -> InstacloneConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL env var must be set");

    InstacloneConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
