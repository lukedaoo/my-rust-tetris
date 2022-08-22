pub struct Game {
    is_running: bool,
    is_over: bool,
    ticks: u32, // internal game ticks
}

impl Game {
    pub fn pause(&mut self) {
        self.is_running = false;
    }

    pub fn set_game_over(&mut self) {
        self.pause();
        self.is_over = true;
    }

    pub fn is_over(&self) -> bool {
        self.is_over
    }

    pub fn is_running(&self) -> bool {
        self.is_running
    }

    pub fn increase_ticks(&mut self) {
        self.ticks += 1;
    }

    pub fn should_fall(&mut self) -> bool {
        self.ticks % 20 == 0 // 10 frames
    }
}

impl Default for Game {
    fn default() -> Self {
        Game {
            is_running: true,
            is_over: false,
            ticks: 0,
        }
    }
}
