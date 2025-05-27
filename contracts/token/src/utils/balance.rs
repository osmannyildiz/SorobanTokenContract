use crate::{
    constants::{BALANCE_TTL_FULL, BALANCE_TTL_THRESHOLD},
    storage::DataKey,
};
use soroban_sdk::{Address, Env};

pub fn read_balance(env: &Env, address: Address) -> i128 {
    let key = DataKey::Balance(address);
    if let Some(balance) = env.storage().persistent().get::<DataKey, i128>(&key) {
        env.storage()
            .persistent()
            .extend_ttl(&key, BALANCE_TTL_THRESHOLD, BALANCE_TTL_FULL);

        balance
    } else {
        0
    }
}

fn write_balance(env: &Env, address: Address, amount: i128) {
    let key = DataKey::Balance(address);
    env.storage().persistent().set(&key, &amount);

    env.storage()
        .persistent()
        .extend_ttl(&key, BALANCE_TTL_THRESHOLD, BALANCE_TTL_FULL);
}

pub fn increase_balance(env: &Env, address: Address, amount: i128) {
    let balance = read_balance(env, address.clone());
    write_balance(env, address, balance + amount);
}

pub fn decrease_balance(env: &Env, address: Address, amount: i128) {
    let balance = read_balance(env, address.clone());

    if balance < amount {
        panic!("Insufficient balance");
    }

    write_balance(env, address, balance - amount);
}
