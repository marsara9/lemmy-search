#[macro_use] extern crate rocket;

use rocket::fs::{FileServer, relative};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", FileServer::from(relative!("../ui")))
}
