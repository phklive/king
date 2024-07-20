mod agent;
mod constants;
mod contract;
mod game;
mod handlers;
mod summary;
mod types;
mod utils;

use actix_web::{App, HttpServer};

use crate::handlers::{health, play, players};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(health).service(play).service(players))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
