use revm::primitives::U256;

// CONTRACT
// ================================================================================================

pub const BYTECODE_PATH: &str = "contract/bytecode.txt";
pub const ABI_PATH: &str = "contract/abi.json";

// CONVERSIONS
// ================================================================================================

pub const ETH_1: U256 = U256::from_limbs([1000000000000000000, 0, 0, 0]);
