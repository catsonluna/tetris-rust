use std::fmt::Debug;

use rand::Rng;

use crate::engine::managers::{
    game_manager::{self, write_game_manager, KeyboardAction},
    game_state::{self, write_game_state},
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
        }
    }
}

pub fn on_tick() {
    let game_state = &mut write_game_state();
    let game_manager = &mut write_game_manager();

    if !game_manager.in_game {
        return;
    }

    if !game_manager.running {
        return;
    }

    should_respawn(game_state);
    check_game_over(game_manager, game_state);
    if !game_manager.in_game {
        return;
    }

    if !game_manager.running {
        return;
    }
    check_spawn(game_manager, game_state);
    check_move(game_manager, game_state);
    move_down(game_state, false);
    destoy_lines(game_state);

    if game_manager.input_buffer.len() > 0 {
        game_manager.input_buffer.clear();
    }
}

fn should_respawn(game_state: &mut std::sync::RwLockWriteGuard<'_, game_state::GameState>) {
    if game_state.ground_ticks > 48 {
        game_state.ground_ticks = 0;
        game_state.controlling = 0;
    }
}

fn check_spawn(
    game_manager: &mut std::sync::RwLockWriteGuard<'_, game_manager::GameManager>,
    game_state: &mut std::sync::RwLockWriteGuard<'_, game_state::GameState>,
) {
    if game_state.controlling == 0 {
        let _shape_none = vec![
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
        ];

        let shapes = game_manager.pieces.clone();
        let rng = &mut game_manager.rng;

        let shape = &shapes[rng.gen_range(0..shapes.len())];

        game_state.current_piece = shape.clone();

        let random = rng.gen::<i32>();

        for (y, row) in shape.layout.iter().enumerate() {
            for (x, &val) in row.iter().enumerate() {
                game_state.arena[y][x + 8] = if val == 1 { random } else { 0 };
            }
        }

        game_state.current_center = (10, 2);
        game_state.controlling = random;
        game_state.all_pieces.push((random, shape.clone()));
        game_state.has_held = false;
    }
}

fn check_move(
    game_manager: &mut std::sync::RwLockWriteGuard<'_, game_manager::GameManager>,
    game_state: &mut std::sync::RwLockWriteGuard<'_, game_state::GameState>,
) {
    let actions = process_input_buffer(game_manager, game_state);

    for action in actions {
        match action {
            Action::Rotate => rotate(game_state),
            Action::MoveRight => move_right(game_state),
            Action::MoveLeft => move_left(game_state),
            Action::MoveDown => _ = move_down(game_state, true),
            Action::Drop => drop(game_state),
            Action::Hold => hold(game_state, game_manager),
        }
    }

    if game_state.right_hold.is_pressed {
        game_state.right_hold.move_ticks += 1;
        if game_state.right_hold.move_ticks > 5 {
            move_right(game_state);
            game_state.right_hold.move_ticks = 4;
        }
    } else {
        game_state.right_hold.move_ticks = 0;
    }

    if game_state.left_hold.is_pressed {
        game_state.left_hold.move_ticks += 1;
        if game_state.left_hold.move_ticks > 5 {
            move_left(game_state);
            game_state.left_hold.move_ticks = 4;
        }
    } else {
        game_state.left_hold.move_ticks = 0;
    }

    if game_state.down_hold.is_pressed {
        game_state.down_hold.move_ticks += 1;
        if game_state.down_hold.move_ticks > 5 {
            move_down(game_state, true);
            game_state.down_hold.move_ticks = 4;
        }
    } else {
        game_state.down_hold.move_ticks = 0;
    }
}

fn move_right(game_state: &mut std::sync::RwLockWriteGuard<'_, game_state::GameState>) {
    let mut can_move = true;
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

        // move the center of the piece to the right
        game_state.current_center.0 += 1;
    }
}

fn move_left(game_state: &mut std::sync::RwLockWriteGuard<'_, game_state::GameState>) {
    let mut can_move = true;

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
        // move the center of the piece to the left
        game_state.current_center.0 -= 1;
    }
}

