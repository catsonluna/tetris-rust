use rand::Rng;

use crate::engine::managers::{game_manager::{read_game_manager, write_game_manager}, game_state::{read_game_state, write_game_state}};



pub fn on_tick() {
    if !read_game_manager().in_game {
        return;
    }

    if !read_game_manager().running {
        return;
    }

    should_respawn();
    check_spawn();
    move_down();
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

fn move_down() {
    let mut changed = false;
    let game_state = &mut write_game_state();

    if game_state.drop_ticks > 0.0 {
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
        game_state.drop_ticks = 1.0;
    }
}