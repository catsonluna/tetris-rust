use once_cell::sync::Lazy;
use raylib::{color::Color, ffi::KeyboardKey};
use std::{
    fmt::Debug,
    sync::{RwLock, RwLockReadGuard, RwLockWriteGuard},
    time::{Duration, Instant},
};
#[derive(PartialEq)] // Add the PartialEq trait
pub enum KeyboardAction {
    Pressed,
    Released,
}

pub struct Block {
    pub layout: Vec<Vec<i32>>,
    pub can_rotate: bool,
    pub color: Color,
    pub name: String,
}

impl Clone for Block {
    fn clone(&self) -> Self {
        Self {
            layout: self.layout.clone(),
            can_rotate: self.can_rotate,
            color: self.color,
            name: self.name.clone(),
        }
    }
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

    pub pieces: Vec<Block>,
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
            pieces: vec![
                Block {
                    layout: vec![
                        vec![0, 0, 0, 0, 0],
                        vec![0, 1, 1, 0, 0],
                        vec![0, 1, 1, 0, 0],
                        vec![0, 0, 0, 0, 0],
                        vec![0, 0, 0, 0, 0],
                    ],
                    can_rotate: false,
                    color: Color::FIREBRICK,
                    name: "Small Block".to_string(),
                },
                Block {
                    layout: vec![
                        vec![0, 0, 0, 0, 0],
                        vec![0, 1, 1, 1, 0],
                        vec![0, 1, 1, 1, 0],
                        vec![0, 1, 1, 1, 0],
                        vec![0, 0, 0, 0, 0],
                    ],
                    can_rotate: false,
                    color: Color::RED,
                    name: "Medium Block".to_string(),
                },
                Block {
                    layout: vec![
                        vec![1, 1, 1, 1, 1],
                        vec![1, 1, 1, 1, 1],
                        vec![1, 1, 1, 1, 1],
                        vec![1, 1, 1, 1, 1],
                        vec![1, 1, 1, 1, 1],
                    ],
                    can_rotate: false,
                    color: Color::DARKRED,
                    name: "Large Block".to_string(),
                },
                Block {
                    layout: vec![
                        vec![0, 0, 0, 0, 0],
                        vec![0, 0, 1, 0, 0],
                        vec![0, 1, 1, 1, 0],
                        vec![0, 0, 0, 0, 0],
                        vec![0, 0, 0, 0, 0],
                    ],
                    can_rotate: true,
                    color: Color::BLUE,
                    name: "Small T".to_string(),
                },
                Block {
                    layout: vec![
                        vec![0, 0, 0, 0, 0],
                        vec![0, 1, 1, 1, 0],
                        vec![0, 0, 1, 0, 0],
                        vec![0, 0, 1, 0, 0],
                        vec![0, 0, 0, 0, 0],
                    ],
                    can_rotate: true,
                    color: Color::DARKBLUE,
                    name: "BIG T".to_string(),
                },
                Block {
                    layout: vec![
                        vec![0, 0, 1, 0, 0],
                        vec![0, 0, 1, 0, 0],
                        vec![0, 0, 1, 0, 0],
                        vec![0, 0, 1, 1, 0],
                        vec![0, 0, 0, 0, 0],
                    ],
                    can_rotate: true,
                    color: Color::GREEN,
                    name: "L".to_string(),
                },
                Block {
                    name: "J".to_string(),
                    layout: vec![
                        vec![0, 0, 1, 0, 0],
                        vec![0, 0, 1, 0, 0],
                        vec![0, 0, 1, 0, 0],
                        vec![0, 1, 1, 0, 0],
                        vec![0, 0, 0, 0, 0],
                    ],
                    can_rotate: true,
                    color: Color::DARKGREEN,
                },
                Block {
                    name: "Pyramid".to_string(),
                    layout: vec![
                        vec![0, 0, 0, 0, 0],
                        vec![0, 0, 1, 0, 0],
                        vec![0, 1, 1, 1, 0],
                        vec![1, 1, 1, 1, 1],
                        vec![0, 0, 0, 0, 0],
                    ],
                    can_rotate: true,
                    color: Color::PURPLE,
                },
            ],
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
