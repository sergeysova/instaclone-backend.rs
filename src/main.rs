#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate rocket_contrib;
extern crate rocket;



fn main() {
    rocket::ignite()
        .mount("/", routes![index, get_user])
        // .catch(errors![error_404])
        .launch();
}
