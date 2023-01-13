use crate::{NUM_COLS, NUM_ROWS};

pub type Frame = Vec<Vec<& 'static str>>;

pub fn new_frame() -> Frame {
    /*
    let mut frame = Vec::with_capacity(NUM_COLS);
    
    for _ in 0..NUM_COLS {
        // let mut col = Vec::with_capacity(NUM_ROWS);
        // for _ in 0..NUM_ROWS{
        //     col.push(" ");
        // }
        // cols.push(col);
        let mut col = (0..NUM_ROWS).map(|_| " ").collect::<Vec<&str>>();
        frame.push(col);
    } */
    let mut frame = (0..NUM_COLS).map(|_| (0..NUM_ROWS).map(|_| " ").collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();
    frame
}

pub trait Drawable {
    fn draw(&self, frame: &mut Frame);
}