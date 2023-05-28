use std::time::Duration;
use std::cmp::max;
use rusty_time::Timer;

use crate::{NUM_COLS, frame::Drawable, NUM_ROWS};

pub struct Invader {
    pub x: usize,
    pub y: usize,
}

pub struct Army {
    pub army: Vec<Invader>,
    move_timer: Timer,
    direction: isize,
}

impl Army {
    pub fn init() -> Self {
        let mut army = Vec::new();
        for x in 1..NUM_COLS-2 {
            for y in 1..9 {
                if x%2==0 && y%2==0 {
                    army.push(Invader { x: x, y: y });
                }
            }
        }
        // for x in 0..
        Self { army: army, move_timer: Timer::from_millis(2000), direction: 1 }
    }
    pub fn update(&mut self, delta: Duration) -> bool { // Boolean for 'whether the entire army moved
        self.move_timer.update(delta);
        if self.move_timer.ready {
            self.move_timer.reset();
            let mut downwards = false; // Should next move be downwards?
            if self.direction == -1 { // If army is currently moving to the left
                let min_x = self.army.iter().map(|invader| invader.x).min().unwrap_or(0);
                if min_x == 0 {
                    downwards = true; // Move downwards one row
                    self.direction = 1; // Then move to the right
                }
            } else if self.direction == 1 { // If army is currently moving to the right
                let max_x = self.army.iter().map(|invader| invader.x).max().unwrap_or(NUM_COLS-1);
                if max_x == NUM_COLS-1 {
                    downwards = true; // Move downwards one row
                    self.direction = -1; // Then move to the left
                }
            }
            if downwards {
                let new_duration = max(self.move_timer.duration.as_millis()-250, 250) as u64; // Make the army move faster, but not lower than 250ms
                self.move_timer = Timer::from_millis(new_duration);
                for invader in self.army.iter_mut() {
                    invader.y += 1;
                }
            } else {
                for invader in self.army.iter_mut() {
                    invader.x = ((invader.x as isize) + self.direction) as usize ;
                }
            }
            return true;
        }
        false
    }

    pub fn are_all_dead(&self) -> bool {
        self.army.is_empty()
    }

    pub fn reached_bottom(&self) -> bool {
        self.army.iter().map(|invader| invader.y).max().unwrap_or(NUM_ROWS-1) >= NUM_ROWS-1
    }
}

impl Drawable for Army {
    fn draw(&self, frame: &mut crate::frame::Frame) {
        for invader in self.army.iter() {
            frame[invader.x][invader.y] = if (self.move_timer.time_left.as_secs_f32()/self.move_timer.duration.as_secs_f32()) > 0.5
                                                {"x"} else {"+"}
        }
    }
}
