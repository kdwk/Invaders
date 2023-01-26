use std::time::Duration;

use rusty_time::Timer;
use crate::frame::{Drawable, Frame};

pub struct Shot {
    pub x: usize,
    pub y: usize,
    pub exploding: bool,
    timer: Timer,
}

impl Shot {
    pub fn init(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            exploding: false,
            timer: Timer::from_millis(50),
        }
    }
    pub fn update(&mut self, delta: Duration){
        self.timer.update(delta);
        if self.timer.ready && !self.exploding {
            if self.y > 0 { // If the shot hasn't reached the top yet
                self.y -= 1;
            }
            self.timer.reset();
        }
    }
    pub fn explode(&mut self){
        self.exploding = true;
        self.timer = Timer::from_millis(250); // 50 ms is not enough to see a timer explode, reassign timer to a timer of longer duration
    }
    pub fn is_dead(&self) -> bool {
        (self.exploding && self.timer.ready) | (self.y == 0)
    }
}

impl Drawable for Shot {
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] = if self.exploding {"*"} else {"|"}
    }
}