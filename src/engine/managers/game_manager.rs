use once_cell::sync::Lazy;
use raylib::ffi::KeyboardKey;
use std::{
    fmt::Debug,
    sync::{RwLock, RwLockReadGuard, RwLockWriteGuard},
    time::{Duration, Instant},
};

pub enum KeyboardAction {
    Pressed,
    Released,
}

impl Debug for KeyboardAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KeyboardAction::Pressed => write!(f, "Pressed"),
            KeyboardAction::Released => write!(f, "Released"),
        }
    }
}

pub struct GameManager {
    pub rng: rand::rngs::ThreadRng,
    pub last_update: Instant,
    pub tick_accumulator: Duration,
    pub delta_time: u128,
    pub in_game: bool,
    pub running: bool,
    pub should_quit: bool,

    pub input_buffer: Vec<(KeyboardKey, KeyboardAction)>,
}

impl GameManager {
    pub fn new() -> Self {
        Self {
            rng: rand::thread_rng(),
            last_update: Instant::now(),
            tick_accumulator: Duration::from_secs(0),
            delta_time: 0,
            in_game: false,
            running: false,
            should_quit: false,
            input_buffer: vec![],
        }
    }
}

unsafe impl Send for GameManager {}
unsafe impl Sync for GameManager {}

pub static GAME_MANAGER: Lazy<RwLock<GameManager>> = Lazy::new(|| RwLock::new(GameManager::new()));

pub fn read_game_manager() -> RwLockReadGuard<'static, GameManager> {
    GAME_MANAGER.read().unwrap()
}

pub fn write_game_manager() -> RwLockWriteGuard<'static, GameManager> {
    GAME_MANAGER.write().unwrap()
}
