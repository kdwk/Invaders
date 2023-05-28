use crate::{NUM_COLS, NUM_ROWS};

pub type Frame = Vec<Vec<& 'static str>>;

pub fn new_frame() -> Frame {
    let frame = (0..NUM_COLS).map(|_| (0..NUM_ROWS).map(|_| " ").collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();
    frame
}

pub trait Drawable {
    fn draw(&self, frame: &mut Frame);
}