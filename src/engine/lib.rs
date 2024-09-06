use super::{events::events::UPDATE_EVENT, listeners::lib::register_events};
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

pub struct RaylibState {
    pub rl: raylib::RaylibHandle,
    pub thread: raylib::RaylibThread,
}

unsafe impl Sync for RaylibState {}
unsafe impl Send for RaylibState {}

lazy_static! {
    pub static ref RAYLIB_STATE: Arc<Mutex<Option<RaylibState>>> = Arc::new(Mutex::new(None));
}

pub fn start() {
    register_events();

    let (rl, thread) = raylib::init()
        .size(1600, 900)
        .resizable()
        .title("Tetris")
        .build();

    {
        let mut state = RAYLIB_STATE.lock().unwrap();
        *state = Some(RaylibState { rl, thread });
    }

    {
        let mut state = RAYLIB_STATE.lock().unwrap();
        if let Some(ref mut raylib_state) = *state {
            raylib_state.rl.set_target_fps(60);
        }
    }

    loop {
        let should_close = {
            let state = RAYLIB_STATE.lock().unwrap();
            if let Some(ref raylib_state) = *state {
                raylib_state.rl.window_should_close()
            } else {
                false
            }
        };

        if should_close {
            break;
        }

        UPDATE_EVENT.call();
    }
}
