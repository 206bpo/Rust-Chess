use crate::Color;
use core::fmt;

use crate::pieces::{Piece, PieceTypes};
pub struct Board {
    pub field: [[FieldState; 8]; 8],
}

#[derive(Copy, Clone)]
pub struct Pos {
    y: usize,
    x: usize,
}

#[derive(Copy, Clone)]
pub enum FieldState {
    Occupied(Piece),
    Empty,
}

impl Pos {
    pub fn new(y: usize, x: usize) -> Pos {
        Pos { y, x }
    }
}

impl Board {
    pub fn new_empty() -> Board {
        Board {
            field: [[FieldState::Empty; 8]; 8],
        }
    }
    pub fn new_ready_to_play() -> Board {
        let mut board = Board::new_empty();
        // Pawns
        for i in 0..8 {
            board.add_piece(
                Piece::new(PieceTypes::PAWN, Color::BLACK),
                Pos { y: 1, x: i },
            );
            board.add_piece(
                Piece::new(PieceTypes::PAWN, Color::WHITE),
                Pos { y: 6, x: i },
            );
        }
        // Rooks
        let rook_white = Piece::new(PieceTypes::ROOK, Color::WHITE);
        let rook_black = Piece::new(PieceTypes::ROOK, Color::BLACK);
        board.add_piece(rook_white, Pos::new(7, 0));
        board.add_piece(rook_white, Pos::new(7, 7));
        board.add_piece(rook_black, Pos::new(0, 0));
        board.add_piece(rook_black, Pos::new(0, 7));
        // Knights
        let knight_white = Piece::new(PieceTypes::KNIGHT, Color::WHITE);
        let knight_black = Piece::new(PieceTypes::KNIGHT, Color::BLACK);
        board.add_piece(knight_white, Pos::new(7, 1));
        board.add_piece(knight_white, Pos::new(7, 6));
        board.add_piece(knight_black, Pos::new(0, 1));
        board.add_piece(knight_black, Pos::new(0, 6));
        // Bishops
        let bishop_white = Piece::new(PieceTypes::BISHOP, Color::WHITE);
        let bishop_black = Piece::new(PieceTypes::BISHOP, Color::BLACK);
        board.add_piece(bishop_white, Pos::new(7, 2));
        board.add_piece(bishop_white, Pos::new(7, 5));
        board.add_piece(bishop_black, Pos::new(0, 2));
        board.add_piece(bishop_black, Pos::new(0, 5));
        // Queens
        board.add_piece(Piece::new(PieceTypes::QUEEN, Color::WHITE), Pos::new(7, 3));
        board.add_piece(Piece::new(PieceTypes::QUEEN, Color::BLACK), Pos::new(0, 3));
        // Kings
        board.add_piece(Piece::new(PieceTypes::KING, Color::WHITE), Pos::new(7, 4));
        board.add_piece(Piece::new(PieceTypes::KING, Color::BLACK), Pos::new(0, 4));
        board
    }
    pub fn add_piece(&mut self, piece: Piece, position: Pos) {
        let x = position.x;
        let y = position.y;
        self.field[y][x] = FieldState::Occupied(piece);
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "  A B C D E F G H")?;
        for (i, y) in self.field.iter().enumerate() {
            write!(f, "{} ", i + 1)?;
            for x in y {
                write!(
                    f,
                    "{} ",
                    match x {
                        FieldState::Occupied(p) => p.get_display(),
                        _ => '0',
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
