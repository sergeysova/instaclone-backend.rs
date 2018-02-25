
use rocket_contrib::Json;
use diesel::QueryResult;
use diesel::prelude::*;
use diesel::result::Error;

use db::DbConn;
use models::User;
use schema::users::dsl;

pub type ApiJson<T> = Json<ApiResponse<T>>;
pub type ApiJsonVec<T> = Json<ApiResponse<Vec<T>>>;

/// Error objects provide additional information about problems encountered while performing an operation
#[derive(Serialize)]
pub struct ApiError {
  /// a unique identifier for this particular occurrence of the problem.
  pub id: String,

  /// the HTTP status code applicable to this problem, expressed as a string value.
  pub status: Option<u32>,

  /// an application-specific error code, expressed as a string value.
  pub code: Option<u32>,

  /// a short, human-readable summary of the problem that SHOULD NOT change from occurrence
  /// to occurrence of the problem, except for purposes of localization.
  pub title: Option<String>,

  /// a human-readable explanation specific to this occurrence of the problem.
  /// Like title, this field’s value can be localized.
  pub detail: Option<String>,
}

/// This object defines a document’s “top level”
#[derive(Serialize)]
pub struct ApiResponse<T> {
  pub data: Option<T>,
  pub errors: Option<Vec<ApiError>>,
}

impl<T> ApiResponse<T> {
  /// Create API response with data
  fn data(data: T) -> ApiResponse<T> {
    ApiResponse { data: Some(data), errors: None }
  }

  /// Create API response with JSON
  fn json(data: T) -> ApiJson<T> {
    Json(Self::data(data))
  }

  /// Create API response with list of json
  fn json_vec(data: Vec<T>) -> ApiJsonVec<T> {
    Json(ApiResponse { data: Some(data), errors: None })
  }
}


/// Get users list
#[get("/users")]
pub fn get_users(conn: DbConn) -> QueryResult<ApiJsonVec<User>> {
  dsl::users
    .load::<User>(&*conn)
    .map(ApiResponse::json_vec)
}

/// Get specific user by ID
#[get("/users/<user_id>")]
pub fn get_user(conn: DbConn, user_id: u32) -> QueryResult<ApiJson<User>> {
  dsl::users.find(user_id as i32)
    .load::<User>(&*conn)
    .and_then(|list| list.first().cloned().ok_or(Error::NotFound))
    .map(ApiResponse::json)
}
