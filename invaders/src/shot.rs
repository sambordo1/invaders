use std::time::Duration;
use rusty_time::timer::Timer;
use crate::frame::{Drawable, Frame};

pub struct Shot {
    pub x: usize,
    pub y: usize,
    pub exploding: bool, // whether its exploding or not
    timer: Timer, // timer to keep track of movement
}

impl Shot { // implement shot logic
    pub fn new(x: usize, y: usize) -> Self {  // construct shot with new()
        Self {
            x,
            y,
            exploding: false,
            timer: Timer::from_millis(50), // laser moves upwards every 50 milliseconds
        }
        }
        pub fn update(&mut self, delta: Duration){ // update the timer
            self.timer.update(delta); // make timer start counting down by delta amount
            if self.timer.ready && !self.exploding { // if timer is ready and not exploding
                if self.y > 0 { // if the shot is not yet at the top of the screen
                    self.y -= 1; // then keep moving shot upwards
                }
                self.timer.reset(); // reset timer
            }
        }
        pub fn explode(&mut self) { // explode the shot
            self.exploding = true; // set shot's exploding flag to true
            self.timer = Timer::from_millis(250); // explosion will last for 250 milliseconds
        }
        pub fn dead(&self) -> bool { // returns true if the shot is dead
            (self.exploding && self.timer.ready) || self.y == 0 // if the shot is exploding, timer is up, or at the top of the screen
        }  
}

impl Drawable for Shot { // implement Drawable for Shot
    fn draw(&self, frame: &mut Frame) { // draw the shot
        frame[self.x][self.y] = if self.exploding{"*"} else {"|"}; // if the shot is exploding, draw *, otherwise draw |
    }
}