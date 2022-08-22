use crate::common::Coord;

#[derive(Copy, Clone)]
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
            BrickType::O => Coord::new(START_AT_X, START_AT_Y),
            BrickType::I => Coord::new(START_AT_X, START_AT_Y),
            BrickType::T => Coord::new(START_AT_X + 1, START_AT_Y),
            BrickType::L => Coord::new(START_AT_X + 1, START_AT_Y),
            BrickType::J => Coord::new(START_AT_X + 1, START_AT_Y),
            BrickType::S => Coord::new(START_AT_X + 1, START_AT_Y),
            BrickType::Z => Coord::new(START_AT_X + 1, START_AT_Y),
        }
    }
}
