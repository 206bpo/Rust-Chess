use crate::pieces::{Piece, PieceColor, PieceTypes};
use core::fmt;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Pos {
    pub y: i8,
    pub x: i8,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ChessField {
    pub position: Pos,
    pub piece: Option<Piece>,
}

impl Pos {
    pub fn new(y: i8, x: i8) -> Pos {
        Pos { y, x }
    }
    pub fn is_in_bounds(&self) -> bool {
        self.y < 8 && self.y >= 0 && self.x < 8 && self.x >= 0
    }
}

impl ChessField {
    pub fn new(position: Pos, piece: Option<Piece>) -> ChessField {
        ChessField { position, piece }
    }
    pub fn set_piece(&mut self, piece: Option<Piece>) {
        self.piece = piece;
    }
}

// pub struct Square {
//     position: Pos,
//     state: FieldState
// }
#[derive(Clone)]
pub struct Board {
    pub field: [[ChessField; 8]; 8],
}

impl Board {
    pub fn new_empty() -> Board {
        let new_field: [[ChessField; 8]; 8] = std::array::from_fn(|y| {
            std::array::from_fn(|x| ChessField {
                position: Pos::new(y as i8, x as i8),
                piece: None,
            })
        });

        Board { field: new_field }
    }
    pub fn new_ready_to_play() -> Board {
        let mut board = Board::new_empty();
        // Pawns
        for i in 0..8 {
            board.add_piece(
                Piece::new(PieceTypes::PAWN, PieceColor::BLACK),
                Pos { y: 1, x: i },
            );
            board.add_piece(
                Piece::new(PieceTypes::PAWN, PieceColor::WHITE),
                Pos { y: 6, x: i },
            );
        }
        // Rooks
        let rook_white = Piece::new(PieceTypes::ROOK, PieceColor::WHITE);
        let rook_black = Piece::new(PieceTypes::ROOK, PieceColor::BLACK);
        board.add_piece(rook_white, Pos::new(7, 0));
        board.add_piece(rook_white, Pos::new(7, 7));
        board.add_piece(rook_black, Pos::new(0, 0));
        board.add_piece(rook_black, Pos::new(0, 7));
        // Knights
        let knight_white = Piece::new(PieceTypes::KNIGHT, PieceColor::WHITE);
        let knight_black = Piece::new(PieceTypes::KNIGHT, PieceColor::BLACK);
        board.add_piece(knight_white, Pos::new(7, 1));
        board.add_piece(knight_white, Pos::new(7, 6));
        board.add_piece(knight_black, Pos::new(0, 1));
        board.add_piece(knight_black, Pos::new(0, 6));
        // Bishops
        let bishop_white = Piece::new(PieceTypes::BISHOP, PieceColor::WHITE);
        let bishop_black = Piece::new(PieceTypes::BISHOP, PieceColor::BLACK);
        board.add_piece(bishop_white, Pos::new(7, 2));
        board.add_piece(bishop_white, Pos::new(7, 5));
        board.add_piece(bishop_black, Pos::new(0, 2));
        board.add_piece(bishop_black, Pos::new(0, 5));
        // Queens
        board.add_piece(
            Piece::new(PieceTypes::QUEEN, PieceColor::WHITE),
            Pos::new(7, 3),
        );
        board.add_piece(
            Piece::new(PieceTypes::QUEEN, PieceColor::BLACK),
            Pos::new(0, 3),
        );
        // Kings
        board.add_piece(
            Piece::new(PieceTypes::KING, PieceColor::WHITE),
            Pos::new(7, 4),
        );
        board.add_piece(
            Piece::new(PieceTypes::KING, PieceColor::BLACK),
            Pos::new(0, 4),
        );
        board
    }
    pub fn add_piece(&mut self, piece: Piece, position: Pos) {
        let x = position.x as usize;
        let y = position.y as usize;
        self.field[y][x] = ChessField {
            position,
            piece: Some(piece),
        };
    }
    pub fn get_field(&self, position: Pos) -> ChessField {
        let x = position.x as usize;
        let y = position.y as usize;
        self.field[y][x]
    }
    fn set_field(&mut self, new_state: ChessField) {
        if !new_state.position.is_in_bounds() {
            return;
        };
        self.field[new_state.position.y as usize][new_state.position.x as usize] = new_state;
    }
    pub fn move_piece(&self, field: &mut ChessField, to: Pos) {
        self.get_field(to).set_piece(field.piece);
        field.set_piece(None);
    }
    pub fn draw(&self) {
        print!("{}", self);
    }
    pub fn promote(&mut self, position: Pos, new_piece: Piece) {
        if (position.y != 0 && position.y != 7) || !position.is_in_bounds() {
            return;
        }
        self.set_field(ChessField::new(position, Some(new_piece)));
    }
    pub fn get_all_enemy_pieces_without_king(&self, color: PieceColor) -> Vec<ChessField> {
        let mut res: Vec<ChessField> = Vec::new();
        for y in self.field {
            for x in y {
                match x.piece {
                    None => {
                        continue;
                    }
                    Some(p) => {
                        if p.color != color && p.piece_type != PieceTypes::KING {
                            res.push(x);
                        }
                    }
                }
            }
        }
        res
    }
    pub fn get_all_enemy_pieces(&self, color: PieceColor) -> Vec<ChessField> {
        let mut res: Vec<ChessField> = Vec::new();
        for y in self.field {
            for x in y {
                match x.piece {
                    None => {
                        continue;
                    }
                    Some(p) => {
                        if p.color != color {
                            res.push(x);
                        }
                    }
                }
            }
        }
        res
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
                    match x.piece {
                        Some(p) => p.get_display(),
                        None => '0',
                    }
                )?;
            }
            write!(f, "  {}", 8 - i)?;
            writeln!(f)?;
        }
        writeln!(f, "\n   A B C D E F G H")?;
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
                    match x.piece {
                        Some(p) => p.get_display(),
                        None => '0',
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
