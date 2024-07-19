mod agent;
mod game;
mod utils;

use game::Game;

use crate::agent::Strategy;

const BYTECODE_PATH: &str = "contract/bytecode.txt";
const ABI_PATH: &str = "contract/abi.json";

fn main() {
    // Define agent strategies
    // Will be provided by the frontend on API call
    let strategies = vec![
        (Strategy::Regular, 2),
        (Strategy::Whale, 5),
        (Strategy::Degen, 3),
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

    // pay_in
    let _ = game.pay_in(agent_0.address().clone()).unwrap();

    // king
    let king_0 = game.king(agent_0.address().clone()).unwrap();

    // change king by new payin
    let _ = game.pay_in(agent_1.address().clone()).unwrap();

    // new king
    let king_1 = game.king(agent_1.address().clone()).unwrap();

    println!(
        "These are the two kings that went on the throne: {} and {}",
        king_0, king_1
    );
}
