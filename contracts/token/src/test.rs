#![cfg(test)]

use crate::contract::{TokenContract, TokenContractClient};
use soroban_sdk::{testutils::Address as _, Address, Env, String};

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register(TokenContract, ());
    let client = TokenContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);

    client.initialize(
        &admin,
        &18,
        &String::from_str(&env, "Osi Token"),
        &String::from_str(&env, "OSI"),
    );

    let decimal = client.decimals();
    assert_eq!(decimal, 18);

    let name = client.name();
    assert_eq!(name, String::from_str(&env, "Osi Token"));

    let symbol = client.symbol();
    assert_eq!(symbol, String::from_str(&env, "OSI"));
}
