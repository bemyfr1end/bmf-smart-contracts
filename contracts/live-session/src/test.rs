use crate::{LiveSessionContract, LiveSessionContractClient};
use soroban_sdk::{Env, String};

#[test]
fn it_creates_a_live_session() {
    let env = Env::default();
    let contract_id = env.register_contract(None, LiveSessionContract);
    let client = LiveSessionContractClient::new(&env, &contract_id);

    assert_eq!(
        client.create(
            &String::from_str(&env, "random_id"),
            &String::from_str(&env, "133")
        ),
        true
    );
}