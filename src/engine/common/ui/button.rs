use raylib::prelude::*;

pub fn button(
    d: &mut RaylibDrawHandle,
    width: i32,
    height: i32,
    x: i32,
    y: i32,
    color: Color,
    hover_color: Color,
    text: String,
    font_size: i32,
    font_color: Color,
    hover_font_color: Color,
    disabled: bool,
) -> bool {
    let button_x = x - width / 2;
    let button_y = y - height / 2;

    // if disabled, make the button gray and have no click event
    if disabled {
        d.draw_rectangle(button_x, button_y, width, height, color);
        let text_width = d.measure_text(&text, font_size);

        let text_height = font_size;
        let text_x = button_x + (width - text_width) / 2;
        let text_y = button_y + (height - text_height) / 2;
        d.draw_text(&text, text_x, text_y, font_size, Color::GRAY);
        return false;
    }

    let mut clicked = false;

    let mouse = d.get_mouse_position();
    let mouse_x = mouse.x as i32;
    let mouse_y = mouse.y as i32;

    if mouse_x >= button_x
        && mouse_x <= button_x + width
        && mouse_y >= button_y
        && mouse_y <= button_y + height
    {
        d.draw_rectangle(button_x, button_y, width, height, hover_color);
        let text_width = d.measure_text(&text, font_size);

        let text_height = font_size;

        let text_x = button_x + (width - text_width) / 2;
        let text_y = button_y + (height - text_height) / 2;
        d.draw_text(&text, text_x, text_y, font_size, hover_font_color);
        if d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            clicked = true;
        }
    } else {
        d.draw_rectangle(button_x, button_y, width, height, color);
        let text_width = d.measure_text(&text, font_size);

        let text_height = font_size;
        let text_x = button_x + (width - text_width) / 2;
        let text_y = button_y + (height - text_height) / 2;
        d.draw_text(&text, text_x, text_y, font_size, font_color);
    }
    clicked
}
