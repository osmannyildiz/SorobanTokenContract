use crate::storage::{AllowanceData, AllowanceDataKey, DataKey};
use soroban_sdk::{Address, Env};

pub fn read_allowance(env: &Env, owner: Address, spender: Address) -> AllowanceData {
    let key = DataKey::Allowance(AllowanceDataKey { owner, spender });
    if let Some(allowance) = env.storage().temporary().get::<_, AllowanceData>(&key) {
        if allowance.expiration_ledgers < env.ledger().sequence() {
            AllowanceData {
                amount: 0,
                expiration_ledgers: allowance.expiration_ledgers,
            }
        } else {
            allowance
        }
    } else {
        AllowanceData {
            amount: 0,
            expiration_ledgers: 0,
        }
    }
}

pub fn write_allowance(
    env: &Env,
    owner: Address,
    spender: Address,
    amount: i128,
    expiration_ledgers: u32,
) {
    let allowance = AllowanceData {
        amount,
        expiration_ledgers,
    };

    if amount > 0 && expiration_ledgers < env.ledger().sequence() {
        panic!("expiration_ledgers can't be less than the current ledgers when amount > 0");
    }

    let key = DataKey::Allowance(AllowanceDataKey { owner, spender });
    env.storage().temporary().set(&key, &allowance);

    if amount > 0 {
        let live_for = expiration_ledgers
            .checked_sub(env.ledger().sequence())
            .unwrap();

        env.storage()
            .temporary()
            .extend_ttl(&key, live_for, live_for)
    }
}

pub fn spend_allowance(env: &Env, owner: Address, spender: Address, spend_amount: i128) {
    let allowance = read_allowance(env, owner.clone(), spender.clone());

    if allowance.amount < spend_amount {
        panic!("insufficient allowance");
    }

    let new_amount = allowance.amount - spend_amount;
    write_allowance(
        env,
        owner,
        spender,
        new_amount,
        allowance.expiration_ledgers,
    );
}
