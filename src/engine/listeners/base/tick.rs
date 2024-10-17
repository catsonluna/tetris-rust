use std::fmt::Debug;

use rand::{seq::SliceRandom, Rng};

use crate::engine::{
    events::events::END_GAME_EVENT,
    managers::{
        game_manager::{
            read_game_manager, write_game_manager_input_buffer, write_game_manager_running,
             KeyboardAction,
        },
        game_state::{
            read_game_state, write_game_state_all_pieces, write_game_state_arena,
            write_game_state_controlling, write_game_state_current_center,
            write_game_state_current_piece, write_game_state_down_hold,
            write_game_state_drop_ticks, write_game_state_game_data,
            write_game_state_ground_ticks, write_game_state_has_held, write_game_state_held_piece,
            write_game_state_left_hold, write_game_state_lines_till_next_level,
            write_game_state_piece_queue, write_game_state_right_hold,
        },
    },
};
use raylib::prelude::*;

#[derive(PartialEq)] // Add the PartialEq trait
enum Action {
    MoveRight,
    MoveLeft,
    MoveDown,
    Drop,
    Rotate,
    Hold,
    Pause,
}

impl Debug for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::MoveRight => write!(f, "MoveRight"),
            Action::MoveLeft => write!(f, "MoveLeft"),
            Action::MoveDown => write!(f, "MoveDown"),
            Action::Drop => write!(f, "Drop"),
            Action::Rotate => write!(f, "Rotate"),
            Action::Hold => write!(f, "Hold"),
            Action::Pause => write!(f, "Pause"),
        }
    }
}

pub fn on_tick() {
    // println!("Tick");

    if !read_game_manager().in_game {
        return;
    }

    if !read_game_manager().running {
        return;
    }
    if read_game_state().game_over {
        return;
    }
    clear_ghost();
    should_respawn();
    check_game_over();
    if !read_game_manager().in_game {
        return;
    }

    if !read_game_manager().running {
        return;
    }
    if read_game_state().game_over {
        return;
    }
    check_spawn();
    check_move();
    move_down(false);
    destoy_lines();
    draw_ghost();
    if read_game_manager().input_buffer.len() > 0 {
        write_game_manager_input_buffer(vec![]);
    }
}

fn should_respawn() {
    if read_game_state().ground_ticks > 48 {
        write_game_state_ground_ticks(0);
        write_game_state_controlling(0);
    }
}

fn check_spawn() {
    if read_game_state().controlling == 0 {
        let mut rng = read_game_manager().rng.clone();

        // check if piece queue is less than 6
        if read_game_state().piece_queue.len() < 8 {
            let shapes = read_game_manager().pieces.clone();
            let mut temp_shpaes = shapes.clone();
            let mut cloned_temp_shpaes = temp_shpaes.clone();
            temp_shpaes.append(&mut cloned_temp_shpaes);

            temp_shpaes.shuffle(&mut rng);

            let mut piece_queue = read_game_state().piece_queue.clone();
            piece_queue.append(&mut temp_shpaes);
            write_game_state_piece_queue(piece_queue);
        }

        let shape = read_game_state().piece_queue.clone().remove(0);

        write_game_state_current_piece(shape.clone());

        let mut piece_queue = read_game_state().piece_queue.clone();
        piece_queue.remove(0);
        write_game_state_piece_queue(piece_queue);

        let random = rng.gen::<i32>();

        let mut arena = read_game_state().arena.clone();
        for (y, row) in shape.layout.iter().enumerate() {
            for (x, &val) in row.iter().enumerate() {
                arena[y][x + 8] = if val == 1 { random } else { 0 };
            }
        }
        write_game_state_arena(arena);

        write_game_state_current_center((10, 2));
        write_game_state_controlling(random);
        let mut all_pieces = read_game_state().all_pieces.clone();
        all_pieces.push((random, shape.clone()));
        write_game_state_all_pieces(all_pieces);
        write_game_state_has_held(false);
    }
}

