#![cfg(test)]
extern crate std;

use super::*;
use soroban_sdk::testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation, Ledger};
use soroban_sdk::{symbol_short, token, vec, Address, Env, IntoVal, String};
use token::Client as TokenClient;
use token::StellarAssetClient as TokenAdminClient;

fn create_token_contract<'a>(e: &Env, admin: &Address) -> (TokenClient<'a>, TokenAdminClient<'a>) {
    let sac = e.register_stellar_asset_contract_v2(admin.clone());
    (
        token::Client::new(e, &sac.address()),
        token::StellarAssetClient::new(e, &sac.address()),
    )
}

fn create_wager_contract<'a>(e: &Env) -> WagerContractClient<'a> {
    WagerContractClient::new(e, &e.register_contract(None, WagerContract {}))
}

struct ClaimableBalanceTest<'a> {
    env: Env,
    deposit_address: Address,
    players_address: [Address; 3],
    token: TokenClient<'a>,
    contract: WagerContractClient<'a>,
}

impl<'a> ClaimableBalanceTest<'a> {
    fn setup() -> Self {
        let env = Env::default();
        env.mock_all_auths();

        env.ledger().with_mut(|li| {
            li.timestamp = 12345;
        });

        let deposit_address = Address::generate(&env);

        let players_address = [
            Address::generate(&env),
            Address::generate(&env),
            Address::generate(&env),
        ];

        let token_admin = Address::generate(&env);

        let (token, token_admin_client) = create_token_contract(&env, &token_admin);
        token_admin_client.mint(&deposit_address, &1000);
        token_admin_client.mint(&players_address[0], &1000);
        token_admin_client.mint(&players_address[1], &1000);
        token_admin_client.mint(&players_address[2], &1000);

        let contract = create_wager_contract(&env);
        ClaimableBalanceTest {
            env,
            deposit_address,
            players_address,
            token,
            contract,
        }
    }
}

#[test]
fn it_deposits_and_dispatch() {
    let test = ClaimableBalanceTest::setup();
    let game_id = String::from_str(&test.env, "game_id");

    test.contract.deposit(
        &game_id,
        &test.players_address[0],
        &test.token.address,
        &800,
    );

    assert_eq!(test.token.balance(&test.players_address[0]), 200);
    assert_eq!(test.token.balance(&test.contract.address), 800);
    assert_eq!(test.token.balance(&test.deposit_address), 1000);
    assert_eq!(test.token.balance(&test.players_address[1]), 1000);
    assert_eq!(test.token.balance(&test.players_address[2]), 1000);

    test.contract.deposit(
        &game_id,
        &test.players_address[1],
        &test.token.address,
        &800,
    );

    assert_eq!(test.token.balance(&test.players_address[1]), 200);
    assert_eq!(test.token.balance(&test.contract.address), 1600);
    assert_eq!(test.token.balance(&test.deposit_address), 1000);
    assert_eq!(test.token.balance(&test.players_address[0]), 200);
    assert_eq!(test.token.balance(&test.players_address[2]), 1000);

    test.contract.dispatch(
        &game_id,
        &test.players_address[1],
    );

    assert_eq!(test.token.balance(&test.players_address[1]), 1800);
    assert_eq!(test.token.balance(&test.contract.address), 0);
    assert_eq!(test.token.balance(&test.deposit_address), 1000);
    assert_eq!(test.token.balance(&test.players_address[0]), 200);
    assert_eq!(test.token.balance(&test.players_address[2]), 1000);
}
