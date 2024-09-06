
use raylib::prelude::*;

use crate::engine::{lib:: RAYLIB_STATE, managers::game_state::GAME_STATE};


pub fn on_render() {
    let shape = vec![
        vec![1, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 1],
    ];
    let size = 16.0;

    let mut d = unsafe {
        RAYLIB_STATE.as_mut().unwrap().rl.begin_drawing(&RAYLIB_STATE.as_mut().unwrap().thread)
    };

    let arena = &GAME_STATE.write().unwrap().arena;
    d.clear_background(Color::WHITE);


    d.draw_text(
        format!("fps: {}", d.get_fps()).as_str(),
        12,
        12,
        20,
        Color::BLACK,
    );


    // go over each pixel in arena, and if its 1 draw a square
    for (y, row) in arena.iter().enumerate() {
        for (x, &val) in row.iter().enumerate() {
            if val == 1 {
                d.draw_rectangle(
                    (498.0 + (x as f32 * size)) as i32,
                    (36.0 + (y as f32 * size)) as i32,
                    size as i32,
                    size as i32,
                    Color::BLACK,
                );
            }
            if val == 2 {
                d.draw_rectangle(
                    (498.0 + (x as f32 * size)) as i32,
                    (36.0 + (y as f32 * size)) as i32,
                    size as i32,
                    size as i32,
                    Color::RED,
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