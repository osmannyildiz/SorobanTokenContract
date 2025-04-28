use crate::storage::DataKey;
use soroban_sdk::{Address, Env};

pub fn has_admin(env: &Env) -> bool {
    let key = DataKey::Admin;
    env.storage().instance().has(&key)
}

pub fn read_admin(env: &Env) -> Address {
    let key = DataKey::Admin;
    env.storage().instance().get(&key).unwrap()
}

pub fn write_admin(env: &Env, address: &Address) {
    let key = DataKey::Admin;
    env.storage().instance().set(&key, address);
}
