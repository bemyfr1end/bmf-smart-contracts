#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env, String};

#[contracttype]
#[derive(PartialEq)]
pub enum Status {
    Pending,
    Success,
    Fail,
}

#[contracttype]
pub enum DataKey {
    Balance(String),
    Status(String),
}

#[contract]
pub struct LiveSessionContract;

#[contractimpl]
impl LiveSessionContract {
    /// Create a live session and hold the required balance.
    pub fn deposit(env: Env, session_id: String, price: i128, buyer: Address, token: Address, seller: Address) -> bool {

        // Verify the auth_token that is passed in the request, only the auth_token from an authoritative source should be accepted.
        // TODO: Fetch the amount of tokens from the consumer.
        // TODO: Ensure the consumer has enough tokens to create the live session.
        buyer.require_auth();

        
        // Transfer and hold the tokens in the contract.
        token::Client::new(&env, &token).transfer(&buyer, &env.current_contract_address(), &price);

        // Store the balance for the live session.
        env
            .storage()
            .instance()
            .set(&DataKey::Balance(session_id.clone()), &price);
        
        // TODO: Set the status of the live session to ACTIVE.
        env
            .storage()
            .instance()
            .set(&DataKey::Status(session_id.clone()), &Status::Pending);

        // Ensure the live session has been created.
        let stored_price: i128 = env
            .storage()
            .instance()
            .get(&DataKey::Balance(session_id.clone()))
            .unwrap_or(-1);

        // Ensure the status of the live session is ACTIVE.
        let stored_status: Status = env
            .storage()
            .instance()
            .get(&DataKey::Status(session_id.clone()))
            .unwrap_or(Status::Fail);

        // Return true if everything went well.
        // Check if:
        // -- The balance is set to the price.
        // -- The status is set to ACTIVE.
        stored_price == price && stored_status == Status::Pending
    }
}

mod test;
