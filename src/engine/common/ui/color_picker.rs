use raylib::{color::Color, prelude::RaylibDrawHandle, rgui::RaylibDrawGui};

pub fn color_picker(
    d: &mut RaylibDrawHandle,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    color: raylib::color::Color,
) -> Color {
    let rect_x = x - width / 2;
    let rect_y = y - height / 2;

    let bounds = raylib::math::Rectangle::new(rect_x as f32, rect_y as f32, width as f32, height as f32);
    d.gui_color_picker(bounds, None, color)

}
