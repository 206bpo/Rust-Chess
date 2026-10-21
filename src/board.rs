use crate::Color;
use crate::pieces::{Piece, PieceTypes};
use core::fmt;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Pos {
    pub y: i8,
    pub x: i8,
}

impl Pos {
    pub fn new(y: i8, x: i8) -> Pos {
        Pos { y, x }
    }
    pub fn is_in_bounds(&self) -> bool {
        self.y < 8 && self.y >= 0 && self.x < 8 && self.x >= 0
    }
}

#[derive(Copy, Clone, Debug)]
pub enum FieldState {
    Occupied(Piece),
    Empty,
}

// pub struct Square {
//     position: Pos,
//     state: FieldState
// }

pub struct Board {
    pub field: [[FieldState; 8]; 8],
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
        let x = position.x as usize;
        let y = position.y as usize;
        self.field[y][x] = FieldState::Occupied(piece);
    }
    pub fn get_state_at_pos(&self, position: Pos) -> FieldState {
        let x = position.x as usize;
        let y = position.y as usize;
        self.field[y][x]
    }
    fn set_state_at_pos(&mut self, position: Pos, new_state: FieldState) {
        if !position.is_in_bounds() {
            return;
        };
        self.field[position.y as usize][position.x as usize] = new_state;
    }
    pub fn move_piece(&mut self, from: Pos, to: Pos) {
        let temp = self.get_state_at_pos(from);
        self.set_state_at_pos(from, FieldState::Empty);
        self.set_state_at_pos(to, temp);
    }
    pub fn draw(&self) {
        print!("{}", self);
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "   A B C D E F G H\n")?;
        for (i, y) in self.field.iter().enumerate() {
            write!(f, "{}  ", i + 1)?;
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
            write!(f, "  {}", 8 - i)?;
            writeln!(f)?;
        }
        writeln!(f, "\n  A B C D E F G H")?;
        Ok(())
    }
}
impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "\nX 0 1 2 3 4 5 6 7")?;
        for (i, y) in self.field.iter().enumerate() {
            write!(f, "{} ", i)?;
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
