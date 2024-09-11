use crate::engine::{
    lib::RAYLIB_STATE,
    managers::{
        game_manager::{read_game_manager, write_game_manager},
        game_state::{read_game_state, write_game_state},
    },
};
use raylib::prelude::*;

pub fn on_render() {
    if read_game_manager().in_game {
        render_game();
    } else {
        render_main_menu();
    }
}

fn render_main_menu() {
    let mut state = RAYLIB_STATE.lock().unwrap();
    if let Some(ref mut raylib_state) = *state {
        let mut d = raylib_state.rl.begin_drawing(&raylib_state.thread);

        d.draw_text("Tetris", 498, 116, 100, Color::BLACK);

        d.gui_set_style(
            GuiControl::BUTTON,
            raylib::consts::GuiControlProperty::TEXT_ALIGNMENT as i32,
            raylib::consts::GuiTextAlignment::TEXT_ALIGN_CENTER as i32,
        );

        if d.gui_button(rrect(30, 320, 115, 30), Some(rstr!("Start Game"))) {
            let game_manager = &mut write_game_manager();
            game_manager.in_game = true;
            game_manager.running = true;
            game_manager.input_buffer.clear();

            let game_state = &mut write_game_state();
            game_state.reset();
        }

        if d.gui_button(rrect(30, 360, 115, 30), Some(rstr!("Quit"))) {
            write_game_manager().should_quit = true;
        }

        d.clear_background(raylib::color::Color::WHITE);
    }
}

fn render_game() {
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
                        (624.0 + (x as f32 * size)) as i32,
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
            d.draw_line(624 + (i * 16), 116, 624 + (i * 16), 676, Color::BLACK);
        }

        for i in 0..36 {
            d.draw_line(624, 116 + (i * 16), 960, 116 + (i * 16), Color::BLACK);
        }

        d.draw_text(
            format!("Score: {}", game_state.score).as_str(),
            624,
            12,
            20,
            Color::BLACK,
        );

        d.draw_text(
            format!("Level: {}", game_state.level).as_str(),
            624,
            36,
            20,
            Color::BLACK,
        );

        d.draw_text(
            format!("Speed: {}", game_state.drop_speed).as_str(),
            624,
            60,
            20,
            Color::BLACK,
        );

        d.draw_text(
            format!(
                "Lines till next level: {}",
                game_state.lines_till_next_level
            )
            .as_str(),
            624,
            84,
            20,
            Color::BLACK,
        );
    }
}
