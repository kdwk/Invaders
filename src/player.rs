use std::time::Duration;

use crate::{NUM_COLS, NUM_ROWS, frame::{Frame, Drawable}, shot::Shot};

pub struct Player {
    x: usize,
    y: usize,
    shots: Vec<Shot>,
}

impl Player {
    pub fn init() -> Self {
        Self {
            x: NUM_COLS/2,
            y: NUM_ROWS - 1, // Bottom row (off by one)
            shots: Vec::new(),
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
    pub fn shoot(&mut self) -> bool { // Returns bool because there's max no of shots
        if self.shots.len() < 2 {
            self.shots.push(Shot::init(self.x, self.y-1)); // Location = same horizontal, one above
            true
        } else {
            false
        }
    }
    pub fn update_shots(&mut self, delta: Duration) {
        for shot in self.shots.iter_mut() {
            shot.update(delta);
        }
        self.shots.retain(|shot| !shot.is_dead());
    }
}

impl Drawable for Player {
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] = "A"; // WhatdoyoumeanthisisanA, dis is _spaceship_!!
        for shot in self.shots.iter() {
            shot.draw(frame);
        }
    }
}