use raylib::prelude::*;
fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1600, 900)
        .resizable()
        .title("Tetris")
        .build();

    rl.set_target_fps(60);

    let drop_frame = 8;
    let mut drop_frame_counter = 0;

    let mut down_pressed_for = 0;
    let mut right_pressed_for = 0;
    let mut left_pressed_for = 0;

    let mut should_spawn = false;

    let size = 16.0;

    let mut game_over = false;

    // make an arena, but make it higher by 9 to account for spawning
    let mut arena = vec![vec![0; 37]; 48];

    let shape = vec![
        vec![1, 1, 1, 1, 1], 
        vec![1, 1, 1, 1, 1], 
        vec![1, 1, 1, 1, 1], 
        vec![1, 1, 1, 1, 1], 
        vec![1, 1, 1, 1, 1]];



    for (y, row) in shape.iter().enumerate() {
        for (x, &val) in row.iter().enumerate() {
            arena[y][x + 16] = val;
        }
    }

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);

        // if game over, draw a game over text
        if game_over {
            d.draw_text("Game Over", 498, 116, 20, Color::BLACK);
        }

        if should_spawn {
            for (y, row) in shape.iter().enumerate() {
                for (x, &val) in row.iter().enumerate() {
                    // check if the shape is colliding with anything
                    if arena[y][x + 16] != 0 {
                        game_over = true;
                    }
                }
            }

            // if the shape is not colliding with anything, spawn it
            if !game_over {
                for (y, row) in shape.iter().enumerate() {
                    for (x, &val) in row.iter().enumerate() {
                        arena[y][x + 16] = val;
                    }
                }
            }
            should_spawn = false;
        }

        d.draw_text(
            format!("Drop frame: {}", drop_frame_counter).as_str(),
            12,
            12,
            20,
            Color::BLACK,
        );


        // if the right key is pressed, move everything that is 1 to the right
        if d.is_key_down(KeyboardKey::KEY_RIGHT) {
            right_pressed_for += 1;
            if right_pressed_for > 5 {
                for y in 0..arena.len() {
                    for x in (0..arena[y].len()).rev() {
                        if arena[y][x] == 1 {
                            if x + 1 < arena[y].len() {
                                if arena[y][x + 1] == 0 {
                                    arena[y][x + 1] = 1;
                                    arena[y][x] = 0;
                                }
                            }
                        }
                    }
                }
                right_pressed_for = 0;
            }
        } else {
            right_pressed_for = 0;
        }

        // if the left key is pressed, move everything that is 1 to the left
        if d.is_key_down(KeyboardKey::KEY_LEFT) {
            left_pressed_for += 1;
            if left_pressed_for > 5 {
                for y in 0..arena.len() {
                    for x in 0..arena[y].len() {
                        if arena[y][x] == 1 {
                            if x > 0 {
                                if arena[y][x - 1] == 0 {
                                    arena[y][x - 1] = 1;
                                    arena[y][x] = 0;
                                }
                            }
                        }
                    }
                }
                left_pressed_for = 0;
            }
        } else {
            left_pressed_for = 0;
        }

        // if the down key is pressed, move everything that is 1 down
        if d.is_key_down(KeyboardKey::KEY_DOWN) {
            down_pressed_for += 1;
            if down_pressed_for > 5 {
                for y in (0..arena.len()).rev() {
                    for x in 0..arena[y].len() {
                        if arena[y][x] == 1 {
                            if y + 1 < arena.len() {
                                if arena[y + 1][x] == 0 {
                                    arena[y + 1][x] = 1;
                                    arena[y][x] = 0;
                                }
                            }
                        }
                    }
                }
                down_pressed_for = 0;
            }
        } else {
            down_pressed_for = 0;
        }

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

        // draw a line at the bottom of the arena
        d.draw_line(498, 804, 1090, 804, Color::BLACK);

        // draw a line at the right of the arena
        d.draw_line(1090, 804, 1090, 116, Color::BLACK);

        // draw a grid that goes from top to bottom right to left, where each square is 16x16

        for i in 0..37 {
            d.draw_line(498 + (i * 16), 116, 498 + (i * 16), 804, Color::BLACK);
        }

        for i in 0..44 {
            d.draw_line(498, 116 + (i * 16), 1090, 116 + (i * 16), Color::BLACK);
        }
        
        if drop_frame_counter > drop_frame {
            let mut changed = false;
            for y in (0..arena.len()).rev() {
                for x in 0..arena[y].len() {
                    if arena[y][x] == 1 {
                        if y + 1 >= arena.len() || arena[y + 1][x] != 0 {
                            arena[y][x] = 2;
                            changed = true;
                        }
                    }
                }
                if changed {
                    break;
                }else{
                    // move everything that is 1 down
                    for x in 0..arena[y].len() {
                        if arena[y][x] == 1 {
                            if y + 1 < arena.len() {
                                if arena[y + 1][x] == 0 {
                                    arena[y + 1][x] = 1;
                                    arena[y][x] = 0;
                                }
                            }
                        }
                    }
                }
            }
            if changed {
                for y in 0..arena.len() {
                    for x in 0..arena[y].len() {
                        if arena[y][x] == 1 {
                            arena[y][x] = 2;
                        }
                    }
                }
                should_spawn = true;
            }
            drop_frame_counter = 0;
        }
        drop_frame_counter += 1;
    }
}
