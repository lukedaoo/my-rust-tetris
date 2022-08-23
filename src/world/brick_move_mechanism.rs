use crate::{brick::Brick, common::Direction};

use super::World;

impl World {
    pub fn fall_brick(&mut self) {
        match self.check_and_move(Direction::Down) {
            true => return, // moving down sucessfully
            false => {
                self.stack.push(self.current_brick.clone());
                let next_brick = Brick::random();
                // println!("spawn {:?}", next_brick.brick_type());
                self.current_brick = next_brick;

                if self.check_game_over() {
                    self.game.set_game_over();
                }
            }
        }
    }

    fn is_safe_to_move(&self, direction: Direction) -> bool {
        let dx_dy = Direction::get_dxdy(direction);

        let within_boundary = self
            .current_brick
            .within_boundary(dx_dy, self.dimension.to_tuple());

        let mut collision = false;

        if within_boundary {
            collision = Brick::will_collide_all(&self.current_brick, &self.stack, dx_dy);
        }

        within_boundary && !collision
    }

    pub fn check_and_move(&mut self, direction: Direction) -> bool {
        if !self.is_safe_to_move(direction) {
            return false;
        }
        self.current_brick.move_by_direction(direction);

        true
    }
}
