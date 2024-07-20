use revm::primitives::U256;

// CONTRACT
// ================================================================================================

pub const BYTECODE_PATH: &str = "res/bytecode.txt";
pub const ABI_PATH: &str = "res/abi.json";

// CONVERSIONS
// ================================================================================================

pub const ETH_0: U256 = U256::ZERO;
pub const ETH_1: U256 = U256::from_limbs([1000000000000000000, 0, 0, 0]);
