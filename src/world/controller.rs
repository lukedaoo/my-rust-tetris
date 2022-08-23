use raylib::prelude::*;

use crate::common::Direction;

use super::World;

impl World {
    pub fn on_pressed(&mut self) {
        for key in self.game_control.get_key_pressed_queue() {
            self.handle_key_pressed(key);
        }
        self.game_control.clear_queue();
    }

    pub fn on_paused(&mut self, rl: &mut RaylibHandle) {
        match rl.get_key_pressed() {
            Some(key) => {
                // press r to resume
                if key == KeyboardKey::KEY_R {
                    self.game.resume();
                }
                // presss q to quit
                if key == KeyboardKey::KEY_Q {
                    self.game.stop();
                }
            }
            None => {}
        }
    }

    pub fn on_stopped(&mut self) {
        println!("On stopped");
        unsafe {
            raylib::ffi::CloseWindow();
        }
    }

    fn handle_key_pressed(&mut self, key_pressed: KeyboardKey) {
        match key_pressed {
            KeyboardKey::KEY_LEFT => self.check_and_move(Direction::Left),
            KeyboardKey::KEY_RIGHT => self.check_and_move(Direction::Right),
            KeyboardKey::KEY_DOWN => self.check_and_move(Direction::Down),
            KeyboardKey::KEY_R => {
                self.reset();
                true
            }
            KeyboardKey::KEY_P => {
                if !self.game.is_paused() {
                    self.game.pause();
                }
                true
            }
            // KeyboardKey::KEY_Q => {
            //     if self.game.is_paused() {
            //         self.stop();
            //     }
            //     true
            // }
            _ => false,
        };

        // println!(
        //     "Move status when {:?} pressed: {} ",
        //     key_pressed, move_status_with_key
        // );
    }
}
