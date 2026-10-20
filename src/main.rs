mod board;
mod input;
pub mod pieces;
use std::io;

use crate::pieces::Color;
use board::Board;
use input::MoveInstruction;

fn main() {
    let board: Board = Board::new_ready_to_play();
    println!("{}", board);
    loop {
        next_player_move(&board);
    }
}

fn next_player_move(board: &Board) {
    let input = get_player_input();
    let instruction: MoveInstruction =
        MoveInstruction::from_player_input(input).expect("can't parese instruction");
    let state_from = board.get_state_at_pos(instruction.from);
    let state_to = board.get_state_at_pos(instruction.to);
    dbg!(state_from, state_to);
}

fn get_player_input() -> String {
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Something went wrong while reading line");
    input
}
