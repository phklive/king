use std::fs;

use ethabi::Contract;
use rand::Rng;
use revm::primitives::{
    hex::FromHex, AccountInfo, Address, Bytecode, Bytes, ExecutionResult, Output, TxKind,
    KECCAK_EMPTY, U256,
};

use crate::{
    agent::Agent,
    constants::ETH_1,
    types::{Strategies, Strategy, EVM},
};

// EVM
// ================================================================================================

pub fn generate_account(evm: &mut EVM, balance: U256) -> Address {
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
        .insert_account_info(address, account_info);

    address
}

pub fn setup_tx_env(
    evm: &mut EVM,
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
    evm.context.evm.env.tx.gas_limit = u64::MAX;
}

pub fn wei_to_eth_u64(wei: U256) -> u64 {
    // 1 ETH = 10^18 wei
    let eth_in_wei = U256::from(10).pow(U256::from(18));

    // Perform the division
    let eth = wei / eth_in_wei;

    // Convert to u64, saturating at u64::MAX if the value is too large
    let eth: u64 = eth.try_into().unwrap_or(u64::MAX);

    eth
}

// CONTRACT
// ================================================================================================

pub fn read_contract(
    bytecode_path: &str,
    abi_path: &str,
) -> Result<(Bytes, Contract), Box<dyn std::error::Error>> {
    let bytecode = fs::read_to_string(bytecode_path)?;
    let abi_json = fs::read_to_string(abi_path)?;

    let bytecode = Bytes::from_hex(bytecode.trim())?;
    let abi: Contract = serde_json::from_str(&abi_json)?;

    Ok((bytecode, abi))
}

pub fn deploy_contract(
    evm: &mut EVM,
    bytecode: Bytes,
    caller: Address,
) -> Result<Address, Box<dyn std::error::Error>> {
    // instantiate balance
    let balance = ETH_1 * U256::from(1000);

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
    evm: &mut EVM,
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

pub fn generate_agents(evm: &mut EVM, strategies: Strategies) -> Vec<Agent> {
    let mut agents = Vec::new();
    for (strategy, num) in strategies.0 {
        for _ in 0..num {
            let balance = match strategy {
                Strategy::Regular => ETH_1 * U256::from(10),
                Strategy::Whale => ETH_1 * U256::from(100),
                Strategy::Degen => ETH_1 * U256::from(3),
            };
            let address = generate_account(evm, balance);
            let agent = Agent::new(address, strategy);
            agents.push(agent)
        }
    }

    agents
}
