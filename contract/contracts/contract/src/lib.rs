#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, Env, Address, Symbol
};

#[contract]
pub struct BulkOrderContract;

#[contracttype]
#[derive(Clone)]
pub struct Pool {
    pub total_amount: i128,
    pub threshold: i128,
    pub is_completed: bool,
}

#[contracttype]
pub enum DataKey {
    Pool,
    Contribution(Address),
}

#[contractimpl]
impl BulkOrderContract {

    // 🔹 Initialize pool with threshold
    pub fn init(env: Env, threshold: i128) {
        if env.storage().instance().has(&DataKey::Pool) {
            panic!("Already initialized");
        }

        let pool = Pool {
            total_amount: 0,
            threshold,
            is_completed: false,
        };

        env.storage().instance().set(&DataKey::Pool, &pool);
    }

    // 🔹 Contribute to the pool
    pub fn contribute(env: Env, user: Address, amount: i128) {
        user.require_auth();

        if amount <= 0 {
            panic!("Invalid amount");
        }

        let mut pool: Pool = env
            .storage()
            .instance()
            .get(&DataKey::Pool)
            .expect("Pool not initialized");

        if pool.is_completed {
            panic!("Pool already completed");
        }

        // Update total pool amount
        pool.total_amount += amount;

        // Update user's contribution
        let prev: i128 = env
            .storage()
            .instance()
            .get(&DataKey::Contribution(user.clone()))
            .unwrap_or(0);

        let new_amount = prev + amount;

        env.storage()
            .instance()
            .set(&DataKey::Contribution(user.clone()), &new_amount);

        // Check if threshold reached
        if pool.total_amount >= pool.threshold {
            pool.is_completed = true;
        }

        env.storage().instance().set(&DataKey::Pool, &pool);
    }

    // 🔹 Get pool info
    pub fn get_pool(env: Env) -> Pool {
        env.storage()
            .instance()
            .get(&DataKey::Pool)
            .expect("Pool not initialized")
    }

    // 🔹 Get user contribution
    pub fn get_contribution(env: Env, user: Address) -> i128 {
        env.storage()
            .instance()
            .get(&DataKey::Contribution(user))
            .unwrap_or(0)
    }
}