use raylib::prelude::*;

use crate::common::Direction;

use super::{FocusOn, World};

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
            KeyboardKey::KEY_LEFT => self.check_and_move(FocusOn::CurrentBrick, Direction::Left),
            KeyboardKey::KEY_RIGHT => self.check_and_move(FocusOn::CurrentBrick, Direction::Right),
            KeyboardKey::KEY_DOWN => self.check_and_move(FocusOn::CurrentBrick, Direction::Down),
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
            KeyboardKey::KEY_Z => {
                self.rotate_current_brick_clock_wise();
                true
            }
            KeyboardKey::KEY_X => {
                self.rotate_current_brick_counter_clock_wise();
                true
            }
            KeyboardKey::KEY_SPACE => {
                self.force_fall_brick();
                true
            }
            _ => false,
        };
    }
}
