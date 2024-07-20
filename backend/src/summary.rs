use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{agent::Agent, game::Game};

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
        let block = game.get_current_block();

        // Get balance of winner
        let balance = game.get_account_balance(king_address);

        // Decrement by 1 for the last `pay_out` call
        let times_played = game.get_account_nonce(king_address);

        Self {
            king,
            times_played,
            block,
            balance,
        }
    }
}

impl Display for Summary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Summary:\n\
             King: {:?}\n\
             Times Played: {}\n\
             Block: {}\n\
             Balance: {}",
            self.king, self.times_played, self.block, self.balance
        )
    }
}
