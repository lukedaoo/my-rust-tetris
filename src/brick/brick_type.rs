use crate::common::Coord;

#[derive(Debug, Copy, Clone)]
pub enum BrickType {
    O,
    I,
    T,
    L,
    J,
    S,
    Z,
}

impl BrickType {
    pub fn get_start_coords(self) -> Coord {
        const START_AT_X: u32 = 4;
        const START_AT_Y: u32 = 21; // outside the grid

        match self {
            BrickType::O | BrickType::I => Coord::new(START_AT_X, START_AT_Y),
            _ => Coord::new(START_AT_X + 1, START_AT_Y),
        }
    }

    pub fn get_start_coords_for_incoming_brick(self) -> Coord {
        const START_AT_X: u32 = 16;
        const START_AT_Y: u32 = 10;
        Coord::new(START_AT_X, START_AT_Y)
    }
}
