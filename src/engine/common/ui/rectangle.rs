use raylib::prelude::{RaylibDraw, RaylibDrawHandle};

pub fn rectangle(
    d: &mut RaylibDrawHandle,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    color: raylib::color::Color,
    outline_color: raylib::color::Color,
) {
    let rect_x = x - width / 2;
    let rect_y = y - height / 2;

    d.draw_rectangle(rect_x, rect_y, width, height, color);
    d.draw_rectangle_lines(rect_x, rect_y, width, height, outline_color);
}
