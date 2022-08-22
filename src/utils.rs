use std::{thread, time::Duration};

use raylib::prelude::*;
pub struct Utils;

impl Utils {
    pub fn to_vector2(tuple: (u32, u32)) -> Vector2 {
        Vector2 {
            x: tuple.0 as f32,
            y: tuple.1 as f32,
        }
    }

    pub fn delay(ms: Duration) {
        thread::sleep(ms);
    }
}
