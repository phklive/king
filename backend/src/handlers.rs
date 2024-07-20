use std::fs;

use actix_web::{get, post, web, HttpResponse, Responder, Result};
use log::info;

use crate::{
    constants::PLAYERS_PATH,
    game::Game,
    types::{Players, Strategies},
};

#[get("/health")]
pub async fn health() -> impl Responder {
    HttpResponse::Ok().body("The king game is up and running!")
}

#[get("/players")]
pub async fn players() -> Result<impl Responder> {
    let players_string = fs::read_to_string(PLAYERS_PATH).unwrap();
    let json_data: Players = serde_json::from_str(&players_string).unwrap();

    Ok(web::Json(json_data))
}

#[post("/play")]
pub async fn play(strategies: web::Json<Strategies>) -> Result<impl Responder> {
    // Receive strategies from client
    let strategies = strategies.0;

    // Create new Game
    let mut game = Game::new(strategies);

    // Play the game
    let summary = game.play();

    info!("Game summary: {}", summary);

    Ok(web::Json(summary))
}
