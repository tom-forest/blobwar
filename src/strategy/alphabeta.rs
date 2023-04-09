//! Alpha - Beta algorithm.
use std::fmt;

use super::Strategy;
use crate::strategy::Greedy;
use crate::configuration::{Configuration, Movement};
use crate::shmem::AtomicMove;

/// Anytime alpha beta algorithm.
/// Any time algorithms will compute until a deadline is hit and the process is killed.
/// They are therefore run in another process and communicate through shared memory.
/// This function is intended to be called from blobwar_iterative_deepening.
pub fn alpha_beta_anytime(state: &Configuration) {
    let mut movement = AtomicMove::connect().expect("failed connecting to shmem");
    for depth in 1..100 {
        let chosen_movement = AlphaBeta(depth).compute_next_move(state);
        movement.store(chosen_movement);
    }
}

/// Alpha - Beta algorithm with given maximum number of recursions.
pub struct AlphaBeta(pub u8);

impl fmt::Display for AlphaBeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Alpha - Beta (max level: {})", self.0)
    }
}


fn alpaux(state: &Configuration, prof: u8, alpha: i8) -> (Option<Movement>, i8) {
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
        let (current_move, current_value) = alpaux(&state.play(&mov), prof - 1, -min_opponent_value);
        if current_value < min_opponent_value {
            min_opponent_value = current_value;
            best_move = Some(mov);
            if min_opponent_value <= alpha {
                return (best_move, -min_opponent_value);
            }
        }
    }
    return (best_move, -min_opponent_value);
}


impl Strategy for AlphaBeta {
    fn compute_next_move(&mut self, state: &Configuration) -> Option<Movement> {
        return alpaux(state, self.0, -128).0;
    }
}