fn move_down(
    game_state: &mut std::sync::RwLockWriteGuard<'_, game_state::GameState>,
    forced: bool,
) -> bool {
    let mut can_move_down = true;

    if game_state.drop_ticks > 0.0 && !forced {
        game_state.drop_ticks -= game_state.drop_speed;
        return false;
    }

    // Check if the piece can move down
    for y in (0..game_state.arena.len()).rev() {
        for x in 0..game_state.arena[y].len() {
            if game_state.arena[y][x] == game_state.controlling {
                if y + 1 >= game_state.arena.len()
                    || game_state.arena[y + 1][x] != 0
                        && game_state.arena[y + 1][x] != game_state.controlling
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

    // If it can move down, move everything that is controlling down
    if can_move_down {
        for y in (0..game_state.arena.len()).rev() {
            for x in 0..game_state.arena[y].len() {
                if game_state.arena[y][x] == game_state.controlling {
                    if y + 1 < game_state.arena.len() && game_state.arena[y + 1][x] == 0 {
                        game_state.arena[y + 1][x] = game_state.controlling;
                        game_state.arena[y][x] = 0;
                    }
                }
            }
        }
        game_state.drop_ticks = 6.0;
        game_state.current_center.1 += 1;
        return true;
    } else {
        // If it can't move down, update the ground ticks
        game_state.ground_ticks += 1;
        game_state.controlling = 0;
        game_state.drop_ticks = 0.0;
        return false;
    }
}

fn drop(game_state: &mut std::sync::RwLockWriteGuard<'_, game_state::GameState>) {
    while move_down(game_state, true) {}

    // If no more moves possible, update state
    game_state.controlling = 0;
    game_state.drop_ticks = 0.0;
}

fn check_game_over(
    game_manager: &mut std::sync::RwLockWriteGuard<'_, game_manager::GameManager>,
    game_state: &mut std::sync::RwLockWriteGuard<'_, game_state::GameState>,
) {
    for y in 0..5 {
        for x in 0..game_state.arena[y].len() {
            if game_state.arena[y][x] != 0 && game_state.arena[y][x] != game_state.controlling {
                game_manager.in_game = false;
                game_manager.running = false;
                break;
            }
        }
    }
}

fn destoy_lines(game_state: &mut std::sync::RwLockWriteGuard<'_, game_state::GameState>) {
    let mut was_despawned = true;
    let mut despawned = 0;
    while was_despawned {
        was_despawned = false;
        for y in (0..game_state.arena.len()).rev() {
            let mut full = true;
            for x in 0..game_state.arena[y].len() {
                if game_state.arena[y][x] == 0 || game_state.arena[y][x] == game_state.controlling {
                    full = false;
                }
            }
            if full {
                was_despawned = true;
                despawned += 1;
                for y2 in (0..y).rev() {
                    for x in 0..game_state.arena[y2].len() {
                        game_state.arena[y2 + 1][x] = game_state.arena[y2][x];
                    }
                }
            }
        }
    }
    if despawned > 0 {
        let level = if game_state.level > 15 {
            15
        } else {
            game_state.level
        };
        game_state.score += (despawned * 100 * level) as i32;

        game_state.lines_till_next_level -= despawned as i32;
        if game_state.lines_till_next_level <= 0 {
            game_state.level += 1;
            if game_state.level < 13 {
                game_state.drop_speed = 1.0 + (game_state.level as f32 * 0.87);
            }
            game_state.lines_till_next_level = 10 + (game_state.level as f32 * 1.2) as i32;
        }
    }
}

fn process_input_buffer(
    game_manager: &mut std::sync::RwLockWriteGuard<'_, game_manager::GameManager>,
    game_state: &mut std::sync::RwLockWriteGuard<'_, game_state::GameState>,
) -> Vec<&'static Action> {
    let mut actions: Vec<&Action> = vec![];

    for (key, key_action) in game_manager.input_buffer.iter() {
        if let Some(action) = get_action(key) {
            match action {
                Action::MoveRight => {
                    if key_action == &KeyboardAction::Pressed {
                        actions.push(&Action::MoveRight);
                        game_state.right_hold.is_pressed = true;
                    } else {
                        game_state.right_hold.is_pressed = false;
                    }
                }
                Action::MoveLeft => {
                    if key_action == &KeyboardAction::Pressed {
                        actions.push(&Action::MoveLeft);
                        game_state.left_hold.is_pressed = true;
                    } else {
                        game_state.left_hold.is_pressed = false;
                    }
                }
                Action::MoveDown => {
                    if key_action == &KeyboardAction::Pressed {
                        actions.push(&Action::MoveDown);
                        game_state.down_hold.is_pressed = true;
                    } else {
                        game_state.down_hold.is_pressed = false;
                    }
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
        _ => None,
    }
}

fn rotate(game_state: &mut std::sync::RwLockWriteGuard<'_, game_state::GameState>) {
    // Remove the current piece from the arena
    let (center_x, center_y) = game_state.current_center;
    let controlling_id = game_state.controlling;
    let mut matrix = game_state.current_piece.layout.clone();

    let block = match game_state
        .all_pieces
        .iter()
        .find(|&p| p.0 == controlling_id)
    {
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

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] == 1 {
                let x = j as i32 - 2;
                let y = i as i32 - 2;

                let pos_x = center_x as i32 + x;
                let pos_y = center_y as i32 + y;

                if pos_x < 0
                    || pos_x >= game_state.arena[0].len() as i32
                    || pos_y < 0
                    || pos_y >= game_state.arena.len() as i32
                    || (game_state.arena[pos_y as usize][pos_x as usize] != 0
                        && game_state.arena[pos_y as usize][pos_x as usize] != controlling_id)
                {
                    return;
                }
            }
        }
    }

    // go over each row in arena
    for y in 0..game_state.arena.len() {
        // go over each column in arena
        for x in 0..game_state.arena[y].len() {
            if game_state.arena[y][x] == controlling_id {
                game_state.arena[y][x] = 0;
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

                game_state.arena[pos_y as usize][pos_x as usize] = controlling_id;
            }
        }
    }

    // Update the current piece and center
    game_state.current_piece.layout = matrix;
}

fn hold(game_state: &mut std::sync::RwLockWriteGuard<'_, game_state::GameState>, game_manager: &mut std::sync::RwLockWriteGuard<'_, game_manager::GameManager>) {
    if !game_state.has_held {
        game_state.has_held = true;
    } else {
        return;
    }
    let held_piece = game_state.held_piece.clone();
    let current_piece = game_state.current_piece.clone();

    for y in 0..game_state.arena.len() {
        for x in 0..game_state.arena[y].len() {
            if game_state.arena[y][x] == game_state.controlling {
                game_state.arena[y][x] = 0;
            }
        }
    }

    // check if something is held
    if held_piece.layout.len() == 0 {
        game_state.held_piece = current_piece;
        game_state.controlling = 0;
    } else {

        // create a new controlling id for the held piece
        let random = game_manager.rng.gen::<i32>();


        // spawn the held piece
        for (y, row) in held_piece.layout.iter().enumerate() {
            for (x, &val) in row.iter().enumerate() {
                game_state.arena[y][x + 8] = if val == 1 { random } else { 0 };
            }
        }

        game_state.controlling = random;

        game_state.all_pieces.push((random, held_piece.clone()));
        game_state.current_piece = held_piece.clone();
        game_state.held_piece = current_piece;

        game_state.current_center = (10, 2);
    }
}
