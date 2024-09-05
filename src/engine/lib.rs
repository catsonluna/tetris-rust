use std::time::{Duration, Instant};

use super::{events::events::UPDATE_EVENT, listeners::lib::register_events};
use rand::Rng;
use raylib::prelude::*;
use once_cell::sync::Lazy;


pub struct GameState {
    pub last_update: Instant,
    pub tick_accumulator: Duration,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            last_update: Instant::now(),
            tick_accumulator: Duration::from_secs(0),
        }
    }
}

pub static mut GAME_STATE: Lazy<GameState> = Lazy::new(|| GameState::new());


pub fn start() {
    register_events();

    let mut rng = rand::thread_rng();
    let (mut rl, thread) = raylib::init()
        .size(1600, 900)
        .resizable()
        .title("Tetris")
        .msaa_4x()
        .build();

    rl.set_target_fps(60);

    let drop_frame = 8;
    let mut drop_frame_counter = 0;

    let mut down_pressed_for = 0;
    let mut right_pressed_for = 0;
    let mut left_pressed_for = 0;

    let mut should_spawn = false;

    let mut on_bottom = false;
    let mut on_bottom_for = 0;

    let size = 16.0;

    let mut game_over = false;

    // make an arena, but make it higher by 9 to account for spawning
    let mut arena = vec![vec![0; 21]; 40];

    let shape1 = vec![
        vec![1, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 1],
    ];

    let shape2 = vec![
        vec![0, 0, 1, 0, 0],
        vec![0, 0, 1, 0, 0],
        vec![0, 0, 1, 0, 0],
        vec![0, 0, 1, 0, 0],
        vec![0, 0, 1, 0, 0],
    ];

    let shape3 = vec![
        vec![0, 0, 0, 0, 0],
        vec![0, 0, 1, 0, 0],
        vec![0, 0, 1, 0, 0],
        vec![0, 1, 1, 1, 0],
        vec![1, 1, 1, 1, 1],
    ];

    let shape4 = vec![
        vec![0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0],
        vec![0, 1, 1, 1, 0],
        vec![0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0],
    ];

    let shapes = vec![shape1, shape2];

    let shape = shapes[rng.gen_range(0..shapes.len())].clone();

    for (y, row) in shape.iter().enumerate() {
        for (x, &val) in row.iter().enumerate() {
            arena[y][x + 9] = val;
        }
    }

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);

        // if game over, draw a game over text
        if game_over {
            d.draw_text("Game Over", 498, 116, 20, Color::BLACK);
        }

        if on_bottom {
            on_bottom_for += 1;
            if on_bottom_for > 16 {
                should_spawn = true;
                on_bottom = false;
                on_bottom_for = 0;
            }
        } else {
            on_bottom_for = 0;
        }

        if should_spawn {
            for y in 0..arena.len() {
                for x in 0..arena[y].len() {
                    if arena[y][x] == 1 {
                        arena[y][x] = 2;
                    }
                }
            }
            for (y, row) in shape.iter().enumerate() {
                for (x, &val) in row.iter().enumerate() {
                    // check if the shape is colliding with anything
                    if arena[y][x + 8] != 0 {
                        game_over = true;
                    }
                }
            }

            // if the shape is not colliding with anything, spawn it
            if !game_over {
                let shape = shapes[rng.gen_range(0..shapes.len())].clone();
                for (y, row) in shape.iter().enumerate() {
                    for (x, &val) in row.iter().enumerate() {
                        arena[y][x + 8] = val;
                    }
                }
            }
            should_spawn = false;
        }

        d.draw_text(
            format!("Right frame: {}", right_pressed_for).as_str(),
            12,
            12,
            20,
            Color::BLACK,
        );

        // if the right key is pressed, move everything that is 1 to the right
        if d.is_key_down(KeyboardKey::KEY_RIGHT) {
            right_pressed_for += 1;
            if right_pressed_for > 5 {
                move_right(&mut arena);
                right_pressed_for = 3;
            }
        } else {
            if right_pressed_for > 0 {
                move_right(&mut arena);
            }
            right_pressed_for = 0;
        }

        // if the left key is pressed, move everything that is 1 to the left
        if d.is_key_down(KeyboardKey::KEY_LEFT) {
            left_pressed_for += 1;
            if left_pressed_for > 5 {
                move_left(&mut arena);
                left_pressed_for = 3;
            }
        } else {
            if left_pressed_for > 0 {
                move_left(&mut arena);
            }
            left_pressed_for = 0;
        }

        // if the down key is pressed, move everything that is 1 down
        if d.is_key_down(KeyboardKey::KEY_DOWN) {
            down_pressed_for += 1;
            if down_pressed_for > 5 {
                move_down(&mut arena, &mut on_bottom);
                down_pressed_for = 3;
            }
        } else {
            if down_pressed_for > 0 {
                move_down(&mut arena, &mut on_bottom);
            }
            down_pressed_for = 0;
        }

        // if spacebar is pressed, move everything that is 1 down until it collides with something or reaches the bottom
        if d.is_key_pressed(KeyboardKey::KEY_SPACE) {
            let mut done = false;
            let mut changed = false;
            while !done {
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
                    } else {
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
                    done = true;
                }
            }
            drop_frame_counter = 0;
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

        for i in 0..22 {
            d.draw_line(498 + (i * 16), 116, 498 + (i * 16), 676, Color::BLACK);
        }

        for i in 0..36 {
            d.draw_line(498, 116 + (i * 16), 834, 116 + (i * 16), Color::BLACK);
        }

        if drop_frame_counter > drop_frame {
            move_down(&mut arena, &mut on_bottom);
            drop_frame_counter = 0;
        }
        despawn_full_lines(&mut arena);
        drop_frame_counter += 1;

        UPDATE_EVENT.call();
    }

}

