use std::fmt;

use crate::pieces::{Piece, PieceColor, PieceType};

/// A square on the board. Row 0 is black's back rank, row 7 is white's.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Pos {
    pub row: i8,
    pub col: i8,
}

impl Pos {
    pub const fn new(row: i8, col: i8) -> Pos {
        Pos { row, col }
    }

    pub fn is_in_bounds(self) -> bool {
        (0..8).contains(&self.row) && (0..8).contains(&self.col)
    }

    /// The position shifted by the given amount, or `None` if that leaves the board.
    pub fn offset(self, d_row: i8, d_col: i8) -> Option<Pos> {
        let pos = Pos::new(self.row + d_row, self.col + d_col);
        pos.is_in_bounds().then_some(pos)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Board {
    squares: [[Option<Piece>; 8]; 8],
}

impl Board {
    pub fn new_empty() -> Board {
        Board {
            squares: [[None; 8]; 8],
        }
    }

    pub fn new_ready_to_play() -> Board {
        const BACK_RANK: [PieceType; 8] = [
            PieceType::Rook,
            PieceType::Knight,
            PieceType::Bishop,
            PieceType::Queen,
            PieceType::King,
            PieceType::Bishop,
            PieceType::Knight,
            PieceType::Rook,
        ];
        let mut board = Board::new_empty();
        for (col, piece_type) in BACK_RANK.into_iter().enumerate() {
            let col = col as i8;
            let black = |t| Some(Piece::new(t, PieceColor::Black));
            let white = |t| Some(Piece::new(t, PieceColor::White));
            board.set(Pos::new(0, col), black(piece_type));
            board.set(Pos::new(1, col), black(PieceType::Pawn));
            board.set(Pos::new(6, col), white(PieceType::Pawn));
            board.set(Pos::new(7, col), white(piece_type));
        }
        board
    }

    /// Panics if `pos` is off the board.
    pub fn get(&self, pos: Pos) -> Option<Piece> {
        assert!(pos.is_in_bounds(), "position off the board: {pos:?}");
        self.squares[pos.row as usize][pos.col as usize]
    }

    /// Panics if `pos` is off the board.
    pub fn set(&mut self, pos: Pos, piece: Option<Piece>) {
        assert!(pos.is_in_bounds(), "position off the board: {pos:?}");
        self.squares[pos.row as usize][pos.col as usize] = piece;
    }

    pub fn move_piece(&mut self, from: Pos, to: Pos) {
        let piece = self.get(from);
        self.set(from, None);
        self.set(to, piece);
    }

    /// All pieces on the board together with their positions.
    pub fn pieces(&self) -> impl Iterator<Item = (Pos, Piece)> + '_ {
        self.squares.iter().enumerate().flat_map(|(row, rank)| {
            rank.iter().enumerate().filter_map(move |(col, square)| {
                square.map(|piece| (Pos::new(row as i8, col as i8), piece))
            })
        })
    }

    pub fn king_pos(&self, color: PieceColor) -> Option<Pos> {
        self.pieces()
            .find(|(_, p)| p.piece_type == PieceType::King && p.color == color)
            .map(|(pos, _)| pos)
    }

    pub fn is_in_check(&self, color: PieceColor) -> bool {
        let king = self
            .king_pos(color)
            .expect("every side must have a king on the board");
        self.pieces()
            .filter(|(_, p)| p.color != color)
            .any(|(pos, p)| p.pseudo_legal_moves(pos, self).contains(&king))
    }

    /// Moves of the piece at `from` that don't leave its own king in check.
    pub fn legal_moves(&self, from: Pos) -> Vec<Pos> {
        let Some(piece) = self.get(from) else {
            return Vec::new();
        };
        let mut moves = piece.pseudo_legal_moves(from, self);
        moves.retain(|&to| {
            let mut after = *self;
            after.move_piece(from, to);
            !after.is_in_check(piece.color)
        });
        moves
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "   A B C D E F G H\n")?;
        for (row, rank) in self.squares.iter().enumerate() {
            let label = 8 - row;
            write!(f, "{label}  ")?;
            for square in rank {
                write!(f, "{} ", square.map_or('.', Piece::symbol))?;
            }
            writeln!(f, "  {label}")?;
        }
        writeln!(f, "\n   A B C D E F G H")
    }
}
