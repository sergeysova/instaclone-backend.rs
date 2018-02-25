use std::ops::Deref;
use std::env;

use r2d2;
use r2d2_diesel::ConnectionManager;
use diesel::pg::PgConnection;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

/// Create pool of connections to postgres
pub fn establish_connection() -> DbPool {
    let db_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be present");

    let manager = ConnectionManager::<PgConnection>::new(db_url.as_str());

    r2d2::Pool::new(manager).expect(&format!("Can't connect to database pool {}", &db_url))
}

pub struct DbConn(pub r2d2::PooledConnection<ConnectionManager<PgConnection>>);

impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
  type Error = ();

  fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, Self::Error> {
    let pool = request.guard::<State<DbPool>>()?;

    match pool.get() {
      Ok(conn) => Outcome::Success(DbConn(conn)),
      Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
    }
  }
}

impl Deref for DbConn {
  type Target = PgConnection;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
