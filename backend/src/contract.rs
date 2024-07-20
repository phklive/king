use revm::primitives::{Address, Bytes, U256};

use crate::{
    constants::{ETH_0, ETH_1},
    types::Evm,
    utils::call_contract,
};

#[derive(Debug)]
pub struct Contract {
    abi: ethabi::Contract,
    address: Address,
}

impl Contract {
    pub fn new(abi: ethabi::Contract, address: Address) -> Self {
        Self { abi, address }
    }

    pub fn pay_in(
        &mut self,
        evm: &mut Evm,
        caller: Address,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let data = self.abi.function("payIn")?.encode_input(&[])?.into();

        // call contract
        let _ = call_contract(evm, self.address, caller, ETH_1, Some(data));

        Ok(())
    }

    pub fn pay_out(
        &mut self,
        evm: &mut Evm,
        caller: Address,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let data = self.abi.function("payOut")?.encode_input(&[])?.into();

        // call contract
        let _ = call_contract(evm, self.address, caller, ETH_0, Some(data));

        Ok(())
    }

    pub fn get_king(
        &mut self,
        evm: &mut Evm,
        caller: Address,
    ) -> Result<Address, Box<dyn std::error::Error>> {
        let data: Bytes = self.abi.function("king")?.encode_input(&[])?.into();
        let result = call_contract(evm, self.address, caller, ETH_0, Some(data))?;
        let king_address = Address::from_slice(&result[12..32]);

        Ok(king_address)
    }

    pub fn get_last_block(
        &mut self,
        evm: &mut Evm,
        caller: Address,
    ) -> Result<u64, Box<dyn std::error::Error>> {
        let data: Bytes = self.abi.function("lastBlock")?.encode_input(&[])?.into();

        let result = call_contract(evm, self.address, caller, ETH_0, Some(data))?;

        let array: [u8; 32] = result.as_ref().try_into().expect("Incorrect byte len");

        let last_block = U256::from_be_bytes(array);

        let last_block: u64 = last_block.try_into().unwrap();

        Ok(last_block)
    }

    pub fn get_won(
        &mut self,
        evm: &mut Evm,
        caller: Address,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let data: Bytes = self.abi.function("won")?.encode_input(&[])?.into();

        let result = call_contract(evm, self.address, caller, ETH_0, Some(data))?;

        let array: [u8; 32] = result.as_ref().try_into().expect("Incorrect byte len");

        let is_won = array[31] != 0;

        Ok(is_won)
    }
}
