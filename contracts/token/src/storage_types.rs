use soroban_sdk::{contracttype, Address};

#[derive(Clone)]
#[contracttype]
pub struct AllowanceDataKey {
    pub owner: Address,
    pub spender: Address,
}

#[contracttype]
pub struct AllowanceData {
    pub amount: i128,
    pub expiration_ledgers: u32,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Allowance(AllowanceDataKey),
    Balance(Address),
    Nonce(Address),
    State(Address),
    Admin,
}
