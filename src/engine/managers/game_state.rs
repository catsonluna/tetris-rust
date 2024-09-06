use once_cell::sync::Lazy;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

pub struct GameState {
    pub controlling: i32,
    pub arena: Vec<Vec<i32>>,
    pub drop_speed: f32,
    pub drop_ticks: f32,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            arena: vec![vec![0; 21]; 40],
            controlling: 0,
            drop_speed: 1.0,
            drop_ticks: 0.0,
        }
    }
}

unsafe impl Send for GameState {}
unsafe impl Sync for GameState {}

pub static GAME_STATE: Lazy<RwLock<GameState>> = Lazy::new(|| RwLock::new(GameState::new()));

pub fn read_game_state() -> RwLockReadGuard<'static, GameState> {
    GAME_STATE.read().unwrap()
}

pub fn write_game_state() -> RwLockWriteGuard<'static, GameState> {
    GAME_STATE.write().unwrap()
}
