use crate::board::Pos;

#[derive(Copy, Clone, Debug)]
pub enum PieceTypes {
    PAWN,
    ROOK,
    BISHOP,
    KNIGHT,
    QUEEN,
    KING,
    EMPTY,
}

#[derive(Copy, Clone, Debug)]
pub enum Color {
    BLACK,
    WHITE,
}

#[derive(Copy, Clone, Debug)]
pub struct Piece {
    piece_type: PieceTypes,
    color: Color,
}

impl Piece {
    pub fn new(piece_type: PieceTypes, color: Color) -> Piece {
        Piece { piece_type, color }
    }
    pub fn get_legal_moves(&self, curr_pos: Pos) -> Vec<Pos> {
        match self.piece_type {
            PieceTypes::PAWN => pawn_get_legal_moves(curr_pos),
            PieceTypes::ROOK => rook_get_legal_moves(curr_pos),
            PieceTypes::BISHOP => bishop_get_legal_moves(curr_pos),
            PieceTypes::KNIGHT => knight_get_legal_moves(curr_pos),
            PieceTypes::QUEEN => queen_get_legal_moves(curr_pos),
            PieceTypes::KING => king_get_legal_moves(curr_pos),
            PieceTypes::EMPTY => empty_get_legal_moves(curr_pos),
        }
    }
    pub fn get_display(self) -> char {
        match self.color {
            Color::WHITE => match self.piece_type {
                PieceTypes::PAWN => 'P',
                PieceTypes::ROOK => 'R',
                PieceTypes::BISHOP => 'B',
                PieceTypes::KNIGHT => 'K',
                PieceTypes::QUEEN => 'Q',
                PieceTypes::KING => 'K',
                PieceTypes::EMPTY => '0',
            },
            Color::BLACK => match self.piece_type {
                PieceTypes::PAWN => 'p',
                PieceTypes::ROOK => 'r',
                PieceTypes::BISHOP => 'b',
                PieceTypes::KNIGHT => 'k',
                PieceTypes::QUEEN => 'q',
                PieceTypes::KING => 'k',
                PieceTypes::EMPTY => '0',
            },
        }
    }
}

// Legal Moves
fn pawn_get_legal_moves(curr_pos: Pos) -> Vec<Pos> {
    unimplemented!();
}
fn rook_get_legal_moves(curr_pos: Pos) -> Vec<Pos> {
    unimplemented!();
}
fn bishop_get_legal_moves(curr_pos: Pos) -> Vec<Pos> {
    unimplemented!();
}
fn knight_get_legal_moves(curr_pos: Pos) -> Vec<Pos> {
    unimplemented!();
}
fn queen_get_legal_moves(curr_pos: Pos) -> Vec<Pos> {
    unimplemented!();
}
fn king_get_legal_moves(curr_pos: Pos) -> Vec<Pos> {
    unimplemented!();
}
fn empty_get_legal_moves(curr_pos: Pos) -> Vec<Pos> {
    vec![]
}
