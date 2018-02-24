use context::{Ctx};
use fanta::{MiddlewareChain};

use super::user_service;
use models::users::{NewUser, User};

pub fn create_user(mut context: Ctx, _chain: &MiddlewareChain<Ctx>) -> Ctx {
  context
}

pub fn get_user(mut context: Ctx, _chain: &MiddlewareChain<Ctx>) -> Ctx {
  context
}

pub fn update_user(mut context: Ctx, _chain: &MiddlewareChain<Ctx>) -> Ctx {
  context
}

pub fn delete_user(mut context: Ctx, _chain: &MiddlewareChain<Ctx>) -> Ctx {
  context
}
