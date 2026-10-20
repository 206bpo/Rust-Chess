mod board;
pub mod pieces;
use crate::pieces::Color;
use board::Board;

fn main() {
    let board: Board = Board::new_ready_to_play();
    println!("{}", board);
}
