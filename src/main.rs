extern crate blobwar;
//use blobwar::board::Board;
use blobwar::configuration::Configuration;
use blobwar::strategy::{MinMax, Greedy, Human, AlphaBeta};

fn main() {
    //let board = Board::load("x").expect("failed loading board");
    let board = Default::default();
    let mut game = Configuration::new(&board);
    game.battle(MinMax(2), Greedy());

    /*
    //This should be removed when you want to play and is used to time strategies
    for i in 0..10{
        let board = Default::default();
        let mut game = Configuration::new(&board);
        game.battle_timed(AlphaBeta(3), AlphaBeta(4));
    }
    */
    
}
