use chrono::{DateTime, Utc};
use once_cell::sync::Lazy;
use raylib::{color::Color, ffi::KeyboardKey};
use std::{
    fmt::Debug,
    sync::{RwLock, RwLockReadGuard, RwLockWriteGuard},
    time::{Duration, Instant},
};

use serde::{Deserialize, Serialize};

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


#[derive(Serialize, Deserialize, Debug)]
pub struct SaveData {
    pub check_sum: i32,
    pub best_game: GameData,
    pub history: Vec<GameData>,
}

impl SaveData {
    pub fn new() -> Self {
        Self {
            check_sum: 0,
            best_game: GameData::new(),
            history: vec![],
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameData {
    pub score: i32,
    pub level: i32,
    pub lines_cleared: i32,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
}

impl GameData {
    pub fn new() -> Self {
        Self {
            score: 0,
            level: 1,
            lines_cleared: 0,
            start_time: Utc::now(),
            end_time: Utc::now(),
        }
    }
    
}

impl Clone for GameData {
    fn clone(&self) -> Self {
        Self {
            score: self.score,
            level: self.level,
            lines_cleared: self.lines_cleared,
            start_time: self.start_time,
            end_time: self.end_time,
        }
    }
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

    pub app_start_time: Instant,

    pub save_data: SaveData,
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
                    layout: vec![
                        vec![0, 0, 0, 0, 0],
                        vec![0, 0, 1, 0, 0],
                        vec![0, 0, 1, 0, 0],
                        vec![0, 0, 1, 1, 0],
                        vec![0, 0, 0, 0, 0],
                    ],
                    can_rotate: true,
                    color: Color::GREEN,
                    name: "Small L".to_string(),
                },
                Block {
                    name: "Small J".to_string(),
                    layout: vec![
                        vec![0, 0, 0, 0, 0],
                        vec![0, 0, 1, 0, 0],
                        vec![0, 0, 1, 0, 0],
                        vec![0, 1, 1, 0, 0],
                        vec![0, 0, 0, 0, 0],
                    ],
                    can_rotate: true,
                    color: Color::DARKGREEN,
                },
                Block {
                    name: "I".to_string(),
                    layout: vec![
                        vec![0, 0, 1, 0, 0],
                        vec![0, 0, 1, 0, 0],
                        vec![0, 0, 1, 0, 0],
                        vec![0, 0, 1, 0, 0],
                        vec![0, 0, 1, 0, 0],
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
            app_start_time: Instant::now(),
            save_data: SaveData::new(),
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
