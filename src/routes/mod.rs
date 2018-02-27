use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{Request, FromRequest};
use rocket_contrib::Json;

pub mod users;
pub mod session;

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

#[derive(Debug)]
pub enum ApiKeyError {
  NoHeader,
  InvalidToken,
  InvalidHeader,
}

/// Represents API Key from header `Authorization: Token asdqwezxc123`
pub struct ApiKey(String);

impl ApiKey {
  fn new<T: Into<String>>(key: T) -> ApiKey {
    ApiKey(key.into())
  }
}

impl<'a, 'r> FromRequest<'a, 'r> for ApiKey {
  type Error = ApiKeyError;

  fn from_request(request: &'a Request) -> Outcome<Self, (Status, Self::Error), ()> {
    let key = request.headers().get_one("Authorization");

    if let Some(auth_header) = key {
      let chunks: Vec<_> = auth_header.split(' ').map(str::to_string).collect();

      if chunks.len() == 2 && chunks[0] == "Token" {
        Outcome::Success(ApiKey::new(chunks[1].clone()))
      } else {
        Outcome::Failure((Status::BadRequest, ApiKeyError::InvalidHeader))
      }
    } else {
      Outcome::Failure((Status::BadRequest, ApiKeyError::NoHeader))
    }
  }
}

pub mod catch {
  use rocket::Request;
  use super::{ApiJson, ApiResponse};

  #[derive(Serialize)]
  pub struct Error {
    error: String,
  }

  #[error(400)]
  pub fn handle_400(req: &Request) -> ApiJson<Error> {
    println!("req={:?}", req);
    ApiResponse::json(Error {
      error: "bad_request".into(),
    })
  }
}

