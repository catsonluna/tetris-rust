use raylib::prelude::{RaylibDraw, RaylibDrawHandle};

pub fn shape_creator(
    d: &mut RaylibDrawHandle,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    color: raylib::color::Color,
    outline_color: raylib::color::Color,
    mouse_x: i32,
    mouse_y: i32,
    mouse_pressed: bool,
) -> bool {
    let rect_x = x - width / 2;
    let rect_y = y - height / 2;

    d.draw_rectangle(rect_x, rect_y, width, height, color);
    d.draw_rectangle_lines(rect_x, rect_y, width, height, outline_color);

    if mouse_pressed && mouse_x >= rect_x && mouse_x <= rect_x + width && mouse_y >= rect_y && mouse_y <= rect_y + height {
        return true;
    }

    false
}
