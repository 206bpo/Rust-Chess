mod board;
mod input;
pub mod pieces;

use crate::{
    board::{FieldState, Pos},
    input::PromoteInstruction,
    pieces::{Color, Piece, PieceTypes},
};
use board::Board;
use input::MoveInstruction;

fn main() {
    let mut board: Board = Board::new_empty();
    board.add_piece(
        Piece::new(pieces::PieceTypes::PAWN, Color::WHITE),
        Pos::new(1, 0),
    );
    board.draw();
    loop {
        next_player_move(&mut board);
        board.draw();
        // dbg!(&board);
    }
}

fn next_player_move(board: &mut Board) {
    let instruction: MoveInstruction =
        MoveInstruction::from_player_input().expect("can't parse instruction");
    let state_from = board.get_state_at_pos(instruction.from);
    let piece_from = match state_from {
        FieldState::Occupied(p) => p,
        FieldState::Empty => {
            return;
        }
    };
    let legal_moves = piece_from.get_legal_moves(instruction.from, board);
    if legal_moves.contains(&instruction.to) {
        if (piece_from.piece_type == PieceTypes::PAWN)
            && (instruction.to.y == 0 || instruction.to.y == 7)
        {
            dbg!("promotion detected");
            let promote_instruction: PromoteInstruction =
                match PromoteInstruction::from_player_input(instruction.to) {
                    Some(instruction) => instruction,
                    None => {
                        return;
                    }
                };
            board.move_piece(instruction.from, instruction.to);
            board.promote(promote_instruction);
        } else {
            board.move_piece(instruction.from, instruction.to);
        }
    }
}
