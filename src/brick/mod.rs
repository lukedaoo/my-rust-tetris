use self::{brick_factory::BrickFactory, brick_type::BrickType};
use crate::{
    common::{Coord, Direction},
    config::Config,
};
use raylib::prelude::{Color, RaylibDraw, RaylibDrawHandle};

pub mod brick_factory;
pub mod brick_type;
pub mod collisions;

#[derive(Clone)]
pub struct Brick {
    coords: Vec<Coord>,
    color: Color,
}

impl Brick {
    pub fn random() -> Brick {
        let brick_type = BrickFactory::random_brick_type();
        let start_coords = BrickType::get_start_coords(brick_type);

        Brick::spawn(start_coords, brick_type)
    }

    pub fn spawn_by_brick_type(brick_type: BrickType) -> Brick {
        Brick::spawn(BrickType::get_start_coords(brick_type), brick_type)
    }

    pub fn spawn(start_coords: Coord, brick_type: BrickType) -> Brick {
        let (ref_coords, color) = BrickFactory::get_brick_meta_data(brick_type);
        let new_coords = ref_coords
            .iter()
            .map(|coord| {
                let dx: i32 = coord.x as i32 - ref_coords[0].x as i32;
                let dy: i32 = coord.y as i32 - ref_coords[0].y as i32;
                Coord::new(
                    (start_coords.x as i32 + dx) as u32,
                    (start_coords.y as i32 + dy) as u32,
                )
            })
            .collect();

        Brick {
            coords: new_coords,
            color,
        }
    }
}

impl Brick {
    pub fn render(&self, drawable: &mut RaylibDrawHandle, config: &Config) {
        let grid_size = config.grid_size();

        let dx = grid_size.acutal_width as u32 / grid_size.horizontal_size;
        let dy = *config.height() as u32 / grid_size.vertical_size;

        for (_, coord) in self.coords.iter().enumerate() {
            if coord.y >= grid_size.vertical_size {
                continue;
            }
            let start_pos = grid_size.grid_canvas_l as u32 + coord.x * dx;
            let end_pos = config.height() - (coord.y + 1) * dy + grid_size.margin_top as u32;

            drawable.draw_rectangle(
                start_pos as i32,
                end_pos as i32,
                dx as i32,
                dy as i32,
                self.color,
            );
        }
    }

    pub fn move_by(&mut self, (x, y): (i32, i32)) {
        // Moves all real coords
        self.coords.iter_mut().for_each(|c| {
            c.x = (c.x as i32 + x) as u32;
            c.y = (c.y as i32 + y) as u32;
        });
    }

    pub fn move_by_direction(&mut self, dir: Direction) {
        self.move_by(Direction::get_dxdy(dir))
    }

    pub fn within_boundary(&self, (x, y): (i32, i32), (h_size, v_size): (i32, i32)) -> bool {
        for coord in self.coords.iter() {
            let safe_for_x = (0..h_size as i32).contains(&(coord.x as i32 + x));
            let safe_for_y = (0..(v_size + 4) as i32).contains(&(coord.y as i32 + y));

            if !safe_for_x || !safe_for_y {
                return false;
            }
        }

        true
    }
}
