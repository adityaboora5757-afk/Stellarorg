#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, contracterror, vec, Address, Env, Symbol, Val, Vec};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    NotAuthorized = 1,
    AlreadyCompleted = 2,
    OutOfBounds = 3,
    InvalidAmount = 4,
    AlreadyInitialized = 5,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Milestone {
    pub amount: i128,
    pub description: soroban_sdk::String,
    pub is_completed: bool,
}

#[contracttype]
#[derive(Clone)]
enum DataKey {
    Funder,
    Provider,
    ArbiterContractId,
    TokenId,
    TotalAmount,
    Milestones,
}

#[contract]
pub struct EscrowContract;

#[contractimpl]
impl EscrowContract {
    pub fn initialize(
        env: Env,
        funder: Address,
        provider: Address,
        arbiter_contract_id: Address,
        token_id: Address,
        milestones: Vec<Milestone>,
    ) -> Result<(), Error> {
        if env.storage().instance().has(&DataKey::Funder) {
            return Err(Error::AlreadyInitialized);
        }

        // Calculate total amount
        let mut total_amount: i128 = 0;
        for i in 0..milestones.len() {
            let m = milestones.get(i).unwrap();
            if m.amount <= 0 {
                return Err(Error::InvalidAmount);
            }
            total_amount = total_amount.checked_add(m.amount).unwrap();
        }

        env.storage().instance().set(&DataKey::Funder, &funder);
        env.storage().instance().set(&DataKey::Provider, &provider);
        env.storage().instance().set(&DataKey::ArbiterContractId, &arbiter_contract_id);
        env.storage().instance().set(&DataKey::TokenId, &token_id);
        env.storage().instance().set(&DataKey::Milestones, &milestones);
        env.storage().instance().set(&DataKey::TotalAmount, &total_amount);

        // Lock funder's tokens by calling transfer on the token contract.
        let token_client = soroban_sdk::token::Client::new(&env, &token_id);
        token_client.transfer(&funder, &env.current_contract_address(), &total_amount);

        Ok(())
    }

    pub fn release_milestone(env: Env, milestone_idx: u32) -> Result<(), Error> {
        // Load configurations
        let funder: Address = env.storage().instance().get(&DataKey::Funder).unwrap();
        let provider: Address = env.storage().instance().get(&DataKey::Provider).unwrap();
        let arbiter_contract_id: Address = env
            .storage()
            .instance()
            .get(&DataKey::ArbiterContractId)
            .unwrap();
        let token_id: Address = env.storage().instance().get(&DataKey::TokenId).unwrap();
        let mut milestones: Vec<Milestone> = env
            .storage()
            .instance()
            .get(&DataKey::Milestones)
            .unwrap();

        // Check bounds
        if milestone_idx >= milestones.len() {
            return Err(Error::OutOfBounds);
        }

        // Get milestone
        let mut milestone = milestones.get(milestone_idx).unwrap();
        if milestone.is_completed {
            return Err(Error::AlreadyCompleted);
        }

        // Verify authorization from the arbiter contract
        let args: Vec<Val> = vec![&env, env.current_contract_address().to_val()];
        let is_auth: bool = env.invoke_contract(
            &arbiter_contract_id,
            &Symbol::new(&env, "is_authorized"),
            args,
        );

        if !is_auth {
            return Err(Error::NotAuthorized);
        }

        // Mark milestone as completed
        milestone.is_completed = true;
        milestones.set(milestone_idx, milestone.clone());
        env.storage().instance().set(&DataKey::Milestones, &milestones);

        // Transfer milestone amount from escrow contract to provider
        let token_client = soroban_sdk::token::Client::new(&env, &token_id);
        token_client.transfer(&env.current_contract_address(), &provider, &milestone.amount);

        Ok(())
    }

    // Getters for frontend/verification
    pub fn funder(env: Env) -> Address {
        env.storage().instance().get(&DataKey::Funder).unwrap()
    }

    pub fn provider(env: Env) -> Address {
        env.storage().instance().get(&DataKey::Provider).unwrap()
    }

    pub fn arbiter_contract_id(env: Env) -> Address {
        env.storage().instance().get(&DataKey::ArbiterContractId).unwrap()
    }

    pub fn token_id(env: Env) -> Address {
        env.storage().instance().get(&DataKey::TokenId).unwrap()
    }

    pub fn total_amount(env: Env) -> i128 {
        env.storage().instance().get(&DataKey::TotalAmount).unwrap()
    }

    pub fn milestones(env: Env) -> Vec<Milestone> {
        env.storage().instance().get(&DataKey::Milestones).unwrap()
    }
}

#[cfg(test)]
mod test;
