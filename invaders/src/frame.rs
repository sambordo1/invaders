use crate::NUM_COLS;
use crate::NUM_ROWS;

pub type Frame = Vec<Vec<&'static str>>; // an alias for vector of vectors of borrowed static string slices 

pub fn new_frame() -> Frame { // to generate new frame
    let mut cols = Vec::with_capacity(NUM_COLS); // outer vector 
    for _ in 0..NUM_COLS { // loop through every column
        let mut col = Vec::with_capacity(NUM_ROWS); // inner vector
        for _ in 0..NUM_ROWS {  // loop through rows
            col.push(" "); // blank space frame
        }
        cols.push(col); 
    }
    cols
}

pub trait Drawable {
    fn draw(&self, frame: &mut Frame); // to be Drawable, you need to implement draw method, where you draw yourself into the frame
}