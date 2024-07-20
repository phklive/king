use std::{convert::Infallible, fs};

use ethabi::Contract;
use rand::Rng;
use revm::{
    db::{CacheDB, EmptyDBTyped},
    primitives::{
        hex::FromHex, AccountInfo, Address, Bytecode, Bytes, ExecutionResult, Output, TxKind,
        KECCAK_EMPTY, U256,
    },
    Evm,
};

use crate::{
    agent::{Agent, Strategies, Strategy},
    constants::ETH_1,
};

// EVM
// ================================================================================================

pub fn generate_account(
    evm: &mut Evm<'_, (), CacheDB<EmptyDBTyped<Infallible>>>,
    balance: U256,
) -> Address {
    let mut rng = rand::thread_rng();
    let mut bytes = [0u8; 20];

    // Generate Address
    rng.fill(&mut bytes);
    let address = Address::from(bytes);

    // Create AccountInfo
    let account_info = AccountInfo::new(balance, 0, KECCAK_EMPTY, Bytecode::default());

    // Insert account into Evm's DB
    evm.context
        .evm
        .db
        .insert_account_info(address.clone(), account_info);

    address
}

pub fn setup_tx_env(
    evm: &mut Evm<'_, (), CacheDB<EmptyDBTyped<Infallible>>>,
    caller: Address,
    value: U256,
    to: Option<Address>,
    data: Option<Bytes>,
) {
    // set caller
    evm.context.evm.env.tx.caller = caller;

    // set TxKind
    match to {
        Some(addr) => evm.context.evm.env.tx.transact_to = TxKind::Call(addr),
        None => evm.context.evm.env.tx.transact_to = TxKind::Create,
    }

    // set data
    if let Some(d) = data {
        evm.context.evm.env.tx.data = d
    }

    // set value
    evm.context.evm.env.tx.value = value;

    // set gas price
    evm.context.evm.env.tx.gas_price = U256::ZERO;

    // set gas limit
    evm.context.evm.env.tx.gas_limit = u64::max_value();
}

// CONTRACT
// ================================================================================================

pub fn read_contract(
    bytecode_path: &str,
    abi_path: &str,
) -> Result<(Bytes, Contract), Box<dyn std::error::Error>> {
    let bytecode = fs::read_to_string(bytecode_path)?;
    let abi_json = fs::read_to_string(abi_path)?;

    let bytecode = Bytes::from_hex(&bytecode.trim())?;
    let abi: Contract = serde_json::from_str(&abi_json)?;

    Ok((bytecode, abi))
}

pub fn deploy_contract(
    evm: &mut Evm<'_, (), CacheDB<EmptyDBTyped<Infallible>>>,
    bytecode: Bytes,
) -> Result<Address, Box<dyn std::error::Error>> {
    // Create caller
    let balance = ETH_1 * U256::from(1000);
    let caller = generate_account(evm, balance);

    // setup tx env
    setup_tx_env(evm, caller, balance, None, Some(bytecode));

    // Execute tx
    let result = evm.transact_commit()?;
    let contract_address = match result {
        revm::primitives::ExecutionResult::Success { output, .. } => match output {
            Output::Create(_, Some(addr)) => addr,
            _ => return Err("Failed to get contract address".into()),
        },
        _ => return Err("Transaction has failed.".into()),
    };

    Ok(contract_address)
}

pub fn call_contract(
    evm: &mut Evm<'static, (), CacheDB<EmptyDBTyped<Infallible>>>,
    contract_address: Address,
    caller: Address,
    value: U256,
    data: Option<Bytes>,
) -> Result<Bytes, Box<dyn std::error::Error>> {
    // setup tx env
    setup_tx_env(evm, caller, value, Some(contract_address), data);

    let result = evm.transact_commit()?;
    match result {
        ExecutionResult::Success { output, .. } => Ok(output.into_data()),
        _ => Err("Transaction failed".into()),
    }
}

// AGENTS
// ================================================================================================

pub fn generate_agents(
    evm: &mut Evm<'_, (), CacheDB<EmptyDBTyped<Infallible>>>,
    strategies: Strategies,
) -> Vec<Agent> {
    let mut agents = Vec::new();
    for (strategy, num) in strategies {
        for _ in 0..num {
            let balance = match strategy {
                Strategy::Regular => ETH_1 * U256::from(10),
                Strategy::Whale => ETH_1 * U256::from(100),
                Strategy::Degen => ETH_1 * U256::from(3),
            };
            let address = generate_account(evm, balance);
            let agent = Agent::new(address, strategy.clone());
            agents.push(agent)
        }
    }

    agents
}
