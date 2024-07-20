mod agent;
mod constants;
mod contract;
mod game;
mod summary;
mod utils;

use crate::{agent::Strategy, game::Game};

fn main() {
    // Define agent strategies, will be provided by the frontend on API call
    let strategies = vec![
        (Strategy::Regular, 1),
        (Strategy::Whale, 1),
        (Strategy::Degen, 1),
    ];

    // Create new Game
    let mut game = Game::new(strategies);

    // Play the game
    let summary = game.play();

    println!("Game summary: {:?}", summary);
}
