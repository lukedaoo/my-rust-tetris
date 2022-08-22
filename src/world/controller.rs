use raylib::prelude::*;

use crate::common::Direction;

use super::World;

impl World {
    pub fn on_pressed(&mut self, rl: &mut RaylibHandle) {
        match rl.get_key_pressed() {
            Some(key) => self.handle_key_pressed(key),
            None => {}
        }
    }

    fn handle_key_pressed(&mut self, key_pressed: KeyboardKey) {
        let move_status_with_key = match key_pressed {
            KeyboardKey::KEY_LEFT => self.check_and_move(Direction::Left),
            KeyboardKey::KEY_RIGHT => self.check_and_move(Direction::Right),
            _ => false,
        };

        println!(
            "Move status when {:?} pressed: {} ",
            key_pressed, move_status_with_key
        );
    }
}
