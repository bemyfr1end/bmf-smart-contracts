#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env, Vec, String};

#[derive(Clone, PartialEq)]
#[contracttype]
pub enum Status {
    Pending, // This status indicates that the game has not started yet.
    Locked, // This status indicates that the game has started but the money is not claimable yet.
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Game(String),
    Status(String),
}

// #[derive(Clone)]
// #[contracttype]
// pub enum TimeBoundKind {
//     Before,
//     After,
// }

// #[derive(Clone)]
// #[contracttype]
// pub struct TimeBound {
//     pub kind: TimeBoundKind,
//     pub timestamp: u64,
// }

#[derive(Clone)]
#[contracttype]
pub struct Game {
    pub token: Address,
    pub balance: i128,
    pub players: Vec<Address>,
    // pub time_bound: TimeBound,
}

#[contract]
pub struct WagerContract;

// The 'timelock' part: check that provided timestamp is before/after
// the current ledger timestamp.
// fn check_time_bound(env: &Env, time_bound: &TimeBound) -> bool {
//     let ledger_timestamp = env.ledger().timestamp();

//     match time_bound.kind {
//         TimeBoundKind::Before => ledger_timestamp <= time_bound.timestamp,
//         TimeBoundKind::After => ledger_timestamp >= time_bound.timestamp,
//     }
// }

#[contractimpl]
impl WagerContract {
    pub fn deposit(
        env: Env,
        game_id: String,
        from: Address,
        token: Address,
        amount: i128,
    ) {
        // Check that game status is pendind
        let status: Status = env
            .storage()
            .instance()
            .get(&DataKey::Status(game_id.clone()))
            .unwrap_or(Status::Pending);

        if status != Status::Pending {
            panic!("Game cannot accept deposit anymore");
        }

        // Check that game has not more than two players
        let mut game: Game = env
            .storage()
            .instance()
            .get(&DataKey::Game(game_id.clone()))
            .unwrap_or(Game {
                token: token.clone(),
                balance: 0,
                players: Vec::new(&env),
            });

        if game.players.len() >= 2 {
            panic!("too many claimants");
        }

        // ---- Get money from the player and transfer it to the contract
        // Make sure `from` address authorized the deposit call with all the
        // arguments.
        from.require_auth();

        // Transfer token from `from` to this contract address.
        token::Client::new(&env, &token).transfer(&from, &env.current_contract_address(), &amount);

        // Now that the money is in the contract, we can update the game
        game.players.append(&mut Vec::from_slice(&env, &[from.clone()]));
        game.balance += amount;

        // When the second player joins the game, the game is ready to go, we lock it, else we keep it pending
        if game.players.len() == 2 {
            env
                .storage()
                .instance()
                .set(&DataKey::Status(game_id.clone()), &Status::Locked);
        } else {
            env
                .storage()
                .instance()
                .set(&DataKey::Status(game_id.clone()), &Status::Pending);
        }

        // Store all the necessary info to allow one of the claimants to claim it.
        env.storage().instance().set(&DataKey::Game(game_id.clone()), &game);
    }

    pub fn dispatch(env: Env, game_id: String, winner: Address) {
        let status: Status = env
            .storage()
            .instance()
            .get(&DataKey::Status(game_id.clone()))
            .unwrap_or(Status::Pending);

        if status == Status::Pending {
            panic!("Game isn't ready yet, it needs two players");
        }

        let game: Game = env
            .storage()
            .instance()
            .get(&DataKey::Game(game_id.clone()))
            .unwrap();

        if !game.players.contains(&winner) {
            panic!("winner is not a player of this game");
        }

        // Transfer the stored amount of token to winner after passing
        // all the checks.
        token::Client::new(&env, &game.token).transfer(
            &env.current_contract_address(),
            &winner,
            &game.balance,
        );

        // Remove the balance entry to prevent any further claims.
        env.storage().instance().remove(&DataKey::Game(game_id.clone()));
        env.storage().instance().remove(&DataKey::Status(game_id.clone()));
    }
}

mod test;