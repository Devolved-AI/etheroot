use std::collections::BTreeMap;

pub trait AddressValidation {
    fn is_valid_address(&self, address: &str) -> bool;
}

pub struct AddressValidator;

impl AddressValidation for AddressValidator {
    fn is_valid_address(&self, _address: &str) -> bool {
        true // This is a stub. Update with real address validation logic
    }
}

type Address = String;

#[derive(Debug)]
pub enum BalanceError {
    InvalidAddress,
}

struct Account {
    address_validator: AddressValidator,
    balances: BTreeMap<Address, u128>,
}

impl Account {
    pub fn new() -> Self {
        Self {
            address_validator: AddressValidator,
            balances: BTreeMap::new(),
        }
    }

    pub fn set_balance(&mut self, address: Address, amount: u128) -> Result<(), BalanceError> {
        if !self.address_validator.is_valid_address(&address) {
            return Err(BalanceError::InvalidAddress);
        }
        self.balances.insert(address, amount);
        Ok(())
    }

    pub fn balance(&self, address: &Address) -> Result<u128, BalanceError> {
        if !self.address_validator.is_valid_address(address) {
            return Err(BalanceError::InvalidAddress);
        }

        Ok(*self.balances.get(address).unwrap_or(&0))
    }
}
