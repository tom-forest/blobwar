//! Implementation of the min max algorithm.
use super::Strategy;
use crate::configuration::{Configuration, Movement};
use crate::shmem::AtomicMove;
use std::fmt;
use crate::strategy::Greedy;

/// Min-Max algorithm with a given recursion depth.
pub struct MinMax(pub u8);


fn minaux(state: &Configuration, prof: u8) -> (Option<Movement>, i8) {
    if prof <= 0 {
        let final_move = Greedy().compute_next_move(&state);
        if final_move == None {
            return (None, -state.value());
        }
        let final_config = state.play(&(final_move.unwrap()));
        return (final_move, final_config.value());
    }
    
    let mut min_opponent_value : i8 = 127;
    let mut best_move : Option<Movement> = None;
    let mut current_value : i8;
    let mut current_move : Option<Movement>;
    for mov in state.movements() {
        let (current_move, current_value) = minaux(&state.play(&mov), prof - 1);
        if current_value < min_opponent_value {
            min_opponent_value = current_value;
            best_move = Some(mov);
        }
    }
    return (best_move, -min_opponent_value);
}


impl Strategy for MinMax {
    fn compute_next_move(&mut self, state: &Configuration) -> Option<Movement> {
        return minaux(state, self.0).0;
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
