use rayon::prelude::*;
use std::fs;

use actix_web::{error::ErrorBadRequest, get, post, web, HttpResponse, Responder, Result};
use log::info;

use crate::{
    constants::PLAYERS_PATH,
    game::Game,
    summary::{FinalSummary, Summary},
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
pub async fn play(req: web::Json<(Strategies, u64)>) -> Result<impl Responder> {
    // Receive strategies from client
    let strategies: Strategies = req.0 .0;

    // Receive times from client
    let times: u64 = req.0 .1;

    // Prevent play if 0 or 1 agent
    let num_strategies: u8 = strategies.0.iter().fold(0, |acc, (_, num)| acc + num);

    if num_strategies <= 1 {
        return Err(ErrorBadRequest("Too few agents to start simulation."));
    }

    if times < 1 || times > 100 {
        return Err(ErrorBadRequest("Too many simulation runs."));
    }

    info!(
        "Game has started! {} agents are playing {} times.",
        num_strategies, times
    );

    let summaries: Vec<Summary> = (0..times)
        .into_par_iter()
        .map(|i| {
            info!("Playing iteration: {}", i);
            let mut game = Game::new(strategies.clone());
            game.play()
        })
        .collect();

    let final_summary = FinalSummary::new(summaries);
    info!(
        "Game has ended a new King has been crowned:\n {}",
        final_summary
    );
    Ok(web::Json(final_summary))
}