fn check_move() {
    let actions = process_input_buffer();

    for action in actions {
        match action {
            Action::Rotate => rotate(),
            Action::MoveRight => move_right(),
            Action::MoveLeft => move_left(),
            Action::MoveDown => _ = move_down(true),
            Action::Drop => drop(),
            Action::Hold => hold(),
            Action::Pause => {
                write_game_manager_running(false);
            }
        }
    }

    if read_game_state().right_hold.is_pressed {
        let mut right_hold = read_game_state().right_hold.clone();
        right_hold.move_ticks += 1;
        write_game_state_right_hold(right_hold);
        if read_game_state().right_hold.move_ticks > 10 {
            move_right();
            let mut right_hold = read_game_state().right_hold.clone();
            right_hold.move_ticks = 9;
            write_game_state_right_hold(right_hold);
        }
    } else {
        let mut right_hold = read_game_state().right_hold.clone();
        right_hold.move_ticks = 0;
        write_game_state_right_hold(right_hold);
    }

    if read_game_state().left_hold.is_pressed {
        let mut left_hold = read_game_state().left_hold.clone();
        left_hold.move_ticks += 1;
        write_game_state_left_hold(left_hold);
        if read_game_state().left_hold.move_ticks > 10 {
            move_left();
            let mut left_hold = read_game_state().left_hold.clone();
            left_hold.move_ticks = 9;
            write_game_state_left_hold(left_hold);
        }
    } else {
        let mut left_hold = read_game_state().left_hold.clone();
        left_hold.move_ticks = 0;
        write_game_state_left_hold(left_hold);
    }

    if read_game_state().down_hold.is_pressed {
        let mut down_hold = read_game_state().down_hold.clone();
        down_hold.move_ticks += 1;
        write_game_state_down_hold(down_hold);
        if read_game_state().down_hold.move_ticks > 10 {
            move_down(true);
            let mut down_hold = read_game_state().down_hold.clone();
            down_hold.move_ticks = 9;
            write_game_state_down_hold(down_hold);
        }
    } else {
        let mut down_hold = read_game_state().down_hold.clone();
        down_hold.move_ticks = 0;
        write_game_state_down_hold(down_hold);
    }
}

