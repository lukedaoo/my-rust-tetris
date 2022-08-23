use crate::{
    brick::{brick_factory::BrickFactory, rotation_direction::RotationDirection},
    world::{FocusOn, World},
};

impl World {
    pub fn rotate_current_brick_clock_wise(&mut self) {
        self.rotate_current_brick(RotationDirection::Clockwise)
    }

    pub fn rotate_current_brick_counter_clock_wise(&mut self) {
        self.rotate_current_brick(RotationDirection::CounterClockwise)
    }

    fn rotate_current_brick(&mut self, rotation_direction: RotationDirection) {
        let center_x = self.current_brick.coords()[0].x;
        let center_y = self.current_brick.coords()[0].y;

        let (next_index_diff, m) = match rotation_direction {
            RotationDirection::Clockwise => (1, [[0, -1], [1, 0]]),
            RotationDirection::CounterClockwise => (-1, [[0, 1], [-1, 0]]),
        };

        for i in 1..self.current_brick.coords().len() {
            let t = &mut self.current_brick.coords_mut()[i];

            // Get the original coords by subtracting the origin
            // e.g. (1, 1), (1, 0), etc.
            let x = t.x as i32 - center_x as i32;
            let y = t.y as i32 - center_y as i32;
            // Rotate the coords 90 degrees to the left

            let f_x = x * m[0][0] + y * m[1][0];
            let f_y = x * m[0][1] + y * m[1][1];

            // Add the coords back
            t.x = (f_x + center_x as i32) as u32;
            t.y = (f_y + center_y as i32) as u32;
        }

        let offset_data = BrickFactory::get_offset_data(self.current_brick.brick_type());
        // Try all of the 5 test cases
        for test in offset_data {
            let current_set = test[*self.current_brick.rotation_state().rn() as usize];
            let new_set = test[self
                .current_brick
                .rotation_state()
                .get_increment(next_index_diff) as usize];
            // Checkout <https://harddrop.com/wiki/SRS#How_Guideline_SRS_Really_Works> for more information on how the offset wallkicks are derived
            // Current - Next
            let dx_dy = (current_set[0] - new_set[0], current_set[1] - new_set[1]);

            // Test collisions
            // First make sure it's in boundaries
            if self.is_safe_to_move_with_dxdy(FocusOn::CurrentBrick, dx_dy) {
                // Move tetrimino
                self.current_brick.move_by(dx_dy);
                // Update indice
                self.current_brick
                    .rotation_state_mut()
                    .increment(next_index_diff);
                // Otherwise need to rotate back
                return;
            }
        }

        // Just rotate back if there is conflict, will show up as nothing happened
        // Good place to add sound as well
        self.rotate_current_brick(RotationDirection::flip(rotation_direction));
    }
}
