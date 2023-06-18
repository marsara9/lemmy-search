use std::env;
use actix_files as fs;
use actix_web::{
    App, 
    HttpServer
};

mod search;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let ui_directory = match args.get(1) {
        Some(path) => path,
        None => "./ui"
    }.to_owned();

    HttpServer::new(move || {
        App::new()
            .service(search::heartbeat)
            .service(search::search)
            .service(search::get_instances)
            .service(
                fs::Files::new("/", &ui_directory)
                    .index_file("index.html")
            )
    }).bind(("0.0.0.0", 8000))?
        .run()
        .await
}
