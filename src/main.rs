mod board;
mod input;
pub mod pieces;
use std::io;

use crate::{board::FieldState, pieces::Color};
use board::Board;
use input::MoveInstruction;

fn main() {
    let mut board: Board = Board::new_ready_to_play();
    board.draw();
    loop {
        next_player_move(&mut board);
        board.draw();
        // dbg!(&board);
    }
}

fn next_player_move(board: &mut Board) {
    let input = get_player_input();
    let instruction: MoveInstruction =
        MoveInstruction::from_player_input(input).expect("can't parese instruction");
    let state_from = board.get_state_at_pos(instruction.from);
    let state_to = board.get_state_at_pos(instruction.to);
    let piece_from = match state_from {
        FieldState::Occupied(p) => p,
        FieldState::Empty => {
            return;
        }
    };
    let legal_moves = piece_from.get_legal_moves(instruction.from, board);
    if legal_moves.contains(&instruction.to) {
        board.move_piece(instruction.from, instruction.to);
    }
    dbg!(state_from, state_to);
    dbg!(legal_moves);
}

fn get_player_input() -> String {
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Something went wrong while reading line");
    input
}
