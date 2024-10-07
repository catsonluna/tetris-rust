use raylib::prelude::*;

use crate::engine::common::ui;
use crate::engine::lib::RAYLIB_STATE;
use crate::engine::managers::game_manager::{self, read_game_manager};
use crate::engine::managers::game_state::read_game_state;

// Base resolution as reference
const BASE_WIDTH: i32 = 1600;
const BASE_HEIGHT: i32 = 900;

pub fn on_render() {
    if !read_game_manager().in_game {
        render_main_menu();
    } else {
        render_game();
    }
}

fn get_scaling_factors(d: &RaylibDrawHandle) -> (f32, f32) {
    let screen_width = d.get_screen_width() as f32;
    let screen_height = d.get_screen_height() as f32;

    let scale_x = screen_width / BASE_WIDTH as f32;
    let scale_y = screen_height / BASE_HEIGHT as f32;

    (scale_x, scale_y)
}

fn scaled_value(value: i32, scale: f32) -> i32 {
    (value as f32 * scale).round() as i32
}

fn render_main_menu() {
    let mut state = RAYLIB_STATE.lock().unwrap();
    if let Some(ref mut raylib_state) = *state {
        let mut d = raylib_state.rl.begin_drawing(&raylib_state.thread);
        let (scale_x, scale_y) = get_scaling_factors(&d);

        ui::text::text(
            &mut d,
            scaled_value(800, scale_x),
            scaled_value(116, scale_y),
            Color::BLACK,
            "Tetris".to_string(),
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
            "com.catsonluna.revris.button.play".to_string()
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
            "".to_string()
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
            "".to_string()
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
            "com.catsonluna.revris.button.quit".to_string()
        );

        d.clear_background(Color::from_hex("cfcefc".as_ref()).unwrap());
    }
}

fn render_game() {
    let mut state = RAYLIB_STATE.lock().unwrap();
    if let Some(ref mut raylib_state) = *state {
        let mut d = raylib_state.rl.begin_drawing(&raylib_state.thread);
        d.clear_background(Color::from_hex("cfcefc".as_ref()).unwrap());

        let (scale_x, scale_y) = get_scaling_factors(&d);

        d.draw_fps(scaled_value(10, scale_x), scaled_value(10, scale_y));

        // Draw other UI elements like score, level, and speed using scaled positions
        ui::text::text(
            &mut d,
            // to the left of held piece
            scaled_value(300, scale_x),
            scaled_value(220, scale_y),
            Color::BLACK,
            "Score".to_string(),
            scaled_value(20, scale_y),
        );

        ui::text::text(
            &mut d,
            scaled_value(300, scale_x),
            scaled_value(240, scale_y),
            Color::BLACK,
            read_game_state().game_data.score.to_string(),
            scaled_value(20, scale_y),
        );
        // high score bellow
        ui::text::text(
            &mut d,
            scaled_value(300, scale_x),
            scaled_value(270, scale_y),
            Color::BLACK,
            "High Score".to_string(),
            scaled_value(20, scale_y),
        );
        ui::text::text(
            &mut d,
            scaled_value(300, scale_x),
            scaled_value(290, scale_y),
            Color::BLACK,
            if read_game_manager().save_data.best_game.score > read_game_state().game_data.score {
                read_game_manager().save_data.best_game.score.to_string()
            } else {
                read_game_state().game_data.score.to_string()
            },
            scaled_value(20, scale_y),
        );

        // level bellow
        ui::text::text(
            &mut d,
            scaled_value(300, scale_x),
            scaled_value(320, scale_y),
            Color::BLACK,
            "Level".to_string(),
            scaled_value(20, scale_y),
        );

        ui::text::text(
            &mut d,
            scaled_value(300, scale_x),
            scaled_value(340, scale_y),
            Color::BLACK,
            read_game_state().game_data.level.to_string(),
            scaled_value(20, scale_y),
        );

        let board_x = scaled_value(624, scale_x); // Top-left X position of the game board
        let board_y = scaled_value(56, scale_y); // Top-left Y position of the game board
        let cell_size = scaled_value(16, scale_x); // Size of each cell, scaled based on screen size

        for (y, row) in read_game_state().arena.iter().enumerate() {
            for (x, &val) in row.iter().enumerate() {
                let cell_x = board_x + (x as i32 * cell_size); // Calculate the cell's X position
                let cell_y = board_y + (y as i32 * cell_size); // Calculate the cell's Y position

                if val != 0 && val != 2 {
                    d.draw_rectangle(
                        cell_x,
                        cell_y,
                        cell_size,
                        cell_size,
                        // Find the piece color
                        read_game_state()
                            .all_pieces
                            .iter()
                            .find(|&p| p.0 == val)
                            .unwrap()
                            .1
                            .color,
                    );
                }

                if val == 2 {
                    d.draw_rectangle(cell_x, cell_y, cell_size, cell_size, Color::GRAY);
                }

                if y > 5 {
                    // d.draw_rectangle_lines(cell_x, cell_y, cell_size, cell_size, Color::BLACK);
                    d.draw_rectangle_lines_ex(
                        rrect(
                            cell_x as f32,
                            cell_y as f32,
                            cell_size as f32,
                            cell_size as f32,
                        ),
                        0.5,
                        Color::BLACK,
                    );
                }
            }
        }
        let held_x = scaled_value(500, scale_x);
        let held_y = scaled_value(220, scale_y);

        let held_size = scaled_value(16, scale_x);

        ui::text::text(
            &mut d,
            held_x,
            held_y,
            Color::BLACK,
            "Held Piece".to_string(),
            scaled_value(20, scale_y),
        );

        for (y, row) in read_game_state().held_piece.layout.iter().enumerate() {
            for (x, &val) in row.iter().enumerate() {
                let cell_x = held_x - 40 + (x as i32 * held_size); // Calculate the cell's X position
                let cell_y = held_y + 25 + (y as i32 * held_size); // Calculate the cell's Y position

                if val != 0 {
                    d.draw_rectangle(
                        cell_x,
                        cell_y,
                        held_size,
                        held_size,
                        read_game_state().held_piece.color,
                    );
                }
            }
        }

        // on the right side render the next 5 pieces one on top of the other
        let queue_x = scaled_value(1100, scale_x);
        let queue_y = scaled_value(250, scale_y);
        let queue_size = scaled_value(8, scale_y);

        ui::text::text(
            &mut d,
            queue_x,
            queue_y,
            Color::BLACK,
            "Next Pieces".to_string(),
            scaled_value(20, scale_y),
        );

        for (i, piece) in read_game_state().piece_queue.iter().enumerate() {
            let piece_x = queue_x - scaled_value(25, scale_x);
            let piece_y = queue_y + 25 + ((i as i32) * 5 * queue_size);

            if i > 5 {
                break;
            }

            for (y, row) in piece.layout.iter().enumerate() {
                for (x, &val) in row.iter().enumerate() {
                    let cell_x = piece_x + (x as i32 * queue_size); // Calculate the cell's X position
                    let cell_y = piece_y + (y as i32 * queue_size) + queue_size * i as i32;

                    if val != 0 {
                        d.draw_rectangle(cell_x, cell_y, queue_size, queue_size, piece.color);
                    }
                }
            }
        }

        if read_game_state().game_over {
            render_game_over(&mut d, scale_x, scale_y);
        }

        if !read_game_manager().running {
            render_pause_menu(&mut d, scale_x, scale_y);
        }
    }
}

