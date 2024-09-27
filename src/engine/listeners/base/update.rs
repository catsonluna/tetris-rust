use std::time::{Duration, Instant};

use raylib::ffi::KeyboardKey;

use crate::engine::{
    events::events::{RENDER_EVENT, TICK_EVENT},
    lib::RAYLIB_STATE,
    managers::{
        game_manager::{read_game_manager, write_game_manager, KeyboardAction},
        game_statics::read_game_statics,
    },
};

static USED_KEYS: [KeyboardKey; 7] = [
    KeyboardKey::KEY_RIGHT,
    KeyboardKey::KEY_LEFT,
    KeyboardKey::KEY_DOWN,
    KeyboardKey::KEY_SPACE,
    KeyboardKey::KEY_UP,
    KeyboardKey::KEY_LEFT_SHIFT,
    KeyboardKey::KEY_ESCAPE,
];

pub fn on_update() {
    RENDER_EVENT.call();
    // println!("Update");
    updated_input_buffer();
    do_tick();
}

pub fn do_tick() {
    let tickrate = (1.0 / read_game_statics().tick_rate as f32 * 1000.0) as u64;
    let now = Instant::now();
    let delta_time = now.duration_since(read_game_manager().last_update);
    write_game_manager().delta_time = delta_time.as_micros();

    write_game_manager().last_update = now;
    write_game_manager().tick_accumulator += delta_time;

    while read_game_manager().tick_accumulator >= Duration::from_millis(tickrate) {
        write_game_manager().tick_accumulator -= Duration::from_millis(tickrate);
        TICK_EVENT.call();
    }
}

pub fn updated_input_buffer() {
    let mut game_manager = write_game_manager();
    {
        let mut state = RAYLIB_STATE.lock().unwrap();
        if let Some(ref mut raylib_state) = *state {
            for key in USED_KEYS.iter() {
                if raylib_state.rl.is_key_pressed(*key) {
                    game_manager
                        .input_buffer
                        .push((*key, KeyboardAction::Pressed));
                } else if raylib_state.rl.is_key_released(*key) {
                    game_manager
                        .input_buffer
                        .push((*key, KeyboardAction::Released));
                }
            }
        }
    }
}
