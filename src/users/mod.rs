mod user_controller;
mod user_service;

use fanta::{App};
use context::{generate_context, Ctx};
use self::user_controller::{create_user, get_user, update_user, delete_user};

pub fn init() -> App<Ctx> {
  let mut subroute = App::<Ctx>::create(generate_context);

  subroute.post("/", vec![create_user]);
  subroute.get("/:id", vec![get_user]);
  subroute.put("/:id", vec![update_user]);
  subroute.delete("/:id", vec![delete_user]);

  subroute
}
