
use crate::engine::managers::game_manager::{self, read_game_manager, write_game_manager_save_data, write_game_manager_should_quit};

use super::{
    events::events::UPDATE_EVENT,
    listeners::lib::register_events,
    common::storage,
};
use base64::read;
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
    //1024×576
        .size(1600, 900)
        .resizable()
        .title("Revris")
        .build();

    {
        let mut state = RAYLIB_STATE.lock().unwrap();
        *state = Some(RaylibState { rl, thread });
    }

    {
        let mut state = RAYLIB_STATE.lock().unwrap();
        if let Some(ref mut raylib_state) = *state {
            raylib_state.rl.set_target_fps(120);
            raylib_state.rl.set_exit_key(None);
        }
    }

    // load save data from file
    let save_data = storage::lib::load("save.rvrs");

    // if empty, create new save data
    if save_data.is_empty() {
        // save it as the savedata struct
        // SaveData creation
        let save_data = crate::engine::managers::game_manager::SaveData::new();
        let serialized_save_data = ron::ser::to_string(&save_data).unwrap();

        // Debug the serialized form
        println!("Serialized SaveData: {:?}", serialized_save_data);

        // Save the serialized data
        storage::lib::save("save.rvrs", &serialized_save_data);
    } else {

        let save_data: crate::engine::managers::game_manager::SaveData =
            ron::de::from_str(&save_data).unwrap();

        write_game_manager_save_data(save_data);
    }
    println!("{:#?}", read_game_manager().save_data);

    

    while !read_game_manager().should_quit {
        let should_quit = {
            let state = RAYLIB_STATE.lock().unwrap();
            if let Some(ref raylib_state) = *state {
                raylib_state.rl.window_should_close()
            } else {
                false
            }
        };
        
        write_game_manager_should_quit(read_game_manager().should_quit || should_quit);

        UPDATE_EVENT.call();
    }

    {
        let mut state = RAYLIB_STATE.lock().unwrap();
        *state = None;
    }
}
