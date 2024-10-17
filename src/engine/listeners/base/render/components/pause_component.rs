use raylib::{
    color::Color,
    prelude::{RaylibDraw, RaylibDrawHandle},
};

use crate::engine::{common::ui, listeners::base::render::render::scaled_value};

pub fn render_pause_menu(d: &mut RaylibDrawHandle, scale_x: f32, scale_y: f32) {
    // Example for Pause Menu screen scaling
    d.draw_rectangle(
        scaled_value(400, scale_x),
        scaled_value(256, scale_y),
        scaled_value(160, scale_x),
        scaled_value(160, scale_y),
        Color::WHITE,
    );
    d.draw_rectangle_lines(
        scaled_value(400, scale_x),
        scaled_value(256, scale_y),
        scaled_value(160, scale_x),
        scaled_value(160, scale_y),
        Color::BLACK,
    );

    ui::text::text(
        d,
        scaled_value(480, scale_x),
        scaled_value(270, scale_y),
        Color::BLACK,
        "Paused".to_string(),
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
        "Resume".to_string(),
        scaled_value(20, scale_y),
        Color::BLACK,
        Color::BLACK,
        false,
        "com.catsonluna.revris.button.resume".to_string(),
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
