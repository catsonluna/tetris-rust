use once_cell::sync::Lazy;
use std::sync::{RwLock, RwLockReadGuard};

pub struct GameStatics {
    pub tick_rate: i32,
    pub block_size: i32,
}

impl GameStatics {
    pub fn new() -> Self {
        Self {
            tick_rate: 1,
            block_size: 16,
        }
    }
}

pub static GAME_STATICS: Lazy<RwLock<GameStatics>> = Lazy::new(|| RwLock::new(GameStatics::new()));

pub fn read_game_statics() -> RwLockReadGuard<'static, GameStatics> {
    GAME_STATICS.read().unwrap()
}
