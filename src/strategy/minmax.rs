//! Implementation of the min max algorithm.
use super::Strategy;
use crate::configuration::{Configuration, Movement};
use crate::shmem::AtomicMove;
use std::fmt;

/// Min-Max algorithm with a given recursion depth.
pub struct MinMax(pub u8);

impl Strategy for MinMax {
    fn compute_next_move(&mut self, state: &Configuration) -> Option<Movement> {
        let mut min_opponent_value: i8 = 127; //max value
        let mut max_move : Option<Movement> = None;
        let mut current_value : i8;
        Greedy opponent;
        for mov in state.movements() {
            Configuration opponent_config = state.play(&mov);
            Option<Movement> move_opponent = opponent.compute_next_move(opponent_config);
            if move_opponent == None {
                return mov //the value of the opponent added is 0 here 
            }else{
                current_value = opponent_config.play(move_opponent.unwrap()).value();
                if current_value < min_opponent_value {
                    max_move = mov;
                    min_opponent_value = current_value;
                }
            }
        }
        return max_move;
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
