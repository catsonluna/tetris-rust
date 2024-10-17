use super::game_manager::{Block, GameData};
use arc_swap::ArcSwap;
use lazy_static::lazy_static;
use raylib::color::Color;
use std::sync::Arc;

pub struct GameState {
    pub controlling: i32,
    pub arena: Vec<Vec<i32>>,
    pub current_piece: Block,
    pub current_center: (usize, usize),
    pub all_pieces: Vec<(i32, Block)>,

    pub held_piece: Block,
    pub has_held: bool,

    pub game_over: bool,
    pub game_data: GameData,
    pub lines_till_next_level: i32,

    pub drop_speed: f32,
    pub drop_ticks: f32,
    pub ground_ticks: i32,

    pub left_hold: ActionManager,
    pub right_hold: ActionManager,
    pub down_hold: ActionManager,

    pub piece_queue: Vec<Block>,
}

pub struct ActionManager {
    pub is_pressed: bool,
    pub move_ticks: i32,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            arena: vec![vec![0; 21]; 41],
            controlling: 0,
            drop_speed: 1.0,
            drop_ticks: 0.0,
            ground_ticks: 0,

            game_data: GameData::new(),

            lines_till_next_level: 6,
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

            current_piece: Block {
                layout: vec![],
                can_rotate: false,
                color: Color::WHITE,
                name: "".to_string(),
            },
            current_center: (0, 0),
            held_piece: Block {
                layout: vec![],
                can_rotate: false,
                color: Color::WHITE,
                name: "".to_string(),
            },
            has_held: false,
            all_pieces: vec![],
            piece_queue: vec![],
        }
    }
}

unsafe impl Send for GameState {}
unsafe impl Sync for GameState {}

impl Clone for GameState {
    fn clone(&self) -> Self {
        Self {
            arena: self.arena.clone(),
            controlling: self.controlling,
            drop_speed: self.drop_speed,
            drop_ticks: self.drop_ticks,
            ground_ticks: self.ground_ticks,
            game_data: self.game_data.clone(),
            lines_till_next_level: self.lines_till_next_level,
            game_over: self.game_over,
            left_hold: self.left_hold.clone(),
            right_hold: self.right_hold.clone(),
            down_hold: self.down_hold.clone(),
            current_piece: self.current_piece.clone(),
            current_center: self.current_center,
            held_piece: self.held_piece.clone(),
            has_held: self.has_held,
            all_pieces: self.all_pieces.clone(),
            piece_queue: self.piece_queue.clone(),
        }
    }
}

impl Clone for ActionManager {
    fn clone(&self) -> Self {
        Self {
            is_pressed: self.is_pressed,
            move_ticks: self.move_ticks,
        }
    }
}

lazy_static! {
    static ref GAME_MANAGER: ArcSwap<GameState> = ArcSwap::new(Arc::new(GameState::new()));
}

pub fn read_game_state() -> Arc<GameState> {
    GAME_MANAGER.load_full()
}

pub fn read_game_state_only() -> GameState {
    (**GAME_MANAGER.load()).clone()
}

pub fn write_game_state(game_manager: GameState) {
    GAME_MANAGER.store(Arc::new(game_manager));
}

pub fn write_game_state_arena(arena: Vec<Vec<i32>>) {
    let mut game_manager = read_game_state_only();
    game_manager.arena = arena;
    write_game_state(game_manager);
}

pub fn write_game_state_controlling(controlling: i32) {
    let mut game_manager = read_game_state_only();
    game_manager.controlling = controlling;
    write_game_state(game_manager);
}

pub fn write_game_state_drop_speed(drop_speed: f32) {
    let mut game_manager = read_game_state_only();
    game_manager.drop_speed = drop_speed;
    write_game_state(game_manager);
}

pub fn write_game_state_drop_ticks(drop_ticks: f32) {
    let mut game_manager = read_game_state_only();
    game_manager.drop_ticks = drop_ticks;
    write_game_state(game_manager);
}

pub fn write_game_state_ground_ticks(ground_ticks: i32) {
    let mut game_manager = read_game_state_only();
    game_manager.ground_ticks = ground_ticks;
    write_game_state(game_manager);
}

pub fn write_game_state_game_data(game_data: GameData) {
    let mut game_manager = read_game_state_only();
    game_manager.game_data = game_data;
    write_game_state(game_manager);
}

pub fn write_game_state_lines_till_next_level(lines_till_next_level: i32) {
    let mut game_manager = read_game_state_only();
    game_manager.lines_till_next_level = lines_till_next_level;
    write_game_state(game_manager);
}

pub fn write_game_state_game_over(game_over: bool) {
    let mut game_manager = read_game_state_only();
    game_manager.game_over = game_over;
    write_game_state(game_manager);
}

pub fn write_game_state_left_hold(left_hold: ActionManager) {
    let mut game_manager = read_game_state_only();
    game_manager.left_hold = left_hold;
    write_game_state(game_manager);
}

pub fn write_game_state_right_hold(right_hold: ActionManager) {
    let mut game_manager = read_game_state_only();
    game_manager.right_hold = right_hold;
    write_game_state(game_manager);
}

pub fn write_game_state_down_hold(down_hold: ActionManager) {
    let mut game_manager = read_game_state_only();
    game_manager.down_hold = down_hold;
    write_game_state(game_manager);
}

pub fn write_game_state_current_piece(current_piece: Block) {
    let mut game_manager = read_game_state_only();
    game_manager.current_piece = current_piece;
    write_game_state(game_manager);
}

pub fn write_game_state_current_center(current_center: (usize, usize)) {
    let mut game_manager = read_game_state_only();
    game_manager.current_center = current_center;
    write_game_state(game_manager);
}

pub fn write_game_state_held_piece(held_piece: Block) {
    let mut game_manager = read_game_state_only();
    game_manager.held_piece = held_piece;
    write_game_state(game_manager);
}

pub fn write_game_state_has_held(has_held: bool) {
    let mut game_manager = read_game_state_only();
    game_manager.has_held = has_held;
    write_game_state(game_manager);
}

pub fn write_game_state_all_pieces(all_pieces: Vec<(i32, Block)>) {
    let mut game_manager = read_game_state_only();
    game_manager.all_pieces = all_pieces;
    write_game_state(game_manager);
}

pub fn write_game_state_piece_queue(piece_queue: Vec<Block>) {
    let mut game_manager = read_game_state_only();
    game_manager.piece_queue = piece_queue;
    write_game_state(game_manager);
}
