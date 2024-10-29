use raylib::{color::Color, prelude::RaylibDraw};

use crate::engine::{
    common::ui,
    lib::RAYLIB_STATE,
    listeners::base::render::render::{get_scaling_factors, scaled_value}, managers::game_statics::read_game_statics,
};

pub fn render_main_menu() {
    let mut state = RAYLIB_STATE.lock().unwrap();
    if let Some(ref mut raylib_state) = *state {
        let mut d = raylib_state.rl.begin_drawing(&raylib_state.thread);
        let (scale_x, scale_y) = get_scaling_factors(&d);

        ui::text::text(
            &mut d,
            scaled_value(800, scale_x),
            scaled_value(116, scale_y),
            Color::BLACK,
            "Revris".to_string(),
            scaled_value(100, scale_y),
        );

        ui::button::button(
            &mut d,
            scaled_value(115, scale_x),
            scaled_value(30, scale_y),
            scaled_value(800, scale_x),
            scaled_value(300, scale_y),
            Color::WHITE,
            Color::GRAY,
            "Play".to_string(),
            scaled_value(20, scale_y),
            Color::BLACK,
            Color::BLACK,
            false,
            format!("{}.{}", read_game_statics().url, "button.play".to_string()),
        );

        ui::button::button(
            &mut d,
            scaled_value(115, scale_x),
            scaled_value(30, scale_y),
            scaled_value(800, scale_x),
            scaled_value(350, scale_y),
            Color::WHITE,
            Color::GRAY,
            "Blocks".to_string(),
            scaled_value(20, scale_y),
            Color::BLACK,
            Color::BLACK,
            true,
            "".to_string(),
        );

        // disabled settings button
        ui::button::button(
            &mut d,
            scaled_value(115, scale_x),
            scaled_value(30, scale_y),
            scaled_value(800, scale_x),
            scaled_value(400, scale_y),
            Color::WHITE,
            Color::GRAY,
            "Settings".to_string(),
            scaled_value(20, scale_y),
            Color::BLACK,
            Color::BLACK,
            true,
            "".to_string(),
        );

        ui::button::button(
            &mut d,
            scaled_value(115, scale_x),
            scaled_value(30, scale_y),
            scaled_value(800, scale_x),
            scaled_value(450, scale_y),
            Color::WHITE,
            Color::GRAY,
            "Quit".to_string(),
            scaled_value(20, scale_y),
            Color::BLACK,
            Color::BLACK,
            false,
            format!("{}.{}", read_game_statics().url, "button.quit".to_string()),
        );
        d.clear_background(Color::from_hex("cfcefc".as_ref()).unwrap());
    }
}