fn move_down(arena: &mut Vec<Vec<i32>>, on_bottom: &mut bool) {
    let mut changed = false;
    for y in (0..arena.len()).rev() {
        for x in 0..arena[y].len() {
            if arena[y][x] == 1 {
                if y + 1 >= arena.len() || arena[y + 1][x] != 0 {
                    changed = true;
                }
            }
        }
        if changed {
            *on_bottom = true;

            break;
        } else {
            *on_bottom = false;
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
}

fn despawn_full_lines(arena: &mut Vec<Vec<i32>>) {
    let mut was_despawned = true;
    let mut despawned = 0;
    while was_despawned {
        was_despawned = false;
        for y in (0..arena.len()).rev() {
            let mut full = true;
            for x in 0..arena[y].len() {
                if arena[y][x] == 0 {
                    full = false;
                }
            }
            if full {
                was_despawned = true;
                despawned += 1;
                for y2 in (0..y).rev() {
                    for x in 0..arena[y2].len() {
                        arena[y2 + 1][x] = arena[y2][x];
                    }
                }
            }
        }
    }
    if despawned > 0 {
        println!("Despawned: {}", despawned);
    }
}
fn move_right(arena: &mut Vec<Vec<i32>>) {
    let mut can_move = true;

    // go over each row, and get the furthest right value that is 1, then check if it can move right
    for y in 0..arena.len() {
        let mut furthest_right = None; // Start as None to check if there's a 1
        for x in (0..arena[y].len()).rev() {
            // Iterate from right to left
            if arena[y][x] == 1 {
                furthest_right = Some(x);
                break; // We can break here as we're looking for the first (furthest right) 1
            }
        }
        if let Some(x) = furthest_right {
            if x == arena[y].len() - 1 {
                can_move = false; // If it's already at the right edge, it can't move right
            } else if arena[y][x + 1] != 0 {
                can_move = false; // If the space to the right is not 0, it can't move
            }
        }
    }

    // If it can move right, move everything that is 1 to the right
    if can_move {
        for y in 0..arena.len() {
            for x in (0..arena[y].len()).rev() {
                // Iterate from right to left
                if arena[y][x] == 1 {
                    if x < arena[y].len() - 1 && arena[y][x + 1] == 0 {
                        arena[y][x + 1] = 1;
                        arena[y][x] = 0;
                    }
                }
            }
        }
    }
}

fn move_left(arena: &mut Vec<Vec<i32>>) {
    let mut can_move = true;

    // Go over each row and get the furthest left value that is 1, then check if it can move left
    for y in 0..arena.len() {
        let mut furthest_left = None; // Start as None to check if there's a 1
        for x in 0..arena[y].len() {
            // Iterate from left to right
            if arena[y][x] == 1 {
                furthest_left = Some(x);
                break; // We can break here as we're looking for the first (furthest left) 1
            }
        }
        if let Some(x) = furthest_left {
            if x == 0 {
                can_move = false; // If it's already at the left edge, it can't move left
            } else if arena[y][x - 1] != 0 {
                can_move = false; // If the space to the left is not 0, it can't move
            }
        }
    }

    // If it can move left, move everything that is 1 to the left
    if can_move {
        for y in 0..arena.len() {
            for x in 0..arena[y].len() {
                // Iterate from left to right
                if arena[y][x] == 1 {
                    if x > 0 && arena[y][x - 1] == 0 {
                        arena[y][x - 1] = 1;
                        arena[y][x] = 0;
                    }
                }
            }
        }
    }
}