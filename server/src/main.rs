#[macro_use] extern crate rocket;

use rocket::fs::{FileServer, relative};
use std::env;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    let args: Vec<String> = env::args().collect();
    let ui_directory = match args.get(1) {
        Some(path) => path,
        None => relative!("/ui")
    };

    println!("UI directory set to: '{}'", ui_directory.to_string());

    rocket::build()
        .mount("/", FileServer::from(ui_directory))
}
