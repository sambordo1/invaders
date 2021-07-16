use std::time::Duration;

use rusty_time::prelude::Timer;

use crate::{NUM_COLS, NUM_ROWS};

pub struct Invader { // Invader struct
    pub x: usize, // x-coordinate
    pub y: usize, // y-coordinate
}

pub struct Invaders { // the army of invaders
    pub army: Vec<Invader>, // the vector of invaders
    move_timer: Timer, // timer for the movement of the invaders
    direction: i32, // the direction of movement
}

impl Invaders { // the implementation of the Invaders struct
    pub fn new() -> Self { // create a new army of invaders
        let mut army = Vec::new(); // create a vector of invaders
        for x in 0..NUM_COLS{ // for each invader
            for y in 0..NUM_ROWS{ // for each row
                if (x > 1) // if the invader is not on the left side
                    && (x < NUM_COLS - 2) // and not on the right side
                    && (y > 0) // and not on the top side
                    && (y < 9) // and not on the bottom side
                    && (x % 2 == 0) // and only even rows
                    && (y % 2 == 0) { // and only even columns
                        army.push(Invader {x, y});
                    }
            }
        }
        Self {
            army,
            move_timer: Timer::from_millis(2000),
            direction: 1,
        }
    }
    pub fn update(&mut self, delta: Duration) -> bool {
        self.move_timer.update(delta);
        if self.move_timer.ready {
            self.move.timer.reset();
        }
    }
 }