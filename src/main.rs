use invaders::{frame::{self, new_frame, Drawable}, player::Player, render::render, invaders::{Army}};
use rusty_audio::Audio;
use crossterm::{event::{Event, KeyCode, self}, terminal::{self, EnterAlternateScreen, LeaveAlternateScreen}, cursor::{Show, Hide}, ExecutableCommand};
use crossbeam::{channel};
use std::{error::Error, io, thread, time::{Duration, Instant}};

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

    // Render loop in a separate thread
    let (render_tx, render_rx) = channel::unbounded();
    let render_thread = thread::spawn(move || {
        // Optimization: compare last frame, this frame, only render new stuff
        let mut last_frame = frame::new_frame();
        let mut stdout= io::stdout();
        render(&mut stdout, &last_frame, &last_frame, true);
        loop {
            let current_frame = match render_rx.recv() {
                Ok(returned_frame) => returned_frame,
                // For when the channel is closed ↓
                Err(_) => break,
            };
            render(&mut stdout, &last_frame, &current_frame, false);
            last_frame = current_frame;
        }
    });

    let mut player = Player::init();
    let mut instant = Instant::now();
    let mut army = Army::init();
    // Game loop
    'gameloop: loop {
        let delta = instant.elapsed();
        instant = Instant::now();
        let mut current_frame = new_frame(); // Needs to be mutable because we need to draw player and stuff on it
        // Handle input
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'gameloop;
                    },
                    KeyCode::Left => player.move_left(),
                    KeyCode::Right => player.move_right(),
                    KeyCode::Char(' ') | KeyCode::Enter => {
                        let shot_is_successful = player.shoot();
                        if shot_is_successful {
                            audio.play("pew");
                        }
                    },
                    _ => {}
                }
            }
        }

        // Updates
        player.update_shots(delta);
        if army.update(delta) {
            audio.play("move");
        }

        // Draw & render
        // player.draw(&mut current_frame);
        // army.draw(&mut current_frame);
        let drawables: Vec<&dyn Drawable> = vec![&player, &army];
        for drawable in drawables {
            drawable.draw(&mut current_frame);
        }
        // It'll crash the first few times since receiver is not set up: discard result ↓
        let _ = render_tx.send(current_frame);
        // Game loop will be much faster than render loop, introduce artificial delay such that we don't try to render too many frames per second
        thread::sleep(Duration::from_millis(1));

    }


    //Cleanup
    drop(render_tx); // This should theoretically automatically happen
    render_thread.join().expect("Couldn't join render_thread");
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}