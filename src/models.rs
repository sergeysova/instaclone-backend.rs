
/// Simple user
#[derive(Serialize, Queryable, Clone)]
pub struct User {
  pub id: i32,
  pub username: String,
}

// Simple session
#[derive(Serialize, Queryable, Clone)]
pub struct Session {
  pub key: String,
  pub user_id: i32,
}
