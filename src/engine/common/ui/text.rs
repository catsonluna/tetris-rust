use raylib::prelude::*;

pub fn text(d: &mut RaylibDrawHandle, x: i32, y: i32, color: Color, text: String, font_size: i32) {
    let text_x = x - d.measure_text(&text, font_size) / 2;
    let text_y = y - font_size / 2;

    d.draw_text(&text, text_x, text_y, font_size, color);
}
