#![cfg(test)]

extern crate std; // For "vec!"

use crate::contract::{TokenContract, TokenContractClient};
use soroban_sdk::{
    symbol_short,
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation},
    Address, Env, IntoVal, String, Symbol,
};

fn create_token<'a>(
    env: &Env,
    admin: &Address,
    decimal: u32,
    name: &str,
    symbol: &str,
) -> TokenContractClient<'a> {
    let contract_id = env.register(TokenContract, ());
    let client = TokenContractClient::new(env, &contract_id);
    client.initialize(admin, &decimal, &name.into_val(env), &symbol.into_val(env));
    client
}

#[test]
fn test_1() {
    let env = Env::default();
    env.mock_all_auths();

    let admin1 = Address::generate(&env);
    let admin2 = Address::generate(&env);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);
    let user3 = Address::generate(&env);

    let token = create_token(&env, &admin1, 18, "Osi Token", "OSI");

    // Test metadata
    let decimal = token.decimals();
    assert_eq!(decimal, 18);
    let name = token.name();
    assert_eq!(name, String::from_str(&env, "Osi Token"));
    let symbol = token.symbol();
    assert_eq!(symbol, String::from_str(&env, "OSI"));

    // Admin 1 mints 1000 tokens to User 1
    // User 1: 0 -> 1000
    token.mint(&user1, &1000);
    assert_eq!(
        env.auths(),
        std::vec![(
            admin1.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    token.address.clone(),
                    symbol_short!("mint"),
                    (&user1, 1000_i128).into_val(&env),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );
    assert_eq!(token.balance(&user1), 1000);

    // User 2 approves 500 tokens to User 3
    token.approve(&user2, &user3, &500, &200);
    assert_eq!(
        env.auths(),
        std::vec![(
            user2.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    token.address.clone(),
                    symbol_short!("approve"),
                    (&user2, &user3, 500_i128, 200_u32).into_val(&env),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );
    assert_eq!(token.allowance(&user2, &user3), 500);

    // User 1 transfers 600 tokens to User 2
    // User 1: 1000 -> 400
    // User 2: 0 -> 600
    token.transfer(&user1, &user2, &600);
    assert_eq!(
        env.auths(),
        std::vec![(
            user1.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    token.address.clone(),
                    symbol_short!("transfer"),
                    (&user1, &user2, 600_i128).into_val(&env),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );
    assert_eq!(token.balance(&user1), 400);
    assert_eq!(token.balance(&user2), 600);

    // Using User 2's approval, User 3 transfers 400 tokens from user 2 to user 1
    // User 1: 400 -> 800
    // User 2: 600 -> 200
    token.transfer_from(&user3, &user2, &user1, &400);
    assert_eq!(
        env.auths(),
        std::vec![(
            user3.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    token.address.clone(),
                    Symbol::new(&env, "transfer_from"), // We can't use symbol_short! if length > 9
                    (&user3, &user2, &user1, 400_i128).into_val(&env),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );
    assert_eq!(token.balance(&user1), 800);
    assert_eq!(token.balance(&user2), 200);

    // User 1 transfers 300 tokens to User 3
    // User 1: 800 -> 500
    // User 3: 0 -> 300
    token.transfer(&user1, &user3, &300);
    assert_eq!(
        env.auths(),
        std::vec![(
            user1.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    token.address.clone(),
                    symbol_short!("transfer"),
                    (&user1, &user3, 300_i128).into_val(&env),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );
    assert_eq!(token.balance(&user1), 500);
    assert_eq!(token.balance(&user3), 300);

    // Admin 1 sets Admin 2 as the new admin
    token.set_admin(&admin2);
    assert_eq!(
        env.auths(),
        std::vec![(
            admin1.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    token.address.clone(),
                    symbol_short!("set_admin"),
                    (&admin2,).into_val(&env),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );

    // User 2 approves 500 tokens to User 3
    token.approve(&user2, &user3, &500, &200);
    assert_eq!(
        env.auths(),
        std::vec![(
            user2.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    token.address.clone(),
                    symbol_short!("approve"),
                    (&user2, &user3, 500_i128, 200_u32).into_val(&env),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );
    assert_eq!(token.allowance(&user2, &user3), 500);

    // User 2 removes approval for User 3
    token.approve(&user2, &user3, &0, &200);
    assert_eq!(
        env.auths(),
        std::vec![(
            user2.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    token.address.clone(),
                    symbol_short!("approve"),
                    (&user2, &user3, 0_i128, 200_u32).into_val(&env),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );
    assert_eq!(token.allowance(&user2, &user3), 0);
}

#[test]
fn test_2() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);

    let token = create_token(&env, &admin, 18, "Osi Token", "OSI");

    // Admin mints 1000 tokens to User 1
    // User 1: 0 -> 1000
    token.mint(&user1, &1000);
    assert_eq!(
        env.auths(),
        std::vec![(
            admin.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    token.address.clone(),
                    symbol_short!("mint"),
                    (&user1, 1000_i128).into_val(&env),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );
    assert_eq!(token.balance(&user1), 1000);

    // User 1 approves 500 tokens to User 2
    token.approve(&user1, &user2, &500, &200);
    assert_eq!(
        env.auths(),
        std::vec![(
            user1.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    token.address.clone(),
                    symbol_short!("approve"),
                    (&user1, &user2, 500_i128, 200_u32).into_val(&env),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );
    assert_eq!(token.allowance(&user1, &user2), 500);

    // Using User 1's approval, User 2 burns 500 tokens from user 1
    // User 1: 1000 -> 500
    token.burn_from(&user2, &user1, &500);
    assert_eq!(
        env.auths(),
        std::vec![(
            user2.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    token.address.clone(),
                    symbol_short!("burn_from"),
                    (&user2, &user1, 500_i128).into_val(&env),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );
    assert_eq!(token.allowance(&user1, &user2), 0);
    assert_eq!(token.balance(&user1), 500);
    assert_eq!(token.balance(&user2), 0);

    // User 1 burns 500 tokens
    // User 1: 500 -> 0
    token.burn(&user1, &500);
    assert_eq!(
        env.auths(),
        std::vec![(
            user1.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    token.address.clone(),
                    symbol_short!("burn"),
                    (&user1, 500_i128).into_val(&env),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );
    assert_eq!(token.balance(&user1), 0);
    assert_eq!(token.balance(&user2), 0);
}

#[test]
#[should_panic(expected = "Insufficient balance")]
fn test_transfer_insufficient_balance() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);

    let token = create_token(&env, &admin, 18, "Osi Token", "OSI");

    // Admin mints 1000 tokens to User 1
    // User 1: 0 -> 1000
    token.mint(&user1, &1000);
    assert_eq!(token.balance(&user1), 1000);

    // User 1 transfers 1001 tokens to User 2 (This should panic)
    token.transfer(&user1, &user2, &1001);
}

#[test]
#[should_panic(expected = "Insufficient allowance")]
fn test_transfer_from_insufficient_allowance() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);
    let user3 = Address::generate(&env);

    let token = create_token(&env, &admin, 18, "Osi Token", "OSI");

    // Admin mints 1000 tokens to User 1
    // User 1: 0 -> 1000
    token.mint(&user1, &1000);
    assert_eq!(token.balance(&user1), 1000);

    // User 1 approves 100 tokens to User 3
    token.approve(&user1, &user3, &100, &200);
    assert_eq!(token.allowance(&user1, &user3), 100);

    // Using User 1's approval, User 3 transfers 101 tokens from User 1 to User 2 (This should panic)
    token.transfer_from(&user3, &user1, &user2, &101);
}

#[test]
#[should_panic(expected = "Decimal must not be greater than 18")]
fn test_decimal_is_over_eighteen() {
    let env = Env::default();

    let admin = Address::generate(&env);

    // This should panic
    let _ = create_token(&env, &admin, 19, "Osi Token", "OSI");
}

#[test]
#[should_panic(expected = "Already initialized")]
fn test_initialize_already_initialized() {
    let env = Env::default();

    let admin = Address::generate(&env);

    let token = create_token(&env, &admin, 18, "Osi Token", "OSI");

    // Reinitialize token (This should panic)
    token.initialize(
        &admin,
        &18,
        &"Osi Token 2".into_val(&env),
        &"OSI2".into_val(&env),
    );
}

#[test]
fn test_zero_allowance() {
    // Here we test that transfer_from with a 0 amount does not create an empty allowance data

    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let spender = Address::generate(&env);
    let from = Address::generate(&env);

    let token = create_token(&env, &admin, 18, "Osi Token", "OSI");

    // Using From's approval, Spender transfers 0 tokens from From to Spender
    token.transfer_from(&spender, &from, &spender, &0);
    assert!(token.test_get_allowance(&from, &spender).is_none());
}
