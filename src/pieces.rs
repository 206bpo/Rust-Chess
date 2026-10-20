#[derive(Copy, Clone)]
pub enum PieceTypes {
    PAWN,
    ROOK,
    BISHOP,
    KNIGHT,
    QUEEN,
    KING,
    EMPTY,
}

#[derive(Copy, Clone)]
pub enum Color {
    BLACK,
    WHITE,
}

#[derive(Copy, Clone)]
pub struct Piece {
    piece_type: PieceTypes,
    color: Color,
}

impl Piece {
    pub fn new(piece_type: PieceTypes, color: Color) -> Piece {
        Piece { piece_type, color }
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
