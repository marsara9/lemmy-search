mod api;
mod config;
mod crawler;
mod database;

use std::env;
use actix_files as fs;
use actix_web::{
    App, 
    HttpServer
};
use crawler::Runner;
use database::Database;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let ui_directory = match args.get(1) {
        Some(path) => path,
        None => "./ui"
    }.to_owned();

    let config = config::Config::load();

    let mut database = Database::new(config.postgres);
    let _ = database.init()
        .await;

    let mut cralwer_runner = Runner::new(config.crawler);
    cralwer_runner.start();

    let result = HttpServer::new(move || {
        App::new()
            .service(api::search::heartbeat)
            .service(api::search::search)
            .service(api::search::get_instances)
            .service(
                fs::Files::new("/", &ui_directory)
                    .index_file("index.html")
            )
    }).bind(("0.0.0.0", 8000))?
        .run()
        .await;

    cralwer_runner.stop();

    result
}
