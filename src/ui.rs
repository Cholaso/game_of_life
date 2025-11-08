use macroquad::prelude::*;

pub struct Ui {
    pub font_size: f32,
    pub color: Color,
}

impl Ui {
    pub fn new() -> Self {
        Self {
            font_size: 24.0,
            color: BLACK,
        }
    }

    pub fn button(&self, label: &str, x: f32, y: f32, w: f32, h: f32) -> bool {
        let mouse = mouse_position();
        let hovered = mouse.0 > x && mouse.0 < x + w && mouse.1 > y && mouse.1 < y + h;
        let clicked = hovered && is_mouse_button_pressed(MouseButton::Left);

        draw_rectangle(x, y, w, h, if hovered { GRAY } else { DARKGRAY });
        draw_text(label, x + 10.0, y + h / 1.5, self.font_size, self.color);

        clicked
    }

}