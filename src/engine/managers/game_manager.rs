use arc_swap::ArcSwap;
use chrono::{DateTime, Utc};
use lazy_static::lazy_static;

use raylib::{color::Color, ffi::KeyboardKey};
use std::{
    fmt::Debug,
    sync::Arc,
    time::{Duration, Instant},
};

use serde::{Deserialize, Serialize};

#[derive(PartialEq)] // Add the PartialEq trait
pub enum KeyboardAction {
    Pressed,
    Released,
}
#[derive(Debug)]
pub struct Block {
    pub layout: Vec<Vec<i32>>,
    pub can_rotate: bool,
    pub color: Color,
    pub name: String,
    pub active: bool,
}

impl Block {
    pub fn new() -> Self {
        Self {
            layout: vec![],
            can_rotate: false,
            color: Color::WHITE,
            name: "".to_string(),
            active: false,
        }
    }
}

impl Default for Block {
    fn default() -> Self {
        Self::new()
    }
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

impl Clone for SaveData {
    fn clone(&self) -> Self {
        Self {
            check_sum: self.check_sum,
            best_game: self.best_game.clone(),
            history: self.history.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameData {
    pub id: i32,
    pub score: i32,
    pub level: i32,
    pub lines_cleared: i32,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
}

impl GameData {
    pub fn new() -> Self {
        Self {
            id: 0,
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
            id: self.id,
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
            active: self.active,
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

impl Clone for KeyboardAction {
    fn clone(&self) -> Self {
        match self {
            KeyboardAction::Pressed => KeyboardAction::Pressed,
            KeyboardAction::Released => KeyboardAction::Released,
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

    pub screen: String,
    pub screen_path: Vec<String>,

    pub custom_block: Block,
}

impl GameManager {
    pub fn new() -> Self {
        Self {
            screen: "main".to_string(),
            screen_path: vec![],
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
                    active: true,
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
                    active: true,
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
                    active: true,
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
                    active: true,
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
                    active: true,
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
                    active: true,
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
                    active: true,
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
                    active: true,
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
                    active: true,
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
                    active: true,
                },
            ],
            app_start_time: Instant::now(),
            save_data: SaveData::new(),
            custom_block: Block::new(),
        }
    }
}

unsafe impl Send for GameManager {}
unsafe impl Sync for GameManager {}

impl Clone for GameManager {
    fn clone(&self) -> Self {
        GameManager {
            screen: self.screen.clone(),
            screen_path: self.screen_path.clone(),
            rng: rand::thread_rng(), // The RNG can't be cloned directly; reinitialize
            last_update: self.last_update,
            tick_accumulator: self.tick_accumulator,
            delta_time: self.delta_time,
            in_game: self.in_game,
            running: self.running,
            should_quit: self.should_quit,
            input_buffer: self.input_buffer.clone(),
            pieces: self.pieces.clone(),
            app_start_time: self.app_start_time,
            save_data: self.save_data.clone(),
            custom_block: self.custom_block.clone(),
        }
    }
}

lazy_static! {
    static ref GAME_MANAGER: ArcSwap<GameManager> = ArcSwap::new(Arc::new(GameManager::new()));
}

pub fn read_game_manager() -> Arc<GameManager> {
    GAME_MANAGER.load_full()
}

pub fn read_game_manager_only() -> GameManager {
    (**GAME_MANAGER.load()).clone()
}

pub fn write_game_manager(game_manager: GameManager) {
    GAME_MANAGER.store(Arc::new(game_manager));
}

// make a function for storing each of the game manager fields
pub fn write_game_manager_running(running: bool) {
    let mut game_manager = read_game_manager_only();
    game_manager.running = running;
    GAME_MANAGER.store(Arc::new(game_manager));
}

pub fn write_game_manager_in_game(in_game: bool) {
    let mut game_manager = read_game_manager_only();
    game_manager.in_game = in_game;
    GAME_MANAGER.store(Arc::new(game_manager));
}

pub fn write_game_manager_should_quit(should_quit: bool) {
    let mut game_manager = read_game_manager_only();
    game_manager.should_quit = should_quit;
    GAME_MANAGER.store(Arc::new(game_manager));
}

pub fn write_game_manager_input_buffer(input_buffer: Vec<(KeyboardKey, KeyboardAction)>) {
    let mut game_manager = read_game_manager_only();
    game_manager.input_buffer = input_buffer;
    GAME_MANAGER.store(Arc::new(game_manager));
}

pub fn write_game_manager_pieces(pieces: Vec<Block>) {
    let mut game_manager = read_game_manager_only();
    game_manager.pieces = pieces;
    GAME_MANAGER.store(Arc::new(game_manager));
}

pub fn write_game_manager_save_data(save_data: SaveData) {
    let mut game_manager = read_game_manager_only();
    game_manager.save_data = save_data;
    GAME_MANAGER.store(Arc::new(game_manager));
}

pub fn write_game_manager_last_update(last_update: Instant) {
    let mut game_manager = read_game_manager_only();
    game_manager.last_update = last_update;
    GAME_MANAGER.store(Arc::new(game_manager));
}

pub fn write_game_manager_tick_accumulator(tick_accumulator: Duration) {
    let mut game_manager = read_game_manager_only();
    game_manager.tick_accumulator = tick_accumulator;
    GAME_MANAGER.store(Arc::new(game_manager));
}

pub fn write_game_manager_delta_time(delta_time: u128) {
    let mut game_manager = read_game_manager_only();
    game_manager.delta_time = delta_time;
    GAME_MANAGER.store(Arc::new(game_manager));
}

pub fn write_game_manager_app_start_time(app_start_time: Instant) {
    let mut game_manager = read_game_manager_only();
    game_manager.app_start_time = app_start_time;
    GAME_MANAGER.store(Arc::new(game_manager));
}

pub fn write_game_manager_screen(screen: String) {
    let mut game_manager = read_game_manager_only();
    game_manager.screen = screen;
    GAME_MANAGER.store(Arc::new(game_manager));
}

pub fn write_game_manager_screen_path(screen_path: Vec<String>) {
    let mut game_manager = read_game_manager_only();
    game_manager.screen_path = screen_path;
    GAME_MANAGER.store(Arc::new(game_manager));
}

pub fn write_game_manager_custom_block(custom_blocks: Block) {
    let mut game_manager = read_game_manager_only();
    game_manager.custom_block = custom_blocks;
    GAME_MANAGER.store(Arc::new(game_manager));
}