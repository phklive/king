use revm::primitives::Address;

use crate::game::Game;

pub trait Playable {
    fn play(&self, game: &Game);
}

#[derive(Debug, Clone, Copy)]
pub enum Strategy {
    Regular,
    Whale,
    Degen,
}

pub type Strategies = Vec<(Strategy, u8)>;

#[derive(Debug, Clone, Copy)]
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
    fn play(&self, game: &Game) {
        match self.strategy {
            Strategy::Regular => println!(
                "I am a regular, my address is: {}, and I played on block: {}",
                self.address,
                game.current_block()
            ),
            Strategy::Whale => println!(
                "I am a whale, my address is: {}, and I played on block: {}",
                self.address,
                game.current_block()
            ),
            Strategy::Degen => println!(
                "I am a degen, my address is: {}, and I played on block: {}",
                self.address,
                game.current_block()
            ),
        }
    }
}
