use std::io::{Stdout, Write};
use crate::frame::Frame;
use crossterm::QueueableCommand;
use crossterm::style::{SetBackgroundColor, Color};
use crossterm::terminal::{Clear, ClearType};
use crossterm::cursor::MoveTo;

pub fn render(stdout: &mut Stdout, last_frame: &Frame, curr_frame: &Frame, force:bool){ // render whats changed with last frame and current frame and force:bool for force rendering entire frame
    if force {
        // .unwrap is for crashing if theit is an error
        stdout.queue(SetBackgroundColor(Color::Blue)).unwrap();  // clear the screen to blue to be able to see black playing field
        stdout.queue(Clear(ClearType::All)).unwrap(); // clear the screen
        stdout.queue(SetBackgroundColor(Color::Black)).unwrap(); // set background color to black playing field
    }
    // iterate through entire frame and draw whatever has changed
    for (x, col) in curr_frame.iter().enumerate(){ // for every x index of column vectors in current frame
        for (y, s) in col.iter().enumerate() { // for every y index of stream character in columns 
            if *s != last_frame[x][y] || force { // if character changed or if we're forcing rendering then, 
                stdout.queue(MoveTo(x as u16, y as u16)).unwrap(); // move to the correct location 
                print!("{}", *s); // print a single character at this location
            }
        }
    }
    stdout.flush().unwrap(); // flush all the queues 
} 