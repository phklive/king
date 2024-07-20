use actix_web::{get, post, web, HttpResponse, Responder, Result};

use crate::{game::Game, types::Strategies};

#[get("/health")]
pub async fn health() -> impl Responder {
    HttpResponse::Ok().body("The king game is up and running!")
}

#[post("/play")]
pub async fn play(strategies: web::Json<Strategies>) -> Result<impl Responder> {
    // Receive strategies from client
    let strategies = strategies.0;

    // Create new Game
    let mut game = Game::new(strategies);

    // Play the game
    let summary = game.play();

    Ok(web::Json(summary))
}
