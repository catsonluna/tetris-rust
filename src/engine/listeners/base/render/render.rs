use raylib::prelude::*;

use crate::engine::managers::game_manager::read_game_manager;

use super::screens::create_shape_screen::render_create_shape;
use super::screens::game_screen::render_game;
use super::screens::main_screen::render_main_menu;
use super::screens::unknown_screen::render_unknown_screen;

// Base resolution as reference
const BASE_WIDTH: i32 = 1600;
const BASE_HEIGHT: i32 = 900;

pub fn on_render() {
    match read_game_manager().screen.as_str() {
        "game" => {
            render_game();
        }
        "main" => {
            render_main_menu();
        }
        "create_shape" => {
            render_create_shape();
        }
        _ => {
            render_unknown_screen();
        }
    }
}

pub fn get_scaling_factors(d: &RaylibDrawHandle) -> (f32, f32) {
    let screen_width = d.get_screen_width() as f32;
    let screen_height = d.get_screen_height() as f32;

    let scale_x = screen_width / BASE_WIDTH as f32;
    let scale_y = screen_height / BASE_HEIGHT as f32;

    (scale_x, scale_y)
}

pub fn scaled_value(value: i32, scale: f32) -> i32 {
    (value as f32 * scale).round() as i32
}
