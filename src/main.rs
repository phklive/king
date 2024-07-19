mod agent;
mod constants;
mod contract;
mod game;
mod utils;

use game::Game;

use crate::agent::Strategy;

fn main() {
    // Define agent strategies
    // Will be provided by the frontend on API call
    let strategies = vec![
        (Strategy::Regular, 1),
        (Strategy::Whale, 1),
        // (Strategy::Degen, 3),
    ];

    // Create new Game
    let mut game = Game::new(&strategies);

    // Check current block
    println!("Current block: {}", game.current_block());

    // Advance block
    game.advance_block(1);

    // Check current block
    println!("Current block: {}", game.current_block());

    let agents = game.agents();
    let agent_0 = agents[0];
    let agent_1 = agents[1];

    println!("Created agents: {:#?} and {:#?}", agent_0, agent_1);

    // check king before pay_in
    let king = game.get_king(agent_0.address().clone()).unwrap();

    // pay_in
    let _ = game.pay_in(agent_0.address().clone()).unwrap();

    // king
    let king_0 = game.get_king(agent_0.address().clone()).unwrap();

    // change king by new payin
    let _ = game.pay_in(agent_1.address().clone()).unwrap();

    // new king
    let king_1 = game.get_king(agent_1.address().clone()).unwrap();

    println!(
        "These are the two kings that went on the throne: {}, {} and {}",
        king, king_0, king_1
    );

    println!("Game state: {:#?}", game);

    // test last_block
    let last_block = game.get_last_block(agent_0.address().clone()).unwrap();

    println!("Last block: {}", last_block);

    // test pay_out
    let pay_out = game.pay_out(agent_0.address().clone()).unwrap();
    println!("payout: {:?}", pay_out);

    // test won
    let won = game.get_won(agent_0.address().clone()).unwrap();
    println!("won: {}", won);
}
