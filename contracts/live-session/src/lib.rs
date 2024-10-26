#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, String};

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
    pub fn create(env: Env, session_id: String, price: String) -> bool {
        // TODO: Verify the auth_token that is passed in the request, only the auth_token from an authoritative source should be accepted.
        // TODO: Fetch the amount of tokens from the consumer.
        // TODO: Ensure the consumer has enough tokens to create the live session.
        // TODO: Transfer and hold the tokens in the contract.

        // Store the balance for the live session.
        env
            .storage()
            .instance()
            .set(&DataKey::Balance(session_id.clone()), &price);
        
        // TODO: Set the status of the live session to ACTIVE.

        // Ensure the live session has been created.
        let s: String = env
            .storage()
            .instance()
            .get(&DataKey::Balance(session_id.clone()))
            .unwrap_or(String::from_str(&env, "0"));

        // Return true if everything went well.
        // TODO: Check if:
        // -- The balance is set to the price.
        // -- The status is set to ACTIVE.
        s == price
    }
}

mod test;
