use raylib::prelude::*;

use crate::config::Config;
use crate::utils::Utils;

pub fn render(drawable: &mut RaylibDrawHandle, config: &Config) {
    let grid_size = config.grid_size();

    let dx = grid_size.acutal_width as u32 / grid_size.horizontal_size;
    let dy = config.height() / (grid_size.vertical_size);

    for x in (0..=grid_size.horizontal_size).into_iter() {
        let current_x = x * dx + grid_size.grid_canvas_l as u32;
        let end_y = *config.height_with_margin() - grid_size.margin_bottom as u32;
        // let end_y = *config.height();
        let start_pos = Utils::to_vector2((current_x, grid_size.margin_top as u32));
        // let start_pos = Utils::to_vector2(tuple)
        let end_pos = Utils::to_vector2((current_x, end_y));

        drawable.draw_line_ex(start_pos, end_pos, 0.5_f32, Color::LIGHTGRAY);
    }

    for y in (0..=grid_size.vertical_size).into_iter() {
        let current_y = y * dy + grid_size.margin_top as u32;

        let start_pos = Utils::to_vector2((grid_size.grid_canvas_l as u32, current_y));
        let end_pos = Utils::to_vector2((grid_size.grid_canvas_r as u32, current_y));

        drawable.draw_line_ex(start_pos, end_pos, 0.5_f32, Color::LIGHTGRAY);
    }
}
