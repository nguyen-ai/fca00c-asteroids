#![no_std]

use engine::{Client as GameEngine, Direction};
use soroban_sdk::{contractimpl, BytesN, Env};

pub struct Solution;

mod engine {
    soroban_sdk::contractimport!(file = "../game_engine.wasm");
}

mod test;

#[contractimpl]
impl Solution {
    pub fn solve(env: Env, engine_id: BytesN<32>) {
        let engine = GameEngine::new(&env, &engine_id);

        // YOUR CODE START
        engine.p_shoot();
        engine.p_turn(&Direction::Left);
        engine.p_move(&Option::Some(2));
        engine.p_turn(&Direction::DownLeft);
        engine.p_move(&Option::Some(1));
        engine.p_shoot();
        engine.p_turn(&Direction::Left);
        engine.p_move(&Option::Some(1));
        engine.p_harvest();
        engine.p_turn(&Direction::DownRight);
        engine.p_move(&Option::Some(6));
        engine.p_turn(&Direction::Right);
        engine.p_move(&Option::Some(4));
        engine.p_harvest();
        engine.p_turn(&Direction::Up);
        engine.p_move(&Option::Some(1));
        engine.p_turn(&Direction::DownRight);
        engine.p_shoot();
        engine.p_turn(&Direction::Up);
        engine.p_move(&Option::Some(4));
        engine.p_shoot();
        engine.p_turn(&Direction::UpLeft);
        engine.p_move(&Option::Some(2));
        engine.p_turn(&Direction::UpRight);
        engine.p_shoot();
        engine.p_upgrade();
        engine.p_turn(&Direction::Up);
        engine.p_move(&Option::Some(6));
        engine.p_turn(&Direction::Left);
        engine.p_shoot();
        engine.p_turn(&Direction::Up);
        engine.p_move(&Option::Some(4));

        engine.p_shoot();
        engine.p_turn(&Direction::UpLeft);
        engine.p_move(&Option::Some(5));
        engine.p_turn(&Direction::Left);
        engine.p_move(&Option::Some(3));
        engine.p_shoot();
        engine.p_turn(&Direction::UpRight);
        engine.p_move(&Option::Some(4));
        engine.p_turn(&Direction::DownRight);
        engine.p_shoot();
        engine.p_turn(&Direction::UpRight);
        engine.p_move(&Option::Some(1));
        engine.p_turn(&Direction::Right);
        engine.p_move(&Option::Some(6));
        engine.p_shoot();
        engine.p_move(&Option::Some(3));

        engine.p_turn(&Direction::UpLeft);
        engine.p_shoot();
        engine.p_turn(&Direction::UpRight);
        engine.p_move(&Option::Some(2));
        engine.p_turn(&Direction::UpLeft);
        engine.p_shoot();
        engine.p_turn(&Direction::Right);
        engine.p_move(&Option::Some(1));
        engine.p_turn(&Direction::DownRight);
        engine.p_shoot();
        engine.p_turn(&Direction::Down);
        engine.p_move(&Option::Some(8));
        engine.p_shoot();
        engine.p_turn(&Direction::Right);
        engine.p_move(&Option::Some(2));
        engine.p_turn(&Direction::DownRight);
        engine.p_move(&Option::Some(4));
        engine.p_harvest();
        engine.p_move(&Option::Some(1));
        engine.p_harvest();
        engine.p_move(&Option::Some(2));

        engine.p_shoot();
        engine.p_turn(&Direction::Down);
        engine.p_move(&Option::Some(5));
        engine.p_turn(&Direction::DownLeft);
        engine.p_shoot();
        engine.p_turn(&Direction::Left);
        engine.p_move(&Option::Some(8));
        engine.p_turn(&Direction::Down);
        engine.p_shoot();
        engine.p_move(&Option::Some(2));
        engine.p_turn(&Direction::DownLeft);
        engine.p_move(&Option::Some(1));
        engine.p_shoot();
        engine.p_turn(&Direction::Down);
        engine.p_move(&Option::Some(8));

        engine.p_turn(&Direction::DownRight);
        engine.p_shoot();
        engine.p_move(&Option::Some(5));
        engine.p_turn(&Direction::Right);
        engine.p_move(&Option::Some(4));
        engine.p_turn(&Direction::UpRight);
        engine.p_shoot();
        engine.p_turn(&Direction::Down);
        engine.p_move(&Option::Some(2));
        engine.p_turn(&Direction::DownRight);
        engine.p_shoot();
        engine.p_turn(&Direction::DownLeft);
        engine.p_move(&Option::Some(2));
        engine.p_turn(&Direction::Down);
        engine.p_shoot();
        engine.p_turn(&Direction::Left);
        engine.p_move(&Option::Some(5));
        engine.p_turn(&Direction::UpLeft);
        engine.p_shoot();
        engine.p_move(&Option::Some(5));
        engine.p_turn(&Direction::Left);
        engine.p_move(&Option::Some(2));

        engine.p_shoot();
        engine.p_move(&Option::Some(4));
        engine.p_turn(&Direction::DownLeft);
        engine.p_shoot();
        engine.p_turn(&Direction::Left);
        engine.p_move(&Option::Some(6));
        engine.p_turn(&Direction::DownLeft);
        engine.p_shoot();
        engine.p_turn(&Direction::Left);
        engine.p_move(&Option::Some(3));
        engine.p_turn(&Direction::UpRight);
        engine.p_shoot();
        engine.p_turn(&Direction::DownLeft);
        engine.p_move(&Option::Some(3));
        engine.p_turn(&Direction::Down);
        engine.p_shoot();
        engine.p_turn(&Direction::Left);
        engine.p_move(&Option::Some(1));

        engine.p_harvest();
        engine.p_turn(&Direction::Up);
        engine.p_shoot();
        engine.p_move(&Option::Some(4));
        engine.p_shoot();
        engine.p_turn(&Direction::DownLeft);
        engine.p_move(&Option::Some(6));
        engine.p_turn(&Direction::Up);
        engine.p_shoot();
        engine.p_turn(&Direction::Down);
        engine.p_move(&Option::Some(7));
        engine.p_shoot();

        engine.p_move(&Option::Some(4));
        engine.p_shoot();
        engine.p_turn(&Direction::UpLeft);
        engine.p_move(&Option::Some(3));
        engine.p_turn(&Direction::Left);
        engine.p_shoot();
        engine.p_move(&Option::Some(4));
        engine.p_shoot();

        engine.p_turn(&Direction::Up);
        engine.p_move(&Option::Some(4));
        engine.p_turn(&Direction::UpRight);
        engine.p_shoot();
        engine.p_turn(&Direction::UpLeft);
        engine.p_shoot();
        engine.p_move(&Option::Some(4));
        engine.p_turn(&Direction::Left);
        engine.p_move(&Option::Some(4));

        engine.p_shoot();
        engine.p_turn(&Direction::UpLeft);
        engine.p_move(&Option::Some(6));
        engine.p_turn(&Direction::UpRight);
        engine.p_shoot();
        engine.p_turn(&Direction::DownLeft);
        engine.p_shoot();
        engine.p_move(&Option::Some(4));
        engine.p_turn(&Direction::DownRight);
        engine.p_shoot();
        engine.p_turn(&Direction::Down);
        engine.p_move(&Option::Some(7));
        engine.p_shoot();
        engine.p_move(&Option::Some(4));

        engine.p_turn(&Direction::Right);
        engine.p_shoot();
        engine.p_turn(&Direction::Down);
        engine.p_move(&Option::Some(2));
        engine.p_turn(&Direction::DownLeft);
        engine.p_shoot();
        engine.p_move(&Option::Some(4));
        engine.p_turn(&Direction::Left);
        engine.p_move(&Option::Some(5));
        engine.p_turn(&Direction::DownLeft);
        engine.p_shoot();
        engine.p_turn(&Direction::UpRight);
        engine.p_shoot();
        engine.p_turn(&Direction::UpLeft);
        engine.p_move(&Option::Some(4));
        engine.p_turn(&Direction::Down);
        engine.p_shoot();
        engine.p_turn(&Direction::Right);
        engine.p_shoot();

        engine.p_turn(&Direction::Up);
        engine.p_move(&Option::Some(4));
        engine.p_shoot();
        engine.p_turn(&Direction::UpRight);
        engine.p_move(&Option::Some(3));
        engine.p_turn(&Direction::DownRight);
        engine.p_shoot();
        engine.p_turn(&Direction::UpLeft);
        engine.p_move(&Option::Some(3));
        engine.p_shoot();
        engine.p_turn(&Direction::Up);
        engine.p_move(&Option::Some(6));
        engine.p_turn(&Direction::UpLeft);
        engine.p_shoot();

        engine.p_move(&Option::Some(6));
        engine.p_turn(&Direction::UpRight);
        engine.p_shoot();
        engine.p_move(&Option::Some(6));
        engine.p_turn(&Direction::Right);
        engine.p_move(&Option::Some(3));
        engine.p_shoot();
        engine.p_turn(&Direction::DownRight);
        engine.p_shoot();
        engine.p_turn(&Direction::Up);
        engine.p_move(&Option::Some(4));
        engine.p_harvest();
        engine.p_turn(&Direction::UpRight);
        engine.p_shoot();
        engine.p_move(&Option::Some(2));
        engine.p_turn(&Direction::UpLeft);
        engine.p_shoot();

        engine.p_turn(&Direction::Up);
        engine.p_move(&Option::Some(4));
        engine.p_turn(&Direction::UpRight);
        engine.p_shoot();
        engine.p_turn(&Direction::Up);
        engine.p_move(&Option::Some(1));
        engine.p_turn(&Direction::UpLeft);
        engine.p_shoot();
        engine.p_turn(&Direction::Left);
        engine.p_shoot();

        engine.p_turn(&Direction::UpRight);
        engine.p_move(&Option::Some(6));
        engine.p_turn(&Direction::Up);
        engine.p_shoot();
        engine.p_turn(&Direction::Right);
        engine.p_move(&Option::Some(2));
        engine.p_turn(&Direction::DownRight);
        engine.p_move(&Option::Some(2));
        engine.p_turn(&Direction::Down);
        engine.p_shoot();
        engine.p_move(&Option::Some(5));
        engine.p_turn(&Direction::Right);
        engine.p_shoot();

        engine.p_turn(&Direction::DownRight);
        engine.p_move(&Option::Some(2));
        engine.p_turn(&Direction::Right);
        engine.p_shoot();
        engine.p_turn(&Direction::DownRight);
        engine.p_move(&Option::Some(1));
        engine.p_turn(&Direction::Down);
        engine.p_shoot();
        engine.p_turn(&Direction::Left);
        engine.p_move(&Option::Some(1));
        engine.p_turn(&Direction::Down);
        engine.p_move(&Option::Some(5));
        engine.p_shoot();
        engine.p_turn(&Direction::DownRight);
        engine.p_move(&Option::Some(8));
        engine.p_turn(&Direction::Right);
        engine.p_shoot();
        engine.p_turn(&Direction::UpRight);
        engine.p_move(&Option::Some(4));

        engine.p_turn(&Direction::Right);
        engine.p_shoot();
        engine.p_turn(&Direction::Up);
        engine.p_shoot();
        engine.p_turn(&Direction::UpRight);
        engine.p_move(&Option::Some(8));
        engine.p_turn(&Direction::Down);
        engine.p_shoot();
        engine.p_turn(&Direction::UpLeft);
        engine.p_move(&Option::Some(2));
        engine.p_turn(&Direction::Left);
        engine.p_move(&Option::Some(1));
        engine.p_turn(&Direction::DownLeft);
        engine.p_shoot();

        engine.p_turn(&Direction::UpLeft);
        engine.p_move(&Option::Some(5));
        engine.p_turn(&Direction::Left);
        engine.p_shoot();
        engine.p_turn(&Direction::UpRight);
        engine.p_move(&Option::Some(2));
        engine.p_shoot();
        engine.p_turn(&Direction::DownRight);
        engine.p_shoot();
        engine.p_turn(&Direction::Up);
        engine.p_move(&Option::Some(5));
        engine.p_turn(&Direction::UpLeft);
        engine.p_shoot();
        engine.p_turn(&Direction::UpRight);
        engine.p_move(&Option::Some(2));
        engine.p_turn(&Direction::Left);
        engine.p_shoot();
        engine.p_turn(&Direction::Right);
        engine.p_move(&Option::Some(4));
        engine.p_shoot();
        engine.p_turn(&Direction::DownRight);
        engine.p_shoot();

        engine.p_turn(&Direction::Up);
        engine.p_move(&Option::Some(7));
        engine.p_turn(&Direction::Left);
        engine.p_shoot();
        engine.p_turn(&Direction::Right);
        engine.p_move(&Option::Some(4));
        engine.p_shoot();
        engine.p_turn(&Direction::Up);
        engine.p_shoot();
        engine.p_turn(&Direction::Right);
        engine.p_move(&Option::Some(7));
        engine.p_shoot();
        engine.p_turn(&Direction::Up);
        engine.p_move(&Option::Some(1));
        engine.p_shoot();
        engine.p_turn(&Direction::Right);
        engine.p_move(&Option::Some(7));
        engine.p_shoot();
        engine.p_turn(&Direction::UpRight);
        engine.p_move(&Option::Some(4));
        engine.p_turn(&Direction::DownRight);
        engine.p_shoot();

        engine.p_turn(&Direction::UpRight);
        engine.p_move(&Option::Some(6));
        engine.p_turn(&Direction::Up);
        engine.p_shoot();
        engine.p_turn(&Direction::Up);
        engine.p_move(&Option::Some(7));
        engine.p_turn(&Direction::DownLeft);
        engine.p_shoot();
        engine.p_turn(&Direction::UpRight);
        engine.p_move(&Option::Some(1));
        engine.p_shoot();
        engine.p_turn(&Direction::UpLeft);
        engine.p_shoot();

        // YOUR CODE END
    }
}
