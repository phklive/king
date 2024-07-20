use std::convert::Infallible;

use revm::{
    db::{CacheDB, EmptyDB, EmptyDBTyped},
    primitives::{Address, U256},
    Evm,
};

use crate::{
    agent::{Agent, Playable, Strategies},
    constants::{ABI_PATH, BYTECODE_PATH, ETH_1},
    contract::Contract,
    summary::Summary,
    utils::{deploy_contract, generate_account, generate_agents, read_contract},
};

#[derive(Debug)]
pub struct Game {
    evm: Evm<'static, (), CacheDB<EmptyDBTyped<Infallible>>>,
    contract: Contract,
    agents: Vec<Agent>,
    ended: bool,
    master: Address,
}

impl Game {
    // CORE
    // ================================================================================================

    pub fn new(strategies: Strategies) -> Self {
        // Instantiate Evm
        let cache_db = CacheDB::new(EmptyDB::default());
        let mut evm = Evm::builder().with_db(cache_db).build();

        // Create agents
        let agents = generate_agents(&mut evm, strategies);

        // Read contract
        let (bytecode, abi) = read_contract(BYTECODE_PATH, ABI_PATH).unwrap();

        // Create game master
        let balance = ETH_1 * U256::from(1000);
        let master = generate_account(&mut evm, balance);

        // Deploy contract
        let contract_address = deploy_contract(&mut evm, bytecode, master).unwrap();

        // Instantiate Contract struct
        let contract = Contract::new(abi, contract_address);

        Self {
            evm,
            contract,
            agents: agents.to_vec(),
            ended: false,
            master,
        }
    }

    pub fn play(&mut self) -> Summary {
        let mut counter = 0;
        // play the game, update ended when one agent has won the game
        while !self.ended {
            counter += 1;

            let agents = self.agents.clone();

            // loop over agents and make them play
            for agent in agents {
                agent.play(self)
            }

            // advance block by 1
            self.advance_block(1);

            if self.get_won(self.master).unwrap() || counter == 100000 {
                self.ended = true;
            }
        }

        // return summary of the game to frontend
        Summary::new(self.agents[0], 10)
    }

    pub fn agents(&self) -> &[Agent] {
        self.agents.as_slice()
    }

    // EVM
    // ================================================================================================

    pub fn current_block(&self) -> U256 {
        self.evm.context.evm.env.block.number
    }

    pub fn advance_block(&mut self, increment: u64) {
        self.evm.context.evm.env.block.number += U256::from(increment);
    }

    // CONTRACT
    // ================================================================================================

    pub fn pay_in(&mut self, caller: Address) -> Result<(), Box<dyn std::error::Error>> {
        self.contract.pay_in(&mut self.evm, caller)
    }

    pub fn pay_out(&mut self, caller: Address) -> Result<(), Box<dyn std::error::Error>> {
        self.contract.pay_out(&mut self.evm, caller)
    }

    pub fn get_king(&mut self, caller: Address) -> Result<Address, Box<dyn std::error::Error>> {
        self.contract.get_king(&mut self.evm, caller)
    }

    pub fn get_last_block(&mut self, caller: Address) -> Result<U256, Box<dyn std::error::Error>> {
        self.contract.get_last_block(&mut self.evm, caller)
    }

    pub fn get_won(&mut self, caller: Address) -> Result<bool, Box<dyn std::error::Error>> {
        self.contract.get_won(&mut self.evm, caller)
    }
}
