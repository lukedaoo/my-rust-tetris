use crate::brick::brick_factory::BrickFactory;
use crate::world::FocusOn;
use crate::{brick::Brick, common::Direction, world::World};

impl World {
    pub fn fall_brick(&mut self) {
        match self.check_and_move(FocusOn::CurrentBrick, Direction::Down) {
            true => return, // moving down sucessfully
            false => {
                self.stack.push(self.current_brick.clone());
                let next_brick = Brick::spawn_by_brick_type(self.next_brick_type);
                self.current_brick = next_brick;

                self.next_brick_type = BrickFactory::random_brick_type();

                if self.check_game_over() {
                    self.game.set_game_over();
                }
            }
        }
    }

    pub fn fall_preview_brick(&mut self) {
        self.preview_brick = self.current_brick.clone();
        loop {
            if !self.check_and_move(FocusOn::PreviewBrick, Direction::Down) {
                return;
            }
        }
    }

    pub fn force_fall_brick(&mut self) {
        self.current_brick = self.preview_brick.clone();
        self.fall_brick();
    }

    pub fn is_safe_to_move(&self, focused_on: FocusOn, direction: Direction) -> bool {
        let dx_dy = Direction::get_dxdy(direction);
        self.is_safe_to_move_with_dxdy(focused_on, dx_dy)
    }

    pub fn is_safe_to_move_with_dxdy(&self, focused_on: FocusOn, dx_dy: (i32, i32)) -> bool {
        let brick = match focused_on {
            FocusOn::CurrentBrick => self.current_brick(),
            FocusOn::PreviewBrick => self.preview_brick(),
        };
        let within_boundary = brick.within_boundary(dx_dy, self.dimension.to_tuple());

        let mut collision = false;

        if within_boundary {
            collision = Brick::will_collide_all(&brick, &self.stack, dx_dy);
        }

        within_boundary && !collision
    }

    pub fn check_and_move(&mut self, focused_on: FocusOn, direction: Direction) -> bool {
        if !self.is_safe_to_move(focused_on.clone(), direction) {
            return false;
        }
        match focused_on {
            FocusOn::CurrentBrick => {
                self.current_brick.move_by_direction(direction);
            }
            FocusOn::PreviewBrick => {
                self.preview_brick.move_by_direction(direction);
            }
        };

        true
    }
}
