use event_listener_primitives::HandlerId;
use once_cell::sync::Lazy;
use std::sync::Mutex;

use crate::engine::{events::events::{BUTTON_EVENT, RENDER_EVENT, START_GAME_EVENT, TICK_EVENT, UPDATE_EVENT}, managers::{game_manager::{write_game_manager_in_game, write_game_manager_input_buffer, write_game_manager_running, write_game_manager_should_quit, GameData}, game_state::{write_game_state, write_game_state_arena, write_game_state_controlling, write_game_state_game_data, GameState}}};

use super::base::{render::on_render, tick::on_tick, update::on_update};

static EVENT_HANDLES: Lazy<Mutex<Vec<HandlerId>>> = Lazy::new(|| Mutex::new(Vec::new()));

pub fn register_events() {
    register(UPDATE_EVENT.on_event(on_update));
    register(TICK_EVENT.on_event(on_tick));
    register(RENDER_EVENT.on_event(on_render));
    register(BUTTON_EVENT.on_event(handle_button));
    register(START_GAME_EVENT.on_event(|| {
        write_game_state(GameState::new());
        write_game_manager_in_game(true);
        write_game_manager_running(true);
    }));
}

pub fn register(handler: HandlerId) {
    EVENT_HANDLES.lock().unwrap().push(handler);
}


pub fn handle_button(test: String) {
    
    // create a switch statement to handle the different button events
    match test.as_str() {
        "com.catsonluna.revris.button.play" => {
            START_GAME_EVENT.call();
        }
        "com.catsonluna.revris.button.quit" => {
            write_game_manager_should_quit(true);
        }
        "com.catsonluna.revris.button.restart" => {
            write_game_manager_running(false);
            START_GAME_EVENT.call();
        }
        "com.catsonluna.revris.button.resume" => {
            write_game_manager_running(true);
        }
        "com.catsonluna.revris.button.main_menu" => {
            write_game_manager_running(false);
            write_game_manager_in_game(false);

        }
        _ => {
            println!("Unknown: {}", test);
        }
    }

}