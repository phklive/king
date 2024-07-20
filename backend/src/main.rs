mod agent;
mod constants;
mod contract;
mod game;
mod handlers;
mod summary;
mod types;
mod utils;

use actix_cors::Cors;
use actix_web::{App, HttpServer};

use crate::handlers::{health, play, players};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the logger
    env_logger::init();

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .service(health)
            .service(play)
            .service(players)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
