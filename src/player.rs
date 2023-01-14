use crate::{NUM_COLS, NUM_ROWS, frame::{self, Frame, Drawable}};

pub struct Player {
    x: usize,
    y: usize,
}

impl Player {
    pub fn init() -> Self {
        Self {
            x: NUM_COLS/2,
            y: NUM_ROWS - 1 // Bottom row (off by one)
        }
    }
    pub fn move_left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }
    pub fn move_right(&mut self) {
        if self.x < NUM_COLS-1 {
            self.x += 1;
        }
    }
}

impl Drawable for Player {
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] = "A"; // WhatdoyoumeanthisisanA, dis is _spaceship_!!
    }
}