use std::time::{Duration, Instant};

use crate::engine::{
    events::events::{RENDER_EVENT, TICK_EVENT},
    managers::game_state::{read_game_state, write_game_state},
};

pub fn on_update() {
    RENDER_EVENT.call();
    do_tick();
}

pub fn do_tick() {
    let tickrate = (1.0 / 30.0 * 1000.0) as u64;
    let now = Instant::now();
    let delta_time = now.duration_since(read_game_state().last_update);

    write_game_state().last_update = now;
    write_game_state().tick_accumulator += delta_time;

    while read_game_state().tick_accumulator >= Duration::from_millis(tickrate) {
        write_game_state().tick_accumulator -= Duration::from_millis(tickrate);
        TICK_EVENT.call();
    }
}
