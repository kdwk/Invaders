use std::error::Error;
use rusty_audio::Audio;
use rusty_timer::Timer;
use crossterm::terminal;
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::cursor::{Show, Hide};
use crossterm::ExecutableCommand;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();
    audio.add("explode", "data/sound/explode.wav");
    audio.add("lose", "data/sound/lose.wav");
    audio.add("move", "data/sound/move.wav");
    audio.add("pew", "data/sound/pew.wav");
    audio.add("startup", "data/sound/startup.wav");
    audio.add("win", "data/sound/win.wav");
    audio.play("startup");

    //Terminal
    let mut stdout = io::stdout();
    //terminal is from crossterm
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;


    //Cleanup
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
