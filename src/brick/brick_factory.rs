use crate::brick::brick_type::*;
use crate::common::Coord;
use rand::Rng;
use raylib::prelude::*;
pub struct BrickFactory;

/// Tetrominos of type J, L, S, T or Z each have 5 tests, accounting for each of the 4 indices, each with a cartesion coord
pub const JLSTZ_OFFSET_DATA: [[[i32; 2]; 4]; 5] = [
    [[0, 0], [0, 0], [0, 0], [0, 0]],
    [[0, 0], [1, 0], [0, 0], [-1, 0]],
    [[0, 0], [1, -1], [0, 0], [-1, -1]],
    [[0, 0], [0, 2], [0, 0], [0, 2]],
    [[0, 0], [1, 2], [0, 0], [-1, 2]],
];

/// Tetromino of type  has 5 tests, each with 4 indices, each with a cartesion coord
pub const I_OFFSET_DATA: [[[i32; 2]; 4]; 5] = [
    [[0, 0], [-1, 0], [-1, 1], [0, 1]],
    [[-1, 0], [0, 0], [1, 1], [0, 1]],
    [[2, 0], [0, 0], [-2, 1], [0, -1]],
    [[-1, 0], [0, 1], [1, 0], [0, -1]],
    [[2, 0], [0, -2], [-2, 0], [0, 2]],
];
pub const O_OFFSET_DATA: [[[i32; 2]; 4]; 1] = [[[0, 0], [0, -1], [-1, -1], [-1, 0]]];

impl BrickFactory {
    pub fn random_brick_type() -> BrickType {
        let mut rand = rand::thread_rng();

        match rand.gen_range(0..7) {
            0 => BrickType::O,
            1 => BrickType::I,
            2 => BrickType::T,
            3 => BrickType::L,
            4 => BrickType::J,
            5 => BrickType::S,
            6 => BrickType::Z,
            _ => {
                panic!();
            }
        }
    }

    pub fn get_brick_meta_data(brick_type: BrickType) -> (Vec<Coord>, Color) {
        let (reference_coords, color) = match brick_type {
            BrickType::O => (BrickFactory::coord_o_block(), Color::YELLOW),
            BrickType::I => (BrickFactory::coord_i_block(), Color::BLUE),
            BrickType::T => (BrickFactory::coord_t_block(), Color::PURPLE),
            BrickType::L => (BrickFactory::coord_l_block(), Color::ORANGE),
            BrickType::J => (BrickFactory::coord_j_block(), Color::DARKBLUE),
            BrickType::S => (BrickFactory::coord_s_block(), Color::GREEN),
            BrickType::Z => (BrickFactory::coord_z_block(), Color::RED),
        };

        (reference_coords, color)
    }

    pub fn get_offset_data(brick_type: BrickType) -> &'static [[[i32; 2]; 4]] {
        match brick_type {
            BrickType::J | BrickType::L | BrickType::S | BrickType::T | BrickType::Z => {
                &JLSTZ_OFFSET_DATA[..]
            }
            BrickType::I => &I_OFFSET_DATA[..],
            BrickType::O => &O_OFFSET_DATA[..],
        }
    }

    fn coord_o_block() -> Vec<Coord> {
        vec![
            Coord::new(0, 0),
            Coord::new(1, 0),
            Coord::new(0, 1),
            Coord::new(1, 1),
        ]
    }

    fn coord_i_block() -> Vec<Coord> {
        vec![
            Coord::new(1, 0),
            Coord::new(0, 0),
            Coord::new(2, 0),
            Coord::new(3, 0),
        ]
    }

    fn coord_t_block() -> Vec<Coord> {
        vec![
            Coord::new(1, 0),
            Coord::new(0, 0),
            Coord::new(1, 1),
            Coord::new(2, 0),
        ]
    }

    fn coord_l_block() -> Vec<Coord> {
        vec![
            Coord::new(1, 0),
            Coord::new(0, 0),
            Coord::new(2, 0),
            Coord::new(2, 1),
        ]
    }

    fn coord_j_block() -> Vec<Coord> {
        vec![
            Coord::new(1, 0),
            Coord::new(0, 0),
            Coord::new(2, 0),
            Coord::new(0, 1),
        ]
    }

    fn coord_s_block() -> Vec<Coord> {
        vec![
            Coord::new(1, 0),
            Coord::new(0, 0),
            Coord::new(1, 1),
            Coord::new(2, 1),
        ]
    }

    fn coord_z_block() -> Vec<Coord> {
        vec![
            Coord::new(1, 0),
            Coord::new(2, 0),
            Coord::new(0, 1),
            Coord::new(1, 1),
        ]
    }
}
