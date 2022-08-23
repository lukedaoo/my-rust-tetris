#[derive(Clone)]
pub struct CircularNum {
    rn: u32,
    max: u32,
}

impl CircularNum {
    pub fn get_increment(&self, dx: i32) -> u32 {
        ((self.rn as i32 + dx + self.max as i32) % self.max as i32) as u32
    }

    pub fn increment(&mut self, dx: i32) {
        self.rn = ((self.rn as i32 + dx + self.max as i32) % self.max as i32) as u32;
    }

    /// Get a reference to the circular num's rn.
    pub fn rn(&self) -> &u32 {
        &self.rn
    }
}

impl Default for CircularNum {
    fn default() -> Self {
        CircularNum { rn: 0, max: 4 }
    }
}