fn move_right() {
    let mut can_move = true;
    // go over each row, and get the furthest right value that is 1, then check if it can move right
    let mut arena = read_game_state().arena.clone();
    let controlling = read_game_state().controlling.clone();
    for y in 0..arena.len() {
        let mut furthest_right = None; // Start as None to check if there's a 1
        for x in (0..arena[y].len()).rev() {
            // Iterate from right to left
            if arena[y][x] == controlling {
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

    if can_move {
        for y in 0..arena.len() {
            for x in (0..arena[y].len()).rev() {
                // Iterate from right to left
                if arena[y][x] == read_game_state().controlling {
                    if x < arena[y].len() - 1 && arena[y][x + 1] == 0 {
                        arena[y][x + 1] = controlling;
                        arena[y][x] = 0;
                    }
                }
            }
        }

        write_game_state_arena(arena);

        // move the center of the piece to the right
        // game_state.current_center.0 += 1;
        let current_center = read_game_state().current_center.clone();
        write_game_state_current_center((current_center.0 + 1, current_center.1));
    }
}

fn move_left() {
    let mut can_move = true;
    let mut arena = read_game_state().arena.clone();
    let controlling = read_game_state().controlling.clone();

    // Go over each row and get the furthest left value that is 1, then check if it can move left
    for y in 0..arena.len() {
        let mut furthest_left = None; // Start as None to check if there's a 1
        for x in 0..arena[y].len() {
            // Iterate from left to right
            if arena[y][x] == controlling {
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
                if arena[y][x] == controlling {
                    if x > 0 && arena[y][x - 1] == 0 {
                        arena[y][x - 1] = controlling;
                        arena[y][x] = 0;
                    }
                }
            }
        }
        // move the center of the piece to the left
        // game_state.current_center.0 -= 1;
        write_game_state_arena(arena);
        let current_center = read_game_state().current_center.clone();
        write_game_state_current_center((current_center.0 - 1, current_center.1));
    }
}

fn move_down(forced: bool) -> bool {
    let mut can_move_down = true;

    if read_game_state().drop_ticks > 0.0 && !forced {
        // read_game_state().drop_ticks -= read_game_state().drop_speed;
        write_game_state_drop_ticks(read_game_state().drop_ticks - read_game_state().drop_speed);
        return false;
    }

    // Check if the piece can move down
    for y in (0..read_game_state().arena.len()).rev() {
        for x in 0..read_game_state().arena[y].len() {
            if read_game_state().arena[y][x] == read_game_state().controlling {
                if y + 1 >= read_game_state().arena.len()
                    || read_game_state().arena[y + 1][x] != 0
                        && read_game_state().arena[y + 1][x] != read_game_state().controlling
                {
                    can_move_down = false;
                    break;
                }
            }
        }
        if !can_move_down {
            break;
        }
    }

    let mut arena = read_game_state().arena.clone();

    // If it can move down, move everything that is controlling down
    if can_move_down {
        for y in (0..arena.len()).rev() {
            for x in 0..arena[y].len() {
                if arena[y][x] == read_game_state().controlling {
                    if y + 1 < arena.len() && arena[y + 1][x] == 0 {
                        arena[y + 1][x] = read_game_state().controlling;
                        arena[y][x] = 0;
                    }
                }
            }
        }
        // game_state.drop_ticks = 12.0;
        // game_state.current_center.1 += 1;
        write_game_state_arena(arena);
        write_game_state_drop_ticks(12.0);
        let current_center = read_game_state().current_center.clone();
        write_game_state_current_center((current_center.0, current_center.1 + 1));
        return true;
    } else {
        // If it can't move down, update the ground ticks
        // game_state.ground_ticks += 1;
        let ground_ticks = read_game_state().ground_ticks + 1;
        write_game_state_ground_ticks(ground_ticks);
        return false;
    }
}

fn drop() {
    while move_down(true) {}
    // game_state.controlling = 0;
    // game_state.drop_ticks = 0.0;
    write_game_state_controlling(0);
    write_game_state_drop_ticks(0.0);
}

fn check_game_over() {
    let arena = read_game_state().arena.clone();
    for y in 0..5 {
        for x in 0..arena[y].len() {
            if arena[y][x] != 0 && arena[y][x] != read_game_state().controlling {
                END_GAME_EVENT.call();
                break;
            }
        }
    }
}

fn destoy_lines() {
    let mut was_despawned = true;
    let mut despawned = 0;
    let mut arena = read_game_state().arena.clone();
    while was_despawned {
        was_despawned = false;
        for y in (0..arena.len()).rev() {
            let mut full = true;
            for x in 0..arena[y].len() {
                if arena[y][x] == 0 || arena[y][x] == read_game_state().controlling {
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

    write_game_state_arena(arena);
    if despawned > 0 {
        let level = if read_game_state().game_data.level > 15 {
            15
        } else {
            read_game_state().game_data.level
        };
        // game_state.game_data.score += (despawned * 100 * level) as i32;
        let mut game_data = read_game_state().game_data.clone();
        game_data.score += (despawned * 100 * level) as i32;
        write_game_state_game_data(game_data);

        // read_game_state().lines_till_next_level -= despawned as i32;
        let lines_till_next_level = read_game_state().lines_till_next_level - despawned as i32;
        write_game_state_lines_till_next_level(lines_till_next_level);
        if read_game_state().lines_till_next_level <= 0 {
            // game_state.game_data.level += 1;
            let mut game_data = read_game_state().game_data.clone();
            game_data.level += 1;
            write_game_state_game_data(game_data);
            if read_game_state().game_data.level < 13 {
                // game_state.drop_speed = 1.0 + (game_state.game_data.level as f32 * 0.75) / 2f32;
                write_game_state_drop_ticks(
                    1.0 + (read_game_state().game_data.level as f32 * 0.75) / 2f32,
                );
            }
            // game_state.lines_till_next_level = 5 + (game_state.game_data.level as f32 * 1.2) as i32;
            write_game_state_lines_till_next_level(
                5 + (read_game_state().game_data.level as f32 * 1.2) as i32,
            );
        }

        // game_state.game_data.lines_cleared += despawned as i32;
        let mut game_data = read_game_state().game_data.clone();
        game_data.lines_cleared += despawned as i32;
        write_game_state_game_data(game_data);
    }
}

fn process_input_buffer() -> Vec<&'static Action> {
    let mut actions: Vec<&Action> = vec![];

    for (key, key_action) in read_game_manager().input_buffer.iter() {
        if let Some(action) = get_action(key) {
            match action {
                Action::MoveRight => {
                    let mut right_hold = read_game_state().right_hold.clone();
                    if key_action == &KeyboardAction::Pressed {
                        actions.push(&Action::MoveRight);
                        // game_state.right_hold.is_pressed = true;
                        right_hold.is_pressed = true;
                    } else {
                        // game_state.right_hold.is_pressed = false;
                        right_hold.is_pressed = false;
                    }
                    write_game_state_right_hold(right_hold);
                }
                Action::MoveLeft => {
                    let mut left_hold = read_game_state().left_hold.clone();
                    if key_action == &KeyboardAction::Pressed {
                        actions.push(&Action::MoveLeft);
                        left_hold.is_pressed = true;
                    } else {
                        left_hold.is_pressed = false;
                    }
                    write_game_state_left_hold(left_hold);
                }
                Action::MoveDown => {
                    let mut down_hold = read_game_state().down_hold.clone();
                    if key_action == &KeyboardAction::Pressed {
                        actions.push(&Action::MoveDown);
                        down_hold.is_pressed = true;
                    } else {
                        down_hold.is_pressed = false;
                    }
                    write_game_state_down_hold(down_hold);
                }
                Action::Drop => {
                    if key_action == &KeyboardAction::Pressed {
                        actions.push(&Action::Drop);
                    }
                }
                Action::Rotate => {
                    if key_action == &KeyboardAction::Pressed {
                        actions.push(&Action::Rotate);
                    }
                }
                Action::Hold => {
                    if key_action == &KeyboardAction::Pressed {
                        actions.push(&Action::Hold);
                    }
                }
                Action::Pause => {
                    if key_action == &KeyboardAction::Pressed {
                        actions.push(&Action::Pause);
                    }
                }
            }
        }
    }

    actions
}

fn get_action(key: &KeyboardKey) -> Option<Action> {
    match key {
        KeyboardKey::KEY_RIGHT => Some(Action::MoveRight),
        KeyboardKey::KEY_LEFT => Some(Action::MoveLeft),
        KeyboardKey::KEY_DOWN => Some(Action::MoveDown),
        KeyboardKey::KEY_SPACE => Some(Action::Drop),
        KeyboardKey::KEY_UP => Some(Action::Rotate),
        KeyboardKey::KEY_LEFT_SHIFT => Some(Action::Hold),
        KeyboardKey::KEY_ESCAPE => Some(Action::Pause),
        _ => None,
    }
}

fn rotate() {
    // Remove the current piece from the arena
    let (center_x, center_y) = read_game_state().current_center;
    let controlling_id = read_game_state().controlling;
    let mut matrix = read_game_state().current_piece.layout.clone();

    let binding = read_game_state().clone();
    let block = match binding.all_pieces.iter().find(|&p| p.0 == controlling_id) {
        Some(piece) => &piece.1,
        None => return,
    };

    if !block.can_rotate {
        return;
    }

    let mut new_matrix = vec![vec![0; matrix.len()]; matrix[0].len()];
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            new_matrix[j][matrix.len() - 1 - i] = matrix[i][j];
        }
    }

    matrix = new_matrix;

    let mut arena = read_game_state().arena.clone();

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] == 1 {
                let x = j as i32 - 2;
                let y = i as i32 - 2;

                let pos_x = center_x as i32 + x;
                let pos_y = center_y as i32 + y;

                if pos_x < 0
                    || pos_x >= arena[0].len() as i32
                    || pos_y < 0
                    || pos_y >= arena.len() as i32
                    || (arena[pos_y as usize][pos_x as usize] != 0
                        && arena[pos_y as usize][pos_x as usize] != controlling_id)
                {
                    return;
                }
            }
        }
    }

    // go over each row in arena
    for y in 0..arena.len() {
        // go over each column in arena
        for x in 0..arena[y].len() {
            if arena[y][x] == controlling_id {
                arena[y][x] = 0;
            }
        }
    }

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] == 1 {
                // get the position of the block based on how far it is from (2, 2)
                let x = j as i32 - 2;
                let y = i as i32 - 2;

                let pos_x = center_x as i32 + x;
                let pos_y = center_y as i32 + y;

                arena[pos_y as usize][pos_x as usize] = controlling_id;
            }
        }
    }

    write_game_state_arena(arena);

    // Update the current piece and center
    // game_state.current_piece.layout = matrix;
    let mut current_piece = read_game_state().current_piece.clone();
    current_piece.layout = matrix;
    write_game_state_current_piece(current_piece);
}

