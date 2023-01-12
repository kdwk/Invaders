use std::error::Error;
use rusty_audio::Audio;
use rusty_timer::Timer;

fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();
    audio.add("explode", "data/sound/explode.wav");
    audio.add("lose", "data/sound/lose.wav");
    audio.add("move", "data/sound/move.wav");
    audio.add("pew", "data/sound/pew.wav");
    audio.add("startup", "data/sound/startup.wav");
    audio.add("win", "data/sound/win.wav");
    audio.play("startup");

    //Cleanup
    audio.wait();
    Ok(())
}
