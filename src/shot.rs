use rusty_timer::Timer;
use crate::frame::Drawable;

pub struct Shot {
    pub x: usize,
    pub y: usize,
    pub exploding: bool,
    tiner: Timer,
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
}

impl Drawable for Shot {
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] = "|"
    }
}