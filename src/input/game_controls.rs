use raylib::{prelude::KeyboardKey, RaylibHandle};

use super::controller_key::{Buffer, ControllerKey, KeyState, Repeat};

pub struct GameControls {
    control_keys: Vec<ControllerKey>,
    key_pressed_queue: Vec<KeyboardKey>,
}

impl GameControls {
    pub fn clear_queue(&mut self) {
        self.key_pressed_queue.clear();
    }

    pub fn get_key_pressed_queue(&self) -> Vec<KeyboardKey> {
        self.key_pressed_queue.clone()
    }

    pub fn update(&mut self, rl: &RaylibHandle) {
        for control_key in self.control_keys.iter_mut() {
            if control_key.update(rl) {
                self.key_pressed_queue.push(control_key.key);
            }
        }
    }
}

impl Default for GameControls {
    fn default() -> Self {
        let move_down = ControllerKey {
            key: KeyboardKey::KEY_DOWN,
            repeat: Repeat { delay: 0, rate: 4 },
            state: KeyState::default(),
            buffer: Buffer::Closed,
        };

        let move_left = ControllerKey {
            key: KeyboardKey::KEY_LEFT,
            repeat: Repeat { delay: 0, rate: 4 },
            state: KeyState::default(),
            buffer: Buffer::Closed,
        };

        let move_right = ControllerKey {
            key: KeyboardKey::KEY_RIGHT,
            repeat: Repeat { delay: 4, rate: 4 },
            state: KeyState::default(),
            buffer: Buffer::Closed,
        };

        let reset = ControllerKey {
            key: KeyboardKey::KEY_R,
            repeat: Repeat { delay: 4, rate: 0 },
            state: KeyState::default(),
            buffer: Buffer::Closed,
        };

        let pause = ControllerKey {
            key: KeyboardKey::KEY_P,
            repeat: Repeat { delay: 0, rate: 0 },
            state: KeyState::default(),
            buffer: Buffer::Closed,
        };

        let quit = ControllerKey {
            key: KeyboardKey::KEY_Q,
            repeat: Repeat { delay: 0, rate: 0 },
            state: KeyState::default(),
            buffer: Buffer::Closed,
        };

        let controls = vec![move_down, move_left, move_right, reset, pause, quit];

        GameControls {
            control_keys: controls,
            key_pressed_queue: Vec::new(),
        }
    }
}
