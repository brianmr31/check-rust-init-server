#![feature(decl_macro, proc_macro_hygiene)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate dotenv;
#[macro_use] extern crate rocket_contrib;

mod connection;
mod core;
mod version;

use dotenv::dotenv;

fn main() {
    dotenv().ok();
    rocket::ignite()
        .manage(connection::init_pool())
        .mount("/version",
             routes![ version::handler::index])
        .register(
            catchers![
                core::core::internal_error,
                core::core::not_found,
                core::core::req_params_error
            ])
        .launch();
}
