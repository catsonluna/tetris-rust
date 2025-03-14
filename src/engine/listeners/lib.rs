use event_listener_primitives::HandlerId;
use once_cell::sync::Lazy;
use std::sync::Mutex;

use crate::engine::{
    common::storage,
    events::events::{
        BUTTON_EVENT, END_GAME_EVENT, RENDER_EVENT, START_GAME_EVENT, TICK_EVENT, UPDATE_EVENT,
    },
    managers::{
        game_manager::{
            self, read_game_manager, write_game_manager, write_game_manager_custom_block, write_game_manager_in_game, write_game_manager_running, write_game_manager_save_data, write_game_manager_screen, write_game_manager_should_quit, Block
        },
        game_state::{
            read_game_state, write_game_state, write_game_state_game_data,
            write_game_state_game_over, GameState,
        }, game_statics::read_game_statics,
    },
};

use super::base::{render::render::on_render, tick::on_tick, update::on_update};

static EVENT_HANDLES: Lazy<Mutex<Vec<HandlerId>>> = Lazy::new(|| Mutex::new(Vec::new()));

pub fn register_events() {
    register(UPDATE_EVENT.on_event(on_update));
    register(TICK_EVENT.on_event(on_tick));
    register(RENDER_EVENT.on_event(on_render));
    register(BUTTON_EVENT.on_event(handle_button));
    register(START_GAME_EVENT.on_event(|| {
        let rand = rand::random::<i32>();
        let mut game_state = GameState::new();
        game_state.game_data.id = rand;
        write_game_state(game_state);
        write_game_manager_in_game(true);
        write_game_manager_running(true);
        write_game_manager_screen("game".to_string());
    }));

    register(END_GAME_EVENT.on_event(|| {
        // game_state.game_over = true;
        write_game_state_game_over(true);

        let mut game_data = read_game_state().game_data.clone();
        game_data.end_time = chrono::offset::Utc::now();
        write_game_state_game_data(game_data);

        // if the score is higher, set this as the best game
        if read_game_state().game_data.score > read_game_manager().save_data.best_game.score {
            // game_manager.save_data.best_game = game_state.game_data.clone();
            let mut save_data = read_game_manager().save_data.clone();
            save_data.best_game = read_game_state().game_data.clone();
            write_game_manager_save_data(save_data);
        }

        // check if the id is in save_data.history
        let mut save_data = read_game_manager().save_data.clone();
        let mut found = false;
        for game in save_data.history.iter() {
            if game.id == read_game_state().game_data.id {
                found = true;
                break;
            }
        }

        // if the id is not in save_data.history, add it
        if !found {
            save_data.history.push(read_game_state().game_data.clone());
            write_game_manager_save_data(save_data);
        }

        // save the game data
        let serialized_save_data = ron::ser::to_string(&read_game_manager().save_data).unwrap();
        storage::lib::save("save.rvrs", &serialized_save_data);
    }));
}

pub fn register(handler: HandlerId) {
    EVENT_HANDLES.lock().unwrap().push(handler);
}

pub fn handle_button(test: String) {
    // create a switch statement to handle the different button events
    match test.as_str().replace(&format!("{}.", &read_game_statics().url), "").as_str() {
        "button.play" => {
            START_GAME_EVENT.call();
        }
        "button.quit" => {
            END_GAME_EVENT.call();
            write_game_manager_should_quit(true);
        }
        "button.restart" => {
            write_game_manager_running(false);
            START_GAME_EVENT.call();
        }
        "button.resume" => {
            write_game_manager_running(true);
        }
        "button.main_menu" => {
            END_GAME_EVENT.call();
            write_game_manager_running(false);
            write_game_manager_in_game(false);
            write_game_manager_screen("main".to_string());
        }
        "button.blocks" => {
            write_game_manager_custom_block(Block::default());
            write_game_manager_screen("create_shape".to_string());
        }
        "button.save_shape" => {
            let mut game_manager = game_manager::read_game_manager_only();
            game_manager.pieces.push(game_manager.custom_block.clone());
            write_game_manager_custom_block(Block::default());
            write_game_manager(game_manager);
            BUTTON_EVENT.call(format!("{}.{}", read_game_statics().url, "button.main_menu".to_string()));
        }
        _ => {
            println!("Unknown: {}", test);
        }
    }
}
