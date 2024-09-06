use super::{events::events::UPDATE_EVENT, listeners::lib::register_events};

pub struct RaylibState {
    pub rl: raylib::RaylibHandle,
    pub thread: raylib::RaylibThread,
}

pub static mut RAYLIB_STATE: Option<RaylibState> = None;

pub fn start() {
    register_events();

    let (rl, thread) = raylib::init()
        .size(1600, 900)
        .resizable()
        .title("Tetris")
        .build();

    unsafe {
        RAYLIB_STATE = Some(RaylibState {
            rl,
            thread,
        });
    }

    unsafe { RAYLIB_STATE.as_mut().unwrap().rl.set_target_fps(60) };

    while !unsafe { RAYLIB_STATE.as_mut().unwrap().rl.window_should_close() } {
        UPDATE_EVENT.call();
    }
}
