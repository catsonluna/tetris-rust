use raylib::{color::Color, prelude::RaylibDraw};

use crate::engine::{
    common::ui,
    lib::RAYLIB_STATE,
    listeners::base::render::render::{get_scaling_factors, scaled_value},
};

pub fn render_create_shape() {
    let mut state = RAYLIB_STATE.lock().unwrap();
    if let Some(ref mut raylib_state) = *state {
        let mut d = raylib_state.rl.begin_drawing(&raylib_state.thread);
        let (scale_x, scale_y) = get_scaling_factors(&d);

        ui::text::text(
            &mut d,
            scaled_value(800, scale_x),
            scaled_value(116, scale_y),
            Color::BLACK,
            "Create Shape".to_string(),
            scaled_value(100, scale_y),
        );





        d.clear_background(Color::from_hex("cfcefc".as_ref()).unwrap());
    }
}
