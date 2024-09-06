use once_cell::sync::Lazy;
use std::{
    sync::{RwLock, RwLockReadGuard, RwLockWriteGuard},
    time::{Duration, Instant},
};

pub struct GameState {
    pub last_update: Instant,
    pub tick_accumulator: Duration,
    pub arena: Vec<Vec<i32>>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            last_update: Instant::now(),
            tick_accumulator: Duration::from_secs(0),
            arena: vec![vec![0; 21]; 40],
        }
    }
}

pub static GAME_STATE: Lazy<RwLock<GameState>> = Lazy::new(|| RwLock::new(GameState::new()));

pub fn read_game_state() -> RwLockReadGuard<'static, GameState> {
    GAME_STATE.read().unwrap()
}

pub fn write_game_state() -> RwLockWriteGuard<'static, GameState> {
    GAME_STATE.write().unwrap()
}
