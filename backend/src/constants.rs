use revm::primitives::U256;

// CONTRACT
// ================================================================================================

pub const BYTECODE_PATH: &str = "static/bytecode.txt";
pub const ABI_PATH: &str = "static/abi.json";

// PLAYERS
// ================================================================================================

pub const PLAYERS_PATH: &str = "static/players.json";

// CONVERSIONS
// ================================================================================================

pub const ETH_0: U256 = U256::ZERO;
pub const ETH_1: U256 = U256::from_limbs([1000000000000000000, 0, 0, 0]);
