use std::ffi::CString;

use raylib::{color::Color, math::rrect, prelude::RaylibDraw, rgui::{IntoCStr, RaylibDrawGui}};

use crate::engine::{
    common::ui,
    lib::RAYLIB_STATE,
    listeners::base::render::render::{get_scaling_factors, scaled_value}, managers::{game_manager::{self, Block}, game_statics::read_game_statics},
};

pub fn render_create_shape() {
    let mut state = RAYLIB_STATE.lock().unwrap();
    if let Some(ref mut raylib_state) = *state {
        let mut d = raylib_state.rl.begin_drawing(&raylib_state.thread);
        let (scale_x, scale_y) = get_scaling_factors(&d);

        // if shape is empty fill with 0
        let game_manager = game_manager::read_game_manager();
        if game_manager.custom_block.layout.len() == 0 {
            let shape = vec![vec![0; 5]; 5];
            let block: Block = Block {
                layout: shape,
                color: Color::AQUA,
                can_rotate: false,
                ..Default::default()
            };
            game_manager::write_game_manager_custom_block(block);
        }

        ui::text::text(
            &mut d,
            scaled_value(800, scale_x),
            scaled_value(116, scale_y),
            Color::BLACK,
            "Create Shape".to_string(),
            scaled_value(100, scale_y),
        );

        // create a 5x5 grid of buttons
        for y in 0..5 {
            for x in 0..5 {
                // create a grid of 30x30 buttons
                let game_manager = game_manager::read_game_manager();
                
                if ui::check_box::check_box(&mut d, 
                scaled_value(30, scale_x),
                scaled_value(30, scale_y),
                scaled_value(730 + x * 35, scale_x),    
                scaled_value(300 + y * 35, scale_y),
                Color::WHITE,
                Color::GRAY,
                Color::AQUA,
                
                if game_manager.custom_block.layout[y as usize][x as usize] == 1 {true} else {false},
                ) {
                    let mut shape = game_manager.custom_block.layout.clone();
                    shape[y as usize][x as usize] = if shape[y as usize][x as usize] == 1 {0} else {1};

                    let mut game_manager = game_manager::read_game_manager_only();
                    game_manager.custom_block.layout = shape;


                    game_manager::write_game_manager_custom_block(game_manager.custom_block.clone());
                }
            }
        }

        ui::text::text(&mut d, 
            scaled_value(600, scale_x),
            scaled_value(300, scale_y),
            Color::BLACK,
            "Rotatable?".to_string(),
            scaled_value(20, scale_y),
        );

        if ui::check_box::check_box(&mut d, 
            scaled_value(30, scale_x),
            scaled_value(30, scale_y),
            scaled_value(600, scale_x),    
            scaled_value(350, scale_y),
            Color::WHITE,
            Color::GRAY,
            Color::AQUA,
            game_manager.custom_block.can_rotate,
        ) {
            let mut block = game_manager.custom_block.clone();
            block.can_rotate = !block.can_rotate;
            game_manager::write_game_manager_custom_block(block);
        }

        ui::text::text(&mut d, 
            scaled_value(600, scale_x),
            scaled_value(440, scale_y),
            Color::BLACK,
            "Color".to_string(),
            scaled_value(20, scale_y),
        );

        let color = ui::color_picker::color_picker(&mut d, 
            scaled_value(600, scale_x),    
            scaled_value(500, scale_y),
            scaled_value(100, scale_x),
            scaled_value(100, scale_y),
            game_manager.custom_block.color,
        );

        let game_manager = game_manager::read_game_manager_only();

        let mut block = game_manager.custom_block.clone();
        block.color = color;
        game_manager::write_game_manager_custom_block(block);


        ui::button::button(
            &mut d,
            scaled_value(115, scale_x),
            scaled_value(30, scale_y),
            scaled_value(800, scale_x),
            scaled_value(500, scale_y),
            Color::WHITE,
            Color::GRAY,
            "Save".to_string(),
            scaled_value(20, scale_y),
            Color::BLACK,
            Color::BLACK,
            false,
            format!("{}.{}", read_game_statics().url, "button.save_shape".to_string()),
        );





        d.clear_background(Color::from_hex("cfcefc".as_ref()).unwrap());
    }
}
