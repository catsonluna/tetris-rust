use std::time::{Duration, Instant};

use crate::engine::{
    events::events::{RENDER_EVENT, TICK_EVENT},
    managers::{game_manager::{read_game_manager, write_game_manager}, game_statics::read_game_statics},
};

pub fn on_update() {
    RENDER_EVENT.call();
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
