//! Dumb greedy algorithm.
use super::Strategy;
use crate::configuration::{Configuration, Movement};
use std::fmt;

/// Dumb algorithm.
/// Amongst all possible movements return the one which yields the configuration with the best
/// immediate value.
pub struct Greedy();

impl fmt::Display for Greedy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Greedy")
    }
}

impl Strategy for Greedy {
    fn compute_next_move(&mut self, state: &Configuration) -> Option<Movement> {
        let mut max_value: i8 = i8::MIN;
        let mut current_value : i8;
        let mut max_move : Option<Movement> = None;
        for mov in state.movements() {
            current_value = state.play(&mov).value();
            if current_value>max_value {
                max_value = current_value;
                max_move = Some(mov)
            }
        }
        return max_move;
    }
    
}
