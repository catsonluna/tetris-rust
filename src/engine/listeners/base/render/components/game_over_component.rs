use raylib::{color::Color, prelude::RaylibDrawHandle};

use crate::engine::{common::ui, listeners::base::render::render::scaled_value};

pub fn render_game_over(d: &mut RaylibDrawHandle, scale_x: f32, scale_y: f32) {
    ui::rectangle::rectangle(
        d,
        scaled_value(480, scale_x),
        scaled_value(335, scale_y),
        scaled_value(160, scale_x),
        scaled_value(160, scale_y),
        Color::WHITE,
        Color::BLACK,
    );

    ui::text::text(
        d,
        scaled_value(480, scale_x),
        scaled_value(275, scale_y),
        Color::BLACK,
        "Game Over".to_string(),
        scaled_value(20, scale_y),
    );

    ui::button::button(
        d,
        scaled_value(140, scale_x),
        scaled_value(30, scale_y),
        scaled_value(480, scale_x),
        scaled_value(310, scale_y),
        Color::GRAY,
        Color::LIGHTBLUE,
        "Restart".to_string(),
        scaled_value(20, scale_y),
        Color::BLACK,
        Color::BLACK,
        false,
        "com.catsonluna.revris.button.restart".to_string(),
    );

    ui::button::button(
        d,
        scaled_value(140, scale_x),
        scaled_value(30, scale_y),
        scaled_value(480, scale_x),
        scaled_value(350, scale_y),
        Color::GRAY,
        Color::LIGHTBLUE,
        "Main Menu".to_string(),
        scaled_value(20, scale_y),
        Color::BLACK,
        Color::BLACK,
        false,
        "com.catsonluna.revris.button.main_menu".to_string(),
    );

    ui::button::button(
        d,
        scaled_value(140, scale_x),
        scaled_value(30, scale_y),
        scaled_value(480, scale_x),
        scaled_value(390, scale_y),
        Color::GRAY,
        Color::LIGHTBLUE,
        "Quit".to_string(),
        scaled_value(20, scale_y),
        Color::BLACK,
        Color::BLACK,
        false,
        "com.catsonluna.revris.button.quit".to_string(),
    );
}
