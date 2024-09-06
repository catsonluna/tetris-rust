use crate::engine::managers::{game_manager::read_game_manager, game_state::{read_game_state, write_game_state}};



pub fn on_tick() {
    if !read_game_manager().in_game {
        return;
    }

    if !read_game_manager().running {
        return;
    }

    check_spawn();
    move_down();
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
    
        for (y, row) in shape.iter().enumerate() {
            for (x, &val) in row.iter().enumerate() {
                write_game_state().arena[y][x + 8] = val;
            }
        }
    
        write_game_state().controlling = 1;
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
            if game_state.arena[y][x] == 1 {
                if y + 1 >= game_state.arena.len() || game_state.arena[y + 1][x] != 0 {
                    changed = true;
                }
            }
        }
        if changed {
            break;
        } else {
            for x in 0..game_state.arena[y].len() {
                if game_state.arena[y][x] == 1 {
                    if y + 1 < game_state.arena.len() {
                        if game_state.arena[y + 1][x] == 0 {
                            game_state.arena[y + 1][x] = 1;
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