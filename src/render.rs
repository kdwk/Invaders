use crossterm::{QueueableCommand, style::{SetBackgroundColor, Color}, terminal::{Clear, ClearType}, cursor::MoveTo};
use std::io::{Stdout, Write};
use crate::frame::Frame;

pub fn render(stdout: &mut Stdout, last_frame: &Frame, current_frame: &Frame, force: bool) {
    if force {
        stdout.queue(SetBackgroundColor(Color::Blue)).unwrap();
        stdout.queue(Clear(ClearType::All)).unwrap();
        stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
    }
    for (x, col) in current_frame.iter().enumerate() {
        for (y, char) in col.iter().enumerate() {
            if *char != last_frame[x][y] || force {
                stdout.queue(MoveTo(x as u16, y as u16)).unwrap();
                print!("{}", *char);
            }
        }
    }
    stdout.flush().unwrap();
}