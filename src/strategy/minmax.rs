//! Implementation of the min max algorithm.
use super::Strategy;
use crate::configuration::{Configuration, Movement};
use crate::shmem::AtomicMove;
use std::fmt;
use crate::strategy::Greedy;

/// Min-Max algorithm with a given recursion depth.
pub struct MinMax(pub u8);

impl Strategy for MinMax {
    fn compute_next_move(&mut self, state: &Configuration) -> Option<Movement> {
        if self.0 <= 0 {
            return Greedy().compute_next_move(state);
        }
        let mut min_opponent_value: i8 = 127; //max value
        let mut best_move : Option<Movement> = None;
        let mut current_value : i8;
        for mov in state.movements() {
            let move_opponent = MinMax(self.0 - 1).compute_next_move(&state.play(&mov));
            if move_opponent == None {
                current_value = state.value_opponent(); //the value of the opponent added is 0 here 
            }else{
                current_value = state.play_opponent(&(move_opponent.unwrap())).value_opponent();
            }
            if current_value < min_opponent_value {
                best_move = Some(mov);
                min_opponent_value = current_value;
            }
        }
        return best_move;
    }
}

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Min - Max (max level: {})", self.0)
    }
}

/// Anytime min max algorithm.
/// Any time algorithms will compute until a deadline is hit and the process is killed.
/// They are therefore run in another process and communicate through shared memory.
/// This function is intended to be called from blobwar_iterative_deepening.
pub fn min_max_anytime(state: &Configuration) {
    let mut movement = AtomicMove::connect().expect("failed connecting to shmem");
    for depth in 1..100 {
        movement.store(MinMax(depth).compute_next_move(state));
    }
}
