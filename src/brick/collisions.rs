use std::collections::HashSet;

use crate::common::Coord;

use super::Brick;

impl Brick {
    pub fn will_collide_all(current_brick: &Brick, stack: &[Brick], (x, y): (i32, i32)) -> bool {
        for brick in stack {
            let will_collide = Brick::will_collide(current_brick, brick, (x, y));
            if will_collide {
                return true;
            }
        }
        false
    }

    fn will_collide(first_brick: &Brick, second_brick: &Brick, (dx, dy): (i32, i32)) -> bool {
        let mut coords: HashSet<Coord> = HashSet::new();

        for first_brick_coord in first_brick.coords.iter() {
            let new_coord = Coord::new(
                (first_brick_coord.x as i32 + dx) as u32,
                (first_brick_coord.y as i32 + dy) as u32,
            );

            coords.insert(new_coord);
        }

        for second_brick_coord in second_brick.coords.iter() {
            if coords.contains(second_brick_coord) {
                return true;
            }
        }

        false
    }
}