fn hold() {
    if !read_game_state().has_held {
        write_game_state_has_held(true);
    } else {
        return;
    }
    let held_piece = read_game_state().held_piece.clone();
    let current_piece = read_game_state().current_piece.clone();

    let mut arena = read_game_state().arena.clone();
    for y in 0..arena.len() {
        for x in 0..arena[y].len() {
            if arena[y][x] == read_game_state().controlling {
                arena[y][x] = 0;
            }
        }
    }

    write_game_state_arena(arena);
    let mut arena = read_game_state().arena.clone();

    // check if something is held
    if held_piece.layout.len() == 0 {
        write_game_state_held_piece(current_piece.clone());
        write_game_state_controlling(0);
    } else {
        let random = read_game_manager().rng.clone().gen::<i32>();

        // spawn the held piece
        for (y, row) in held_piece.layout.iter().enumerate() {
            for (x, &val) in row.iter().enumerate() {
                arena[y][x + 8] = if val == 1 { random } else { 0 };
            }
        }

        write_game_state_arena(arena);

        // game_state.controlling = random;
        write_game_state_controlling(random);

        let mut all_pieces = read_game_state().all_pieces.clone();
        all_pieces.push((random, held_piece.clone()));
        write_game_state_all_pieces(all_pieces);
        write_game_state_current_piece(held_piece.clone());
        write_game_state_held_piece(current_piece.clone());

        // game_state.current_center = (10, 2);
        write_game_state_current_center((10, 2));
    }
}

