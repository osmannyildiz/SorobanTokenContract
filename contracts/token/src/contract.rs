use crate::{
    constants::{INSTANCE_TTL_FULL, INSTANCE_TTL_THRESHOLD},
    storage::DataKey,
    utils::{
        admin::{has_admin, read_admin, write_admin},
        allowance::{read_allowance, spend_allowance, write_allowance},
        balance::{decrease_balance, increase_balance, read_balance},
        metadata::{read_decimal, read_name, read_symbol, write_metadata},
    },
};
use soroban_sdk::{
    contract, contractimpl,
    token::{self, Interface as _},
    Address, Env, String,
};
use soroban_token_sdk::{metadata::TokenMetadata, TokenUtils};

#[cfg(test)]
use crate::storage::{AllowanceData, AllowanceDataKey};

fn assert_nonnegative_amount(amount: i128) {
    if amount < 0 {
        panic!("Negative amount is not allowed: {}", amount);
    }
}

fn assert_account_not_frozen(env: &Env, account: &Address) {
    let key = DataKey::Frozen(account.clone());
    if env
        .storage()
        .instance()
        .get::<_, bool>(&key)
        .unwrap_or(false)
    {
        panic!("Account is frozen");
    }
}

fn emit_custom_event(env: &Env, event_type: &str, admin: Address, account: Address) {
    let topics = (event_type, admin, account);
    let data = ();
    env.events().publish(topics, data);
}

#[contract]
pub struct TokenContract;

#[contractimpl]
impl TokenContract {
    pub fn initialize(env: Env, admin: Address, decimal: u32, name: String, symbol: String) {
        if has_admin(&env) {
            panic!("Already initialized");
        }

        // https://solana.stackexchange.com/q/1293
        if decimal > 18 {
            panic!("Decimal must not be greater than 18");
        }

        write_admin(&env, &admin);

        write_metadata(
            &env,
            TokenMetadata {
                decimal,
                name,
                symbol,
            },
        );
    }

    pub fn mint(env: Env, to: Address, amount: i128) {
        let admin = read_admin(&env);
        admin.require_auth();

        assert_nonnegative_amount(amount);

        env.storage()
            .instance()
            .extend_ttl(INSTANCE_TTL_THRESHOLD, INSTANCE_TTL_FULL);

        increase_balance(&env, to.clone(), amount);

        TokenUtils::new(&env).events().mint(admin, to, amount);
    }

    pub fn set_admin(env: Env, new_admin: Address) {
        let admin = read_admin(&env);
        admin.require_auth();

        env.storage()
            .instance()
            .extend_ttl(INSTANCE_TTL_THRESHOLD, INSTANCE_TTL_FULL);

        write_admin(&env, &new_admin);

        TokenUtils::new(&env).events().set_admin(admin, new_admin);
    }

    pub fn freeze_account(env: Env, account: Address) {
        let admin = read_admin(&env);
        admin.require_auth();

        env.storage()
            .instance()
            .extend_ttl(INSTANCE_TTL_THRESHOLD, INSTANCE_TTL_FULL);

        let key = DataKey::Frozen(account.clone());
        env.storage().instance().set(&key, &true);

        emit_custom_event(&env, "freeze_account", admin, account);
    }

    pub fn unfreeze_account(env: Env, account: Address) {
        let admin = read_admin(&env);
        admin.require_auth();

        env.storage()
            .instance()
            .extend_ttl(INSTANCE_TTL_THRESHOLD, INSTANCE_TTL_FULL);

        let key = DataKey::Frozen(account.clone());
        env.storage().instance().remove(&key);

        emit_custom_event(&env, "unfreeze_account", admin, account);
    }

    #[cfg(test)]
    pub fn test_get_allowance(env: Env, from: Address, spender: Address) -> Option<AllowanceData> {
        let key = DataKey::Allowance(AllowanceDataKey {
            owner: from,
            spender,
        });
        env.storage().temporary().get::<_, AllowanceData>(&key)
    }
}

#[contractimpl]
impl token::Interface for TokenContract {
    fn allowance(env: Env, from: Address, spender: Address) -> i128 {
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_TTL_THRESHOLD, INSTANCE_TTL_FULL);

        read_allowance(&env, from, spender).amount
    }

    fn approve(env: Env, from: Address, spender: Address, amount: i128, expiration_ledger: u32) {
        from.require_auth();

        assert_nonnegative_amount(amount);

        env.storage()
            .instance()
            .extend_ttl(INSTANCE_TTL_THRESHOLD, INSTANCE_TTL_FULL);

        write_allowance(
            &env,
            from.clone(),
            spender.clone(),
            amount,
            expiration_ledger,
        );

        TokenUtils::new(&env)
            .events()
            .approve(from, spender, amount, expiration_ledger);
    }

    fn balance(env: Env, id: Address) -> i128 {
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_TTL_THRESHOLD, INSTANCE_TTL_FULL);

        read_balance(&env, id)
    }

    fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();

        assert_nonnegative_amount(amount);
        assert_account_not_frozen(&env, &from);

        env.storage()
            .instance()
            .extend_ttl(INSTANCE_TTL_THRESHOLD, INSTANCE_TTL_FULL);

        decrease_balance(&env, from.clone(), amount);
        increase_balance(&env, to.clone(), amount);

        TokenUtils::new(&env).events().transfer(from, to, amount);
    }

    fn transfer_from(env: Env, spender: Address, from: Address, to: Address, amount: i128) {
        spender.require_auth();

        assert_nonnegative_amount(amount);
        assert_account_not_frozen(&env, &from);

        env.storage()
            .instance()
            .extend_ttl(INSTANCE_TTL_THRESHOLD, INSTANCE_TTL_FULL);

        spend_allowance(&env, from.clone(), spender, amount);
        decrease_balance(&env, from.clone(), amount);
        increase_balance(&env, to.clone(), amount);

        TokenUtils::new(&env).events().transfer(from, to, amount);
    }

    fn burn(env: Env, from: Address, amount: i128) {
        from.require_auth();

        assert_nonnegative_amount(amount);
        assert_account_not_frozen(&env, &from);

        env.storage()
            .instance()
            .extend_ttl(INSTANCE_TTL_THRESHOLD, INSTANCE_TTL_FULL);

        decrease_balance(&env, from.clone(), amount);

        TokenUtils::new(&env).events().burn(from, amount);
    }

    fn burn_from(env: Env, spender: Address, from: Address, amount: i128) {
        spender.require_auth();

        assert_nonnegative_amount(amount);
        assert_account_not_frozen(&env, &from);

        env.storage()
            .instance()
            .extend_ttl(INSTANCE_TTL_THRESHOLD, INSTANCE_TTL_FULL);

        spend_allowance(&env, from.clone(), spender, amount);
        decrease_balance(&env, from.clone(), amount);

        TokenUtils::new(&env).events().burn(from, amount);
    }

    fn decimals(env: Env) -> u32 {
        read_decimal(&env)
    }

    fn name(env: Env) -> String {
        read_name(&env)
    }

    fn symbol(env: Env) -> String {
        read_symbol(&env)
    }
}
