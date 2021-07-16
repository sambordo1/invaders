use std::time::Duration;

use crate::{NUM_COLS, NUM_ROWS, frame::{Drawable, Frame}, shot::Shot};

pub struct Player {
    x: usize,
    y: usize,
    shots: Vec<Shot>,
}

impl Player { // implement a Player
    pub fn new() -> Self { // make a Player function
        Self {
            x: NUM_COLS / 2, // put us roughly in the middle of screen
            y: NUM_ROWS -1, // last playable row
            shots: Vec::new(), // empty vector of shots
        }
    }
    pub fn move_left(&mut self){ // move player left
        if self.x > 0 { 
            self.x -= 1;
        }
    }
    pub fn move_right(&mut self){ // move player right
        if self.x < NUM_COLS - 1 {
            self.x += 1;
        }
    }
    pub fn shoot(&mut self) -> bool { // shoot a shot
        if self.shots.len() < 2 { // if we have less than 2 shots, shoot
            self.shots.push(Shot::new(self.x, self.y - 1)); // add a shot
            true // return true
        } else { 
            false // return false
        }    
    }
    pub fn update(&mut self, delta: Duration) { // update player
        for shot in self.shots.iter_mut() { // for each shot
            shot.update(delta); // update shot
        }
        self.shots.retain(|shot| !shot.dead()); // remove dead shots
    }
}

impl Drawable for Player { // Draw our player into the frame 
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] = "A"; // set that as a space ship character or "A"
        for shot in self.shots.iter() { // for each shot
            shot.draw(frame); // draw the shot
        }
    }
}