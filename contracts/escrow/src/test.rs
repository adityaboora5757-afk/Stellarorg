#![cfg(test)]
use super::{EscrowContract, EscrowContractClient, Milestone, Error};
use soroban_sdk::{
    contract, contractimpl, vec, Address, Env, Symbol, Vec,
};
use soroban_sdk::testutils::Address as _;

// Mock Arbiter contract for testing inter-contract call
#[contract]
pub struct MockArbiter;

#[contractimpl]
impl MockArbiter {
    pub fn is_authorized(env: Env, _address: Address) -> bool {
        // Read an auth flag from storage, default to true
        env.storage()
            .instance()
            .get::<_, bool>(&Symbol::new(&env, "auth_flag"))
            .unwrap_or(true)
    }

    pub fn set_auth(env: Env, auth: bool) {
        env.storage()
            .instance()
            .set(&Symbol::new(&env, "auth_flag"), &auth);
    }
}

// Helper struct to hold test setup objects
struct TestSetup {
    env: Env,
    escrow_client: EscrowContractClient<'static>,
    funder: Address,
    provider: Address,
    arbiter_id: Address,
    token_id: Address,
    milestones: Vec<Milestone>,
}

fn setup_test(env: &Env) -> TestSetup {
    // mock non-root authorization for the cross-contract transfer
    env.mock_all_auths_allowing_non_root_auth();

    let funder = Address::generate(env);
    let provider = Address::generate(env);

    // Register Mock Arbiter
    let arbiter_id = env.register_contract(None, MockArbiter);

    // Register Mock SAC Token
    let token_admin = Address::generate(env);
    let token_id = env.register_stellar_asset_contract(token_admin.clone());
    let token_admin_client = soroban_sdk::token::StellarAssetClient::new(env, &token_id);
    let token_client = soroban_sdk::token::Client::new(env, &token_id);

    // Define milestones
    let milestones = vec![
        env,
        Milestone {
            amount: 100,
            description: soroban_sdk::String::from_str(env, "First Milestone"),
            is_completed: false,
        },
        Milestone {
            amount: 200,
            description: soroban_sdk::String::from_str(env, "Second Milestone"),
            is_completed: false,
        },
    ];

    // Mint tokens to funder (total = 300)
    token_admin_client.mint(&funder, &300i128);
    assert_eq!(token_client.balance(&funder), 300i128);

    // Register and initialize Escrow contract
    let escrow_id = env.register_contract(None, EscrowContract);
    let escrow_client = EscrowContractClient::new(env, &escrow_id);

    TestSetup {
        env: env.clone(),
        escrow_client,
        funder,
        provider,
        arbiter_id,
        token_id,
        milestones,
    }
}

#[test]
fn test_initialization() {
    let env = Env::default();
    let setup = setup_test(&env);

    // Should succeed and return ()
    setup.escrow_client.initialize(
        &setup.funder,
        &setup.provider,
        &setup.arbiter_id,
        &setup.token_id,
        &setup.milestones,
    );

    // Verify metadata was stored correctly
    assert_eq!(setup.escrow_client.funder(), setup.funder);
    assert_eq!(setup.escrow_client.provider(), setup.provider);
    assert_eq!(setup.escrow_client.arbiter_contract_id(), setup.arbiter_id);
    assert_eq!(setup.escrow_client.token_id(), setup.token_id);
    assert_eq!(setup.escrow_client.total_amount(), 300);

    let contract_milestones = setup.escrow_client.milestones();
    assert_eq!(contract_milestones.len(), 2);
    assert_eq!(contract_milestones.get(0).unwrap().amount, 100);
    assert_eq!(contract_milestones.get(1).unwrap().amount, 200);

    // Verify tokens were locked in the escrow contract
    let token_client = soroban_sdk::token::Client::new(&env, &setup.token_id);
    assert_eq!(token_client.balance(&setup.escrow_client.address), 300);
    assert_eq!(token_client.balance(&setup.funder), 0);
}

#[test]
fn test_release_milestone_success() {
    let env = Env::default();
    let setup = setup_test(&env);

    setup.escrow_client.initialize(
        &setup.funder,
        &setup.provider,
        &setup.arbiter_id,
        &setup.token_id,
        &setup.milestones,
    );

    // Release first milestone - should succeed and return ()
    setup.escrow_client.release_milestone(&0);

    // Verify state update
    let contract_milestones = setup.escrow_client.milestones();
    assert!(contract_milestones.get(0).unwrap().is_completed);
    assert!(!contract_milestones.get(1).unwrap().is_completed);

    // Verify token balances
    let token_client = soroban_sdk::token::Client::new(&env, &setup.token_id);
    assert_eq!(token_client.balance(&setup.provider), 100);
    assert_eq!(token_client.balance(&setup.escrow_client.address), 200);
}

#[test]
fn test_release_milestone_unauthorized() {
    let env = Env::default();
    let setup = setup_test(&env);

    setup.escrow_client.initialize(
        &setup.funder,
        &setup.provider,
        &setup.arbiter_id,
        &setup.token_id,
        &setup.milestones,
    );

    // Set mock arbiter authorization to false
    let arbiter_client = MockArbiterClient::new(&env, &setup.arbiter_id);
    arbiter_client.set_auth(&false);

    // Attempt to release milestone, should return Err
    let res = setup.escrow_client.try_release_milestone(&0);
    assert!(res.is_err() || res.unwrap().is_err());
}

#[test]
fn test_release_already_completed() {
    let env = Env::default();
    let setup = setup_test(&env);

    setup.escrow_client.initialize(
        &setup.funder,
        &setup.provider,
        &setup.arbiter_id,
        &setup.token_id,
        &setup.milestones,
    );

    // First release succeeds
    setup.escrow_client.release_milestone(&0);

    // Re-releasing the same index should fail
    let res = setup.escrow_client.try_release_milestone(&0);
    assert!(res.is_err() || res.unwrap().is_err());
}

#[test]
fn test_release_out_of_bounds() {
    let env = Env::default();
    let setup = setup_test(&env);

    setup.escrow_client.initialize(
        &setup.funder,
        &setup.provider,
        &setup.arbiter_id,
        &setup.token_id,
        &setup.milestones,
    );

    // Attempt to release out-of-bounds index, should fail
    let res = setup.escrow_client.try_release_milestone(&2);
    assert!(res.is_err() || res.unwrap().is_err());
}

#[test]
fn test_initialize_zero_amount() {
    let env = Env::default();
    let setup = setup_test(&env);

    let invalid_milestones = vec![
        &env,
        Milestone {
            amount: 0,
            description: soroban_sdk::String::from_str(&env, "Invalid Milestone"),
            is_completed: false,
        },
    ];

    let res = setup.escrow_client.try_initialize(
        &setup.funder,
        &setup.provider,
        &setup.arbiter_id,
        &setup.token_id,
        &invalid_milestones,
    );
    assert!(res.is_err() || res.unwrap().is_err());
}

#[test]
fn test_initialize_twice() {
    let env = Env::default();
    let setup = setup_test(&env);

    setup.escrow_client.initialize(
        &setup.funder,
        &setup.provider,
        &setup.arbiter_id,
        &setup.token_id,
        &setup.milestones,
    );

    // Initializing again should fail
    let res = setup.escrow_client.try_initialize(
        &setup.funder,
        &setup.provider,
        &setup.arbiter_id,
        &setup.token_id,
        &setup.milestones,
    );
    assert!(res.is_err() || res.unwrap().is_err());
}
