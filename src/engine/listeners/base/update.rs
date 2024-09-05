use std::time::{Duration, Instant};

use crate::engine::{events::events::{RENDER_EVENT, TICK_EVENT}, lib::GAME_STATE};

pub fn on_update() {
    RENDER_EVENT.call();

    do_tick();
    
}

pub fn do_tick() {
    let now = Instant::now();
    let delta_time = now.duration_since(
        unsafe { GAME_STATE
            .last_update }
    );
    unsafe { GAME_STATE.last_update = now };
    
    unsafe {
        GAME_STATE.tick_accumulator += delta_time;   
    }

    // call tick event 20 times per second
    while unsafe { GAME_STATE.tick_accumulator } >= Duration::from_millis(50) {
        unsafe { GAME_STATE.tick_accumulator -= Duration::from_millis(50) };
         TICK_EVENT.call();
    }
}