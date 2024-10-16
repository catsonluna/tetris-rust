use event_listener_primitives::HandlerId;
use once_cell::sync::Lazy;
use std::sync::Mutex;

use crate::engine::{events::events::{BUTTON_EVENT, RENDER_EVENT, START_GAME_EVENT, TICK_EVENT, UPDATE_EVENT}, managers::{game_manager::{write_game_manager_in_game, write_game_manager_input_buffer, write_game_manager_running, write_game_manager_should_quit, GameData}, game_state::write_game_state_game_data}};

use super::base::{render::on_render, tick::on_tick, update::on_update};

static EVENT_HANDLES: Lazy<Mutex<Vec<HandlerId>>> = Lazy::new(|| Mutex::new(Vec::new()));

pub fn register_events() {
    register(UPDATE_EVENT.on_event(on_update));
    register(TICK_EVENT.on_event(on_tick));
    register(RENDER_EVENT.on_event(on_render));
    register(BUTTON_EVENT.on_event(handle_button));
    register(START_GAME_EVENT.on_event(|| {
        let rand = rand::random::<i32>();
        let mut game_data = GameData::new();
        game_data.id = rand;
        write_game_state_game_data(game_data);
        write_game_manager_in_game(true);
        write_game_manager_running(true);
        write_game_manager_input_buffer(Vec::new());
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
        _ => {
            println!("Unknown: {}", test);
        }
    }

}