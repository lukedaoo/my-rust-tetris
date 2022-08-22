use raylib::RaylibHandle;

pub enum KeyState {
    Init,
    Held,
}

impl Default for KeyState {
    fn default() -> Self {
        Self::Init
    }
}

pub enum Buffer {
    Opened(u32),
    Closed,
}

pub struct Repeat {
    pub delay: u32,
    pub rate: u32,
}

pub struct ControllerKey {
    pub key: raylib::consts::KeyboardKey,
    pub state: KeyState,
    pub buffer: Buffer,
    pub repeat: Repeat,
}

impl ControllerKey {
    pub fn close_buffer(&mut self) {
        self.buffer = Buffer::Closed;
    }

    pub fn open_buffer(&mut self) {
        self.buffer = Buffer::Opened(0);
    }

    pub fn increase_buffer(&mut self) {
        if let Buffer::Opened(i) = &mut self.buffer {
            *i += 1;
        }
    }

    pub fn set_state(&mut self, state: KeyState) {
        self.state = state;
    }
}

impl ControllerKey {
    pub fn update(&mut self, rl: &RaylibHandle) -> bool {
        if rl.is_key_pressed(self.key) {
            self.open_buffer();
            return true;
        }

        if let Buffer::Opened(buffer) = self.buffer {
            let res = match self.state {
                KeyState::Init => self.handle_init(buffer, rl),
                KeyState::Held => self.handle_hold(buffer, rl),
            };

            return res;
        }

        false
    }

    fn handle_init(&mut self, buffer: u32, rl: &RaylibHandle) -> bool {
        if !rl.is_key_down(self.key) {
            self.close_buffer();
            return false;
        }
        self.increase_buffer();
        if buffer > self.repeat.delay {
            self.state = KeyState::Held;
        }
        return false;
    }

    fn handle_hold(&mut self, buffer: u32, rl: &RaylibHandle) -> bool {
        if !rl.is_key_down(self.key) {
            self.set_state(KeyState::Init);
            self.close_buffer();
            return false;
        }
        self.increase_buffer();
        if buffer > self.repeat.rate {
            self.open_buffer();
            return true;
        }
        return false;
    }
}
