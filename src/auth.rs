use std::ops::Deref;
use diesel::QueryResult;
use diesel::prelude::*;
use diesel::result::Error;

use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{Request, State, FromRequest};
use rocket_contrib::Json;

use db::{DbPool, DbConn};
use models::{User, Session};
use schema::users::dsl::users;
use schema::sessions::dsl::sessions;

#[derive(Debug)]
pub enum AuthError {
  NoAuthHeader,
  InvalidHeader,
  InvalidToken,
  PoolConnectionFailure,
  NoUserOrSession,
}

/// Represents API Key from header `Authorization: Token asdqwezxc123`
pub struct ApiKey(pub String);

impl ApiKey {
  fn new<T: Into<String>>(key: T) -> ApiKey {
    ApiKey(key.into())
  }

  fn parse(header: Option<&str>) -> Result<Self, AuthError> {
    if let Some(auth_header) = header {
      let chunks: Vec<_> = auth_header.split(' ').map(str::to_string).collect();

      if chunks.len() == 2 && chunks[0] == "Token" {
        Ok(ApiKey::new(chunks[1].clone()))
      } else {
        Err(AuthError::InvalidHeader)
      }
    } else {
      Err(AuthError::NoAuthHeader)
    }
  }
}

impl Deref for ApiKey {
  type Target = String;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

pub struct Auth {
  pub user: User,
  pub session: Session,
}

impl<'a, 'r> FromRequest<'a, 'r> for Auth {
  type Error = AuthError;

  fn from_request(request: &'a Request) -> Outcome<Self, (Status, Self::Error), ()> {
    let header = request.headers().get_one("Authorization");
    let pool = request.guard::<State<DbPool>>();

    if let Outcome::Success(pool) = pool {
      if let Ok(conn) = pool.get() {
        let db = DbConn(conn);

        match ApiKey::parse(header) {
          Ok(api_key) => {
            if let Ok(Some(session)) = sessions.find(&*api_key)
              .load::<Session>(&*db)
              .map(|list| list.first().cloned()) {
              if let Ok(Some(user)) = users.find(session.user_id)
                .load::<User>(&*db)
                .map(|list| list.first().cloned()) {
                  return Outcome::Success(Auth {
                    user,
                    session,
                  })
              }
            }

            Outcome::Failure((Status::BadRequest, AuthError::NoUserOrSession))
          },
          Err(auth_error) => Outcome::Failure((Status::BadRequest, auth_error)),
        }
      } else {
        Outcome::Failure((Status::ServiceUnavailable, AuthError::PoolConnectionFailure))
      }
    } else {
      Outcome::Failure((Status::ServiceUnavailable, AuthError::PoolConnectionFailure))
    }
  }
}