fn render_game_over(d: &mut RaylibDrawHandle, scale_x: f32, scale_y: f32) {
    // Example for Game Over screen scaling
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

    d.draw_text(
        "Game Over",
        scaled_value(410, scale_x),
        scaled_value(266, scale_y),
        scaled_value(20, scale_y),
        Color::BLACK,
    );

    if d.gui_button(
        rrect(
            scaled_value(410, scale_x),
            scaled_value(300, scale_y),
            scaled_value(140, scale_x),
            scaled_value(30, scale_y),
        ),
        Some(rstr!("Restart")),
    ) {
        // let game_manager = &mut write_game_manager();
        // game_manager.in_game = true;
        // game_manager.running = true;
        // game_manager.input_buffer.clear();

        // let mut game_state = write_game_state();
        // *game_state = GameState::new();
    }

    if d.gui_button(
        rrect(
            scaled_value(410, scale_x),
            scaled_value(340, scale_y),
            scaled_value(140, scale_x),
            scaled_value(30, scale_y),
        ),
        Some(rstr!("Main Menu")),
    ) {
        // let game_manager = &mut write_game_manager();
        // game_manager.in_game = false;
        // game_manager.running = false;
    }

    // quit button
    if d.gui_button(
        rrect(
            scaled_value(410, scale_x),
            scaled_value(380, scale_y),
            scaled_value(140, scale_x),
            scaled_value(30, scale_y),
        ),
        Some(rstr!("Quit")),
    ) {
        // write_game_manager().should_quit = true;
    }
}

fn render_pause_menu(d: &mut RaylibDrawHandle, scale_x: f32, scale_y: f32) {
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

    d.draw_text(
        "Paused",
        scaled_value(410, scale_x),
        scaled_value(266, scale_y),
        scaled_value(20, scale_y),
        Color::BLACK,
    );

    if d.gui_button(
        rrect(
            scaled_value(410, scale_x),
            scaled_value(300, scale_y),
            scaled_value(140, scale_x),
            scaled_value(30, scale_y),
        ),
        Some(rstr!("Resume")),
    ) {
        // write_game_manager().running = true;
    }

    if d.gui_button(
        rrect(
            scaled_value(410, scale_x),
            scaled_value(340, scale_y),
            scaled_value(140, scale_x),
            scaled_value(30, scale_y),
        ),
        Some(rstr!("Main Menu")),
    ) {
        // let game_manager = &mut write_game_manager();
        // game_manager.in_game = false;
        // game_manager.running = false;
    }

    // quit button
    if d.gui_button(
        rrect(
            scaled_value(410, scale_x),
            scaled_value(380, scale_y),
            scaled_value(140, scale_x),
            scaled_value(30, scale_y),
        ),
        Some(rstr!("Quit")),
    ) {
        // write_game_manager().should_quit = true;
    }
}
