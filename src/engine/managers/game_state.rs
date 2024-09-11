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

    pub left_hold: ActionManager,
    pub right_hold: ActionManager,
    pub down_hold: ActionManager,
}

pub struct ActionManager {
    pub is_pressed: bool,
    pub move_ticks: i32,
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
            score: 0,
            level: 1,
            lines_till_next_level: 10,
            game_over: false,

            left_hold: ActionManager {
                is_pressed: false,
                move_ticks: 0,
            },

            right_hold: ActionManager {
                is_pressed: false,
                move_ticks: 0,
            },

            down_hold: ActionManager {
                is_pressed: false,
                move_ticks: 0,
            },
        }
    }
    pub fn reset(&mut self) {
        self.arena = vec![vec![0; 21]; 40];
        self.controlling = 0;
        self.drop_speed = 1.0;
        self.drop_ticks = 0.0;
        self.ground_ticks = 0;
        self.colors = vec![];
        self.score = 0;
        self.level = 1;
        self.lines_till_next_level = 10;
        self.game_over = false;

        self.left_hold = ActionManager {
            is_pressed: false,
            move_ticks: 0,
        };

        self.right_hold = ActionManager {
            is_pressed: false,
            move_ticks: 0,
        };

        self.down_hold = ActionManager {
            is_pressed: false,
            move_ticks: 0,
        };
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
