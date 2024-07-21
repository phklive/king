use std::fs;

use actix_web::{error::ErrorBadRequest, get, post, web, HttpResponse, Responder, Result};
use log::info;

use crate::{
    constants::PLAYERS_PATH,
    game::Game,
    summary::FinalSummary,
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

    // Prevent play if 0 or 1 agent
    let num_strategies: u8 = strategies.0.iter().fold(0, |acc, (_, num)| acc + num);

    if num_strategies <= 1 {
        return Err(ErrorBadRequest("Too few agents to start simulation."));
    }

    let times = 100;

    let mut summaries = Vec::new();

    info!("Game has started! {} agents are playing.", num_strategies);

    for i in 0..times {
        // Create new Game
        let mut game = Game::new(strategies.clone());

        // Play the game
        info!("Playing iteration: {}", i);
        let summary = game.play();

        summaries.push(summary);
    }

    let final_summary = FinalSummary::new(summaries);

    info!(
        "Game has ended a new King has been crowned:\n {}",
        final_summary
    );

    Ok(web::Json(final_summary))
}
