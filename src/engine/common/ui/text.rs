use raylib::prelude::*;

pub fn text(
    d: &mut RaylibDrawHandle,
    width: i32,
    height: i32,
    x: i32,
    y: i32,
    color: Color,
    text: String,
    font_size: i32,
) {
    let text_width = d.measure_text(&text, font_size);

    let text_height = font_size;
    let text_x = x + (width - text_width) / 2;
    let text_y = y + (height - text_height) / 2;
    d.draw_text(&text, text_x, text_y, font_size, color);
}
