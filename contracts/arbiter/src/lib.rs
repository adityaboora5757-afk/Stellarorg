#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

#[contracttype]
#[derive(Clone)]
enum DataKey {
    Admin,
}

#[contract]
pub struct ArbiterContract;

#[contractimpl]
impl ArbiterContract {
    pub fn initialize(env: Env, admin: Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("already initialized");
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
    }

    pub fn is_authorized(env: Env, address: Address) -> bool {
        if let Some(admin) = env.storage().instance().get::<_, Address>(&DataKey::Admin) {
            address == admin
        } else {
            false
        }
    }
}
