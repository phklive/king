use core::panic;
use std::convert::Infallible;

use ethabi::Contract;
use revm::{
    db::{CacheDB, EmptyDB, EmptyDBTyped},
    primitives::{Address, Bytes, U256},
    Evm,
};

use crate::{
    agent::{Agent, Strategy},
    constants::{ABI_PATH, BYTECODE_PATH, ETH_1},
    utils::{call_contract, deploy_contract, generate_agents, read_contract},
};

#[derive(Debug)]
pub struct Game {
    evm: Evm<'static, (), CacheDB<EmptyDBTyped<Infallible>>>,
    block: U256,
    agents: Vec<Agent>,
    king: Option<Agent>,
    contract_abi: Contract,
    contract_address: Address,
    ended: bool,
}

impl Game {
    pub fn new(strategies: &[(Strategy, u64)]) -> Self {
        // Instantiate Evm
        let cache_db = CacheDB::new(EmptyDB::default());
        let mut evm = Evm::builder().with_db(cache_db).build();

        println!("Evm clean: {:#?}", evm);
        panic!("Stop for test");

        // Read contract
        let (bytecode, abi) = read_contract(BYTECODE_PATH, ABI_PATH).unwrap();

        // Deploy contract
        let contract_address = deploy_contract(&mut evm, bytecode).unwrap();

        // Create agents
        let agents = generate_agents(&mut evm, strategies);

        Self {
            evm,
            agents: agents.to_vec(),
            king: None,
            contract_abi: abi,
            contract_address,
            ended: false,
            block: U256::ZERO,
        }
    }

    pub fn pay_in(&mut self, caller: Address) -> Result<(), Box<dyn std::error::Error>> {
        let data = self
            .contract_abi
            .function("payIn")?
            .encode_input(&[])?
            .into();

        // call contract
        let _ = call_contract(
            &mut self.evm,
            self.contract_address,
            caller,
            ETH_1,
            Some(data),
        );

        println!("payed in by {}", caller);

        Ok(())
    }

    pub fn king(&mut self, caller: Address) -> Result<Address, Box<dyn std::error::Error>> {
        let data: Bytes = self
            .contract_abi
            .function("king")?
            .encode_input(&[])?
            .into();
        let result = call_contract(
            &mut self.evm,
            self.contract_address,
            caller,
            U256::ZERO,
            Some(data),
        )?;
        let king_address = Address::from_slice(&result[12..32]);

        println!("King query by {}", caller);
        println!("King result: {}", king_address);

        Ok(king_address)
    }

    // pub fn last_block(&mut self, caller: Address) -> Result<>

    pub fn current_block(&self) -> U256 {
        U256::from(self.block)
    }

    pub fn advance_block(&mut self, increment: u64) {
        self.evm.context.evm.env.block.number += U256::from(increment);
        self.block = self.evm.context.evm.env.block.number
    }

    pub fn agents(&self) -> &[Agent] {
        self.agents.as_slice()
    }
}
