use rusty_time::Timer;

use crate::{NUM_COLS, NUM_ROWS};

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
    pub fn update(&mut self, delta: Timer::Duration) -> bool { // Boolean for 'whether the entire army moved
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
            } else if self.direction == 1 {
                let max_x = self.army.iter().map(|invader| invader.x).max().unwrap_or(NUM_COLS-1);
                if max_x == NUM_COLS-1 {
                    downwards = true;
                    self.direction = -1;
                    // TODO
                }
            }
            true
        }
        false
    }
}