fn clear_ghost() {
    let mut arena = read_game_state().arena.clone();
    for y in 0..arena.len() {
        for x in 0..arena[y].len() {
            if arena[y][x] == 2 {
                arena[y][x] = 0;
            }
        }
    }
    write_game_state_arena(arena);
}

fn draw_ghost() {
    let controlling = read_game_state().controlling.clone();
    let arena_backup = read_game_state().arena.clone();

    while move_down_ghost() {}

    let mut arena = read_game_state().arena.clone();

    for y in 0..arena.len() {
        for x in 0..arena[y].len() {
            if arena[y][x] == controlling {
                arena[y][x] = 2;
            }
        }
    }

    for y in 0..arena_backup.len() {
        for x in 0..arena_backup[y].len() {
            if arena_backup[y][x] == controlling {
                arena[y][x] = controlling;
            }
        }
    }

    write_game_state_arena(arena);
    write_game_state_controlling(controlling);
}

fn move_down_ghost() -> bool {
    let mut can_move_down = true;
    let mut arena = read_game_state().arena.clone();
    let controlling = read_game_state().controlling.clone();
    for y in (0..arena.len()).rev() {
        for x in 0..arena[y].len() {
            if arena[y][x] == controlling {
                if y + 1 >= arena.len() || arena[y + 1][x] != 0 && arena[y + 1][x] != controlling {
                    can_move_down = false;
                    break;
                }
            }
        }
        if !can_move_down {
            break;
        }
    }

    // If it can move down, move everything that is controlling down
    if can_move_down {
        for y in (0..arena.len()).rev() {
            for x in 0..arena[y].len() {
                if arena[y][x] == controlling {
                    if y + 1 < arena.len() && arena[y + 1][x] == 0 {
                        arena[y + 1][x] = controlling;
                        arena[y][x] = 0;
                    }
                }
            }
        }
        write_game_state_arena(arena);
        return true;
    } else {
        return false;
    }
}
