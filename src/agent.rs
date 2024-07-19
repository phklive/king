use revm::primitives::Address;

pub trait Playable {
    fn play(&self, block: u64);
}

#[derive(Debug, Clone, Copy)]
pub enum Strategy {
    Regular,
    Whale,
    Degen,
}

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
    fn play(&self, block: u64) {
        match self.strategy {
            Strategy::Regular => println!(
                "I am a regular, my address is: {}, and I played on block: {}",
                self.address, block
            ),
            Strategy::Whale => println!(
                "I am a whale, my address is: {}, and I played on block: {}",
                self.address, block
            ),
            Strategy::Degen => println!(
                "I am a degen, my address is: {}, and I played on block: {}",
                self.address, block
            ),
        }
    }
}
