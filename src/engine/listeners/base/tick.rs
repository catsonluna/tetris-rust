use rand::Rng;

use crate::engine::{lib::RAYLIB_STATE, managers::{game_manager::{self, read_game_manager, write_game_manager}, game_state::{self, read_game_state, write_game_state}}};
use raylib::prelude::*;


pub fn on_tick() {
    if !read_game_manager().in_game {
        return;
    }

    if !read_game_manager().running {
        return;
    }

    should_respawn();
    check_spawn();
    check_move();
    move_down(false);
}

fn should_respawn() {
    let game_state = &mut write_game_state();
    if game_state.ground_ticks > 12 {
        game_state.ground_ticks = 0;
        game_state.controlling = 0;
    }
}

fn check_spawn() {
    if read_game_state().controlling == 0 {
        let shape = vec![
            vec![1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1],
        ];
        
        let rng = &mut write_game_manager().rng;

        let random = rng.gen::<i32>();


        for (y, row) in shape.iter().enumerate() {
            for (x, &val) in row.iter().enumerate() {
                write_game_state().arena[y][x + 8] = if val == 1 { random } else { 0 };
            }
        }
    
        write_game_state().controlling = random;
        let color = (random, raylib::color::Color::new(rng.gen::<u8>(), rng.gen::<u8>(), rng.gen::<u8>(), 255));
        write_game_state().colors.push(color);
    }
}

fn move_down(forced: bool) {
    let mut changed = false;
    let game_state = &mut write_game_state();

    if game_state.drop_ticks > 0.0 && !forced {
        game_state.drop_ticks -= game_state.drop_speed;
        return;
    }
    for y in (0..game_state.arena.len()).rev() {
        for x in 0..game_state.arena[y].len() {
            if game_state.arena[y][x] == game_state.controlling {
                if y + 1 >= game_state.arena.len() || game_state.arena[y + 1][x] != 0 {
                    changed = true;
                }
            }
        }
        if changed {
            game_state.ground_ticks += 1;
            break;
        } else {
            for x in 0..game_state.arena[y].len() {
                if game_state.arena[y][x] == game_state.controlling {
                    if y + 1 < game_state.arena.len() {
                        if game_state.arena[y + 1][x] == 0 {
                            game_state.arena[y + 1][x] = game_state.controlling;
                            game_state.arena[y][x] = 0;
                        }
                    }
                }
            }
        }
    }

    if !changed {
        game_state.drop_ticks = 6.0;
    }
}

fn check_move() {
    let mut state = RAYLIB_STATE.lock().unwrap();
    if let Some(ref mut raylib_state) = *state {
        if raylib_state.rl.is_key_down(KeyboardKey::KEY_S) {
            if read_game_state().down_hold < 4 {
                write_game_state().down_hold += 1;
            }else{
                move_down(true);
                write_game_state().down_hold = 2;
            }
        }else{
            if read_game_state().down_hold > 0 {
                move_down(true);
            }
            write_game_state().down_hold = 0;
        }

        if raylib_state.rl.is_key_down(KeyboardKey::KEY_D) {
            if read_game_state().right_hold < 4 {
                write_game_state().right_hold += 1;
            }else{
                move_right();
                write_game_state().right_hold = 2;
            }
        }else{
            if read_game_state().right_hold > 0 {
                move_right();
            }
            write_game_state().right_hold = 0;
        }

        if raylib_state.rl.is_key_down(KeyboardKey::KEY_A) {
            if read_game_state().left_hold < 4 {
                write_game_state().left_hold += 1;
            }else{
                move_left();
                write_game_state().left_hold = 2;
            }
        }else{
            if read_game_state().left_hold > 0 {
                move_left();
            }
            write_game_state().left_hold = 0;
        }

        if raylib_state.rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            drop();
        }


    }
}


fn move_right() {
    let mut can_move = true;
    let game_state = &mut write_game_state();
    // go over each row, and get the furthest right value that is 1, then check if it can move right
    for y in 0..game_state.arena.len() {
        let mut furthest_right = None; // Start as None to check if there's a 1
        for x in (0..game_state.arena[y].len()).rev() {
            // Iterate from right to left
            if game_state.arena[y][x] == game_state.controlling {
                furthest_right = Some(x);
                break; // We can break here as we're looking for the first (furthest right) 1
            }
        }
        if let Some(x) = furthest_right {
            if x == game_state.arena[y].len() - 1 {
                can_move = false; // If it's already at the right edge, it can't move right
            } else if game_state.arena[y][x + 1] != 0 {
                can_move = false; // If the space to the right is not 0, it can't move
            }
        }
    }

    // If it can move right, move everything that is 1 to the right
    if can_move {
        for y in 0..game_state.arena.len() {
            for x in (0..game_state.arena[y].len()).rev() {
                // Iterate from right to left
                if game_state.arena[y][x] == game_state.controlling {
                    if x < game_state.arena[y].len() - 1 && game_state.arena[y][x + 1] == 0 {
                        game_state.arena[y][x + 1] = game_state.controlling;
                        game_state.arena[y][x] = 0;
                    }
                }
            }
        }
    }
}

fn move_left() {
    let mut can_move = true;
    let game_state = &mut write_game_state();

    // Go over each row and get the furthest left value that is 1, then check if it can move left
    for y in 0..game_state.arena.len() {
        let mut furthest_left = None; // Start as None to check if there's a 1
        for x in 0..game_state.arena[y].len() {
            // Iterate from left to right
            if game_state.arena[y][x] == game_state.controlling {
                furthest_left = Some(x);
                break; // We can break here as we're looking for the first (furthest left) 1
            }
        }
        if let Some(x) = furthest_left {
            if x == 0 {
                can_move = false; // If it's already at the left edge, it can't move left
            } else if game_state.arena[y][x - 1] != 0 {
                can_move = false; // If the space to the left is not 0, it can't move
            }
        }
    }

    // If it can move left, move everything that is 1 to the left
    if can_move {
        for y in 0..game_state.arena.len() {
            for x in 0..game_state.arena[y].len() {
                // Iterate from left to right
                if game_state.arena[y][x] == game_state.controlling {
                    if x > 0 && game_state.arena[y][x - 1] == 0 {
                        game_state.arena[y][x - 1] = game_state.controlling;
                        game_state.arena[y][x] = 0;
                    }
                }
            }
        }
    }
}

fn drop() {
    let mut done = false;
    let mut changed = false;

    let game_state = &mut write_game_state();
    while !done {
        for y in (0..game_state.arena.len()).rev() {
            for x in 0..game_state.arena[y].len() {
                if game_state.arena[y][x] == game_state.controlling {
                    if y + 1 >= game_state.arena.len() || game_state.arena[y + 1][x] != 0 {
                        changed = true;
                    }
                }
            }
            if changed {
                game_state.ground_ticks += 1;
                done = true;
                break;
            } else {
                for x in 0..game_state.arena[y].len() {
                    if game_state.arena[y][x] == game_state.controlling {
                        if y + 1 < game_state.arena.len() {
                            if game_state.arena[y + 1][x] == 0 {
                                game_state.arena[y + 1][x] = game_state.controlling;
                                game_state.arena[y][x] = 0;
                            }
                        }
                    }
                }
            }
        }
    }
    game_state.controlling = 0;
    game_state.drop_ticks = 0.0;
}