use rand::Rng;
use revm::primitives::Address;
use serde::{Deserialize, Serialize};

use crate::{
    game::Game,
    types::{Playable, Strategy},
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
        // Global play conditions:
        // - Don't play if you are already king
        // - Don't play if the game has already ended
        // - Don't play if you have no more money
        let already_king = self.address == game.get_king().unwrap();
        let won = game.get_won().unwrap();
        let balance = game.get_account_balance(self.address);

        if already_king && won {
            // If you have won the game payout
            game.pay_out(self.address).unwrap();
            return;
        } else if already_king || won || balance == 0 {
            return;
        }

        match self.strategy() {
            Strategy::Analyst => {
                // Analyst only plays 1 block before game ends
                let current_block = game.get_current_block();
                let last_block = game.get_last_block().unwrap();

                if current_block == last_block - 1 || current_block > last_block {
                    game.pay_in(self.address).unwrap()
                }
            }
            Strategy::Whale => {
                // Whale plays all the time
                game.pay_in(self.address).unwrap()
            }
            Strategy::Degen => {
                // Degen plays half of the time
                let rand = rand::thread_rng().gen_range(0..10);

                if rand % 2 == 0 {
                    game.pay_in(self.address).unwrap()
                }
            }
        }
    }
}
