use revm::primitives::Address;
use serde::{Deserialize, Serialize};

use crate::{
    game::Game,
    types::{Playable, Strategy},
    utils::wei_to_eth_u64,
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Agent {
    address: Address,
    strategy: Strategy,
}

impl Agent {
    pub fn new(address: Address, strategy: Strategy) -> Self {
        Agent { address, strategy }
    }

    pub fn address(&self) -> &Address {
        &self.address
    }

    pub fn strategy(&self) -> &Strategy {
        &self.strategy
    }
}

impl Playable for Agent {
    fn play(&self, game: &mut Game) {
        // Don't play if you won, are winning or are out of money
        // if self.address == game.get_king().unwrap

        println!(
            "Account balance: {}",
            wei_to_eth_u64(game.get_account_balance(self.address))
        );

        match self.strategy() {
            Strategy::Regular => {
                println!(
                    "I am a regular, my address is: {}, and I played on block: {}",
                    self.address(),
                    game.get_current_block()
                );
                let _ = game.pay_in(self.address);
            }
            Strategy::Whale => println!(
                "I am a whale, my address is: {}, and I played on block: {}",
                self.address,
                game.get_current_block()
            ),
            Strategy::Degen => println!(
                "I am a degen, my address is: {}, and I played on block: {}",
                self.address,
                game.get_current_block()
            ),
        }
    }
}
