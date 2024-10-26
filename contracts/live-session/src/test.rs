
#![cfg(test)]
extern crate std;

use crate::{LiveSessionContract, LiveSessionContractClient};
// use soroban_sdk::{testutils::Address as _, token::{self, TokenClient}, xdr::{Asset, ContractIdPreimage, FromXdr, Limits, WriteXdr}, Address, Bytes, Env, String};
// use token::StellarAssetClient as TokenAdminClient;

use super::*;
use soroban_sdk::testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation, Ledger};
use soroban_sdk::{symbol_short, token, Address, Env, IntoVal};
use token::Client as TokenClient;
use token::StellarAssetClient as TokenAdminClient;

fn create_token_contract<'a>(e: &Env, admin: &Address) -> (TokenClient<'a>, TokenAdminClient<'a>) {
    let sac = e.register_stellar_asset_contract_v2(admin.clone());
    (
        token::Client::new(e, &sac.address()),
        token::StellarAssetClient::new(e, &sac.address()),
    )
}

fn create_live_session_contract<'a>(e: &Env) -> LiveSessionContractClient<'a> {
    LiveSessionContractClient::new(e, &e.register_contract(None, LiveSessionContract {}))
}

struct ClaimableBalanceTest<'a> {
    env: Env,
    deposit_address: Address,
    claim_addresses: [Address; 3],
    token: TokenClient<'a>,
    contract: LiveSessionContractClient<'a>,
}

impl<'a> ClaimableBalanceTest<'a> {
    fn setup() -> Self {
        let env = Env::default();
        env.mock_all_auths();

        env.ledger().with_mut(|li| {
            li.timestamp = 12345;
        });

        let deposit_address = Address::generate(&env);

        let claim_addresses = [
            Address::generate(&env),
            Address::generate(&env),
            Address::generate(&env),
        ];

        let token_admin = Address::generate(&env);

        let (token, token_admin_client) = create_token_contract(&env, &token_admin);
        token_admin_client.mint(&deposit_address, &1000);

        let contract = create_live_session_contract(&env);
        ClaimableBalanceTest {
            env,
            deposit_address,
            claim_addresses,
            token,
            contract,
        }
    }
}

#[test]
fn it_deposits_token_into_contract() {
    let test = ClaimableBalanceTest::setup();
    test.contract.deposit(
        &String::from_str(&test.env, "random_id"),
        &800,
        &test.deposit_address,
        &test.token.address,
        &test.claim_addresses[0],
    );

    let expected_auths_0 = [(
        test.deposit_address.clone(),
        AuthorizedInvocation {
            function: AuthorizedFunction::Contract((
                test.contract.address.clone(),
                symbol_short!("deposit"),
                (
                    String::from_str(&test.env, "random_id"),
                    800_i128,
                    test.deposit_address.clone(),
                    test.token.address.clone(),
                    test.claim_addresses[0].clone(),
                )
                .into_val(&test.env),
            )),
            sub_invocations: std::vec![AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    test.token.address.clone(),
                    symbol_short!("transfer"),
                    (
                        test.deposit_address.clone(),
                        &test.contract.address,
                        800_i128,
                    )
                        .into_val(&test.env),
                )),
                sub_invocations: std::vec![]
            }]
        }
    ),];

    assert_eq!(
        test.env.auths(),
        expected_auths_0
    );

    assert_eq!(test.token.balance(&test.deposit_address), 200);
    assert_eq!(test.token.balance(&test.contract.address), 800);
    assert_eq!(test.token.balance(&test.claim_addresses[0]), 0);
    assert_eq!(test.token.balance(&test.claim_addresses[1]), 0);
    assert_eq!(test.token.balance(&test.claim_addresses[2]), 0);

    test.contract.deposit(
        &String::from_str(&test.env, "random_id_1"),
        &200,
        &test.deposit_address,
        &test.token.address,
        &test.claim_addresses[0],
    );

    let expected_auths_1 = [(
        test.deposit_address.clone(),
        AuthorizedInvocation {
            function: AuthorizedFunction::Contract((
                test.contract.address.clone(),
                symbol_short!("deposit"),
                (
                    String::from_str(&test.env, "random_id_1"),
                    800_i128,
                    test.deposit_address.clone(),
                    test.token.address.clone(),
                    test.claim_addresses[0].clone(),
                )
                .into_val(&test.env),
            )),
            sub_invocations: std::vec![AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    test.token.address.clone(),
                    symbol_short!("transfer"),
                    (
                        test.deposit_address.clone(),
                        &test.contract.address,
                        800_i128,
                    )
                        .into_val(&test.env),
                )),
                sub_invocations: std::vec![]
            }]
        }
    )];

    assert_eq!(test.token.balance(&test.deposit_address), 0);
    assert_eq!(test.token.balance(&test.contract.address), 1000);
    assert_eq!(test.token.balance(&test.claim_addresses[0]), 0);
    assert_eq!(test.token.balance(&test.claim_addresses[1]), 0);
    assert_eq!(test.token.balance(&test.claim_addresses[2]), 0);
}