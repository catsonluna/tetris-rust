use once_cell::sync::Lazy;
use raylib::color::Color;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

pub struct GameState {
    pub controlling: i32,
    pub arena: Vec<Vec<i32>>,

    pub game_over: bool,

    pub score: i32,
    pub level: i32,
    pub lines_till_next_level: i32,

    pub drop_speed: f32,
    pub drop_ticks: f32,
    pub colors: Vec<(i32, Color)>,
    pub ground_ticks: i32,

    pub left_hold: i32,
    pub right_hold: i32,
    pub down_hold: i32,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            arena: vec![vec![0; 21]; 40],
            controlling: 0,
            drop_speed: 1.0,
            drop_ticks: 0.0,
            ground_ticks: 0,
            colors: vec![],
            left_hold: 0,
            right_hold: 0,
            down_hold: 0,
            score: 0,
            level: 1,
            lines_till_next_level: 10,
            game_over: false,
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
