use raylib::prelude::{Color, RaylibDraw, RaylibDrawHandle};

pub struct Textable {
    pub name: String,
    pub location: (i32, i32),
    pub font_size: i32,
    pub color: Color,
}

impl Textable {
    pub fn draw(&self, text: &str, drawable: &mut RaylibDrawHandle) {
        drawable.draw_text(
            text,
            self.location.0,
            self.location.1,
            self.font_size,
            self.color,
        );
    }
}
