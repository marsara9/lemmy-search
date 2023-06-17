#[macro_use] extern crate rocket;

use rocket::fs::FileServer;
use std::env;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    let args: Vec<String> = env::args().collect();
    let ui_directory = match args.first() {
        Some(path) => path,
        None => "../ui"
    };

    rocket::build()
        .mount("/", FileServer::from(ui_directory))
}
