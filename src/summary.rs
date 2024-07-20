use serde::{Deserialize, Serialize};

use crate::{agent::Agent, game::Game, utils::wei_to_eth_u64};

#[derive(Debug, Deserialize, Serialize)]
pub struct Summary {
    king: Agent,
    times_played: u64,
    block: u64,
    balance: u64,
}

impl Summary {
    pub fn new(game: &mut Game) -> Self {
        // Get account of winner
        let king_address = game.get_king().unwrap();

        // Find matching Agent
        let king = game
            .agents()
            .iter()
            .find(|agent| *agent.address() == king_address)
            .unwrap()
            .to_owned();

        // Get last block
        let block: u64 = game.get_current_block().try_into().unwrap();
        println!("block: {:?}", block);

        // Get balance of winner
        let balance = wei_to_eth_u64(game.get_account_balance(king_address));
        println!("balance: {:?}", balance);

        // Decrement by 1 for the last `pay_out` call
        let times_played = game.get_account_nonce(king_address);
        println!("times_played: {:?}", times_played);

        Self {
            king,
            times_played,
            block,
            balance,
        }
    }
}
