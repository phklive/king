use std::convert::Infallible;

use revm::{
    db::{CacheDB, EmptyDB, EmptyDBTyped},
    primitives::{Address, U256},
    Evm,
};

use crate::{
    agent::{Agent, Strategy},
    constants::{ABI_PATH, BYTECODE_PATH},
    contract::Contract,
    summary::Summary,
    utils::{deploy_contract, generate_agents, read_contract},
};

#[derive(Debug)]
pub struct Game {
    evm: Evm<'static, (), CacheDB<EmptyDBTyped<Infallible>>>,
    contract: Contract,
    agents: Vec<Agent>,
    ended: bool,
}

impl Game {
    // CORE
    // ================================================================================================

    pub fn new(strategies: &[(Strategy, u64)]) -> Self {
        // Instantiate Evm
        let cache_db = CacheDB::new(EmptyDB::default());
        let mut evm = Evm::builder().with_db(cache_db).build();

        // Create agents
        let agents = generate_agents(&mut evm, strategies);

        // Read contract
        let (bytecode, abi) = read_contract(BYTECODE_PATH, ABI_PATH).unwrap();

        // Deploy contract
        let contract_address = deploy_contract(&mut evm, bytecode).unwrap();

        // Instantiate Contract struct
        let contract = Contract::new(abi, contract_address);

        Self {
            evm,
            contract,
            agents: agents.to_vec(),
            ended: false,
        }
    }

    pub fn _play(&self) -> Summary {
        // play the game, update ended when one agent has won the game
        while !self.ended {}

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
