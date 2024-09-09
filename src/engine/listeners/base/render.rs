use crate::engine::{lib::RAYLIB_STATE, managers::game_state::read_game_state};
use raylib::prelude::*;

pub fn on_render() {
    let mut state = RAYLIB_STATE.lock().unwrap();
    let size = 16.0;
    if let Some(ref mut raylib_state) = *state {
        let game_state = read_game_state();
        let mut d = raylib_state.rl.begin_drawing(&raylib_state.thread);
        d.clear_background(raylib::color::Color::WHITE);

        d.draw_text(
            format!("fps: {}", d.get_fps()).as_str(),
            12,
            12,
            20,
            Color::BLACK,
        );

        for (y, row) in game_state.arena.iter().enumerate() {
            for (x, &val) in row.iter().enumerate() {
                if val != 0 {
                    d.draw_rectangle(
                        (498.0 + (x as f32 * size)) as i32,
                        (36.0 + (y as f32 * size)) as i32,
                        size as i32,
                        size as i32,
                        read_game_state()
                            .colors
                            .iter()
                            .find(|(id, _)| id == &val)
                            .unwrap()
                            .1,
                    );
                }
            }
        }

        for i in 0..22 {
            d.draw_line(498 + (i * 16), 116, 498 + (i * 16), 676, Color::BLACK);
        }

        for i in 0..36 {
            d.draw_line(498, 116 + (i * 16), 834, 116 + (i * 16), Color::BLACK);
        }
    }
}
