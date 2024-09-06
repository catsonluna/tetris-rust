use event_listener_primitives::HandlerId;
use once_cell::sync::Lazy;
use std::sync::Mutex;

use crate::engine::events::events::{RENDER_EVENT, TICK_EVENT, UPDATE_EVENT};

use super::base::{render::on_render, tick::on_tick, update::on_update};

static EVENT_HANDLES: Lazy<Mutex<Vec<HandlerId>>> = Lazy::new(|| Mutex::new(Vec::new()));

pub fn register_events() {
    register(UPDATE_EVENT.on_event(on_update));
    register(TICK_EVENT.on_event(on_tick));
    register(RENDER_EVENT.on_event(on_render));
}

pub fn register(handler: HandlerId) {
    EVENT_HANDLES.lock().unwrap().push(handler);
}
