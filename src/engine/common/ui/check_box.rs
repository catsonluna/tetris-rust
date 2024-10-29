use raylib::{color::Color, ffi::MouseButton, prelude::{RaylibDraw, RaylibDrawHandle}};

pub fn check_box(
    d: &mut RaylibDrawHandle,
    width: i32,
    height: i32,
    x: i32,
    y: i32,
    color: Color,
    hover_color: Color,
    active_color: Color,
    active: bool,
) -> bool {
    let button_x = x - width / 2;
    let button_y = y - height / 2;

    let mouse = d.get_mouse_position();
    let mouse_x = mouse.x as i32;
    let mouse_y = mouse.y as i32;

    if mouse_x >= button_x
        && mouse_x <= button_x + width
        && mouse_y >= button_y
        && mouse_y <= button_y + height
    {
      
        d.draw_rectangle(button_x, button_y, width, height, hover_color);


        if d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            return true;
        }
    } else {
        if active {
            d.draw_rectangle(button_x, button_y, width, height, active_color);
        } else {
            d.draw_rectangle(button_x, button_y, width, height, color);
        }
    }
    false 
}
