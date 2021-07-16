use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use invaders::{frame::{self, Drawable, new_frame}, player::Player, render};
use rusty_audio::Audio;
use std::{
    error::Error,
    sync::mpsc::{self, Receiver},
    time::{Duration, Instant},
    {io, thread},
};



fn main() -> Result <(), Box<dyn Error>>{
    let mut audio = Audio::new();
    audio.add("explode", "explode.wav");
    audio.add("lose", "lose.wav");
    audio.add("move", "move.wav");
    audio.add("pew", "pew.wav");
    audio.add("startup", "startup.wav");
    audio.add("win", "win.wav");
    audio.add("loser" , "you-lose.wav");
    audio.add("winner", "you-win.wav");

    audio.play("winner");

    // initialize Terminal  or setup game
    let mut stdout = io::stdout(); //access to standard out
    terminal::enable_raw_mode()?; //get keyboard input as it occurs
    stdout.execute(EnterAlternateScreen)?; // alternate screen will activate when you launch, but when you exit you are right back where you were
    stdout.execute(Hide)?;  //hide curser during game


    // Render loop in a separate thread - this thread receives frames and renders them
    let (render_tx, render_rx) = mpsc::channel(); // channel to communicate with our thread
    let render_handle = thread::spawn(move || { // make a thread , use a move closure to capture end of channel that we using 
        let mut last_frame = frame::new_frame(); // initialize last frame to new empty frame
        let mut stdout = io::stdout(); // access to stdout 
        render::render(&mut stdout, &last_frame, &last_frame, true); // render entire screen once
        loop {
            let curr_frame = match render_rx.recv(){ // get current frame, match on result with ok if frame and break if error
                Ok(x) => x, 
                Err(_) => break,
            };
            render::render(&mut stdout, &last_frame, &curr_frame, false); // render frame
            last_frame = curr_frame; // set last frame to current frame 
        }
    });

    //game loop
    let mut player = Player::new(); // create player
    let mut instant = Instant::now(); // get current instant
    'gameloop: loop { 
        // Per-frame initialization 
        let delta = instant.elapsed(); // get elapsed time since last frame
        instant = Instant::now(); // set current instant to current instant
        let mut curr_frame = new_frame(); // initialize current frame to new frame
        //Input
        while event::poll(Duration::default())? { // poll for any input events, return default (0) if no event
            if let Event::Key(key_event) = event::read()? {
                match key_event.code { // match on key events key code
                    KeyCode::Left => player.move_left(), // move left if left key is pressed
                    KeyCode::Right => player.move_right(), // move right if right key is pressed
                    KeyCode::Char(' ') | KeyCode::Enter => { // fire if space is pressed or enter is pressed
                        if player.shoot() { // if we successfully shoot, play sound
                            audio.play("pew"); // play pew sound
                        }
                    }
                    KeyCode::Esc | KeyCode::Char('q') => {  // if esc or q is pressed you exit the game
                        audio.play("loser");
                        break 'gameloop; 
                    },
                    _ => {} // ignore any other key pressed
                }
            }
        }

        //Updates
        player.update(delta); // update timer 
        // Draw and render
        player.draw(&mut curr_frame);
        let _ = render_tx.send(curr_frame); // send our current frame to other thread
        thread::sleep(Duration::from_millis(1)); // add artifical sleep to limit to a thousand frames per second
    }

    // Cleanup game
    drop(render_tx); // drop transmitting side of channel
    render_handle.join().unwrap(); 
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())

}
