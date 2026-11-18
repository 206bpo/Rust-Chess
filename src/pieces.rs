use std::collections::HashMap;

use image::imageops::FilterType;
use macroquad::texture::{FilterMode, Texture2D};

use crate::board::{Board, Pos};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum PieceType {
    Pawn,
    Rook,
    Bishop,
    Knight,
    Queen,
    King,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum PieceColor {
    Black,
    White,
}

impl PieceColor {
    pub fn opposite(self) -> PieceColor {
        match self {
            PieceColor::Black => PieceColor::White,
            PieceColor::White => PieceColor::Black,
        }
    }

    /// Row direction this color's pawns move in.
    fn pawn_direction(self) -> i8 {
        match self {
            PieceColor::White => -1,
            PieceColor::Black => 1,
        }
    }

    fn pawn_start_row(self) -> i8 {
        match self {
            PieceColor::White => 6,
            PieceColor::Black => 1,
        }
    }
}

const STRAIGHT: [(i8, i8); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
const DIAGONAL: [(i8, i8); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
const ALL_DIRECTIONS: [(i8, i8); 8] = [
    (0, 1),
    (0, -1),
    (1, 0),
    (-1, 0),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
];
const KNIGHT_JUMPS: [(i8, i8); 8] = [
    (-2, 1),
    (-2, -1),
    (2, 1),
    (2, -1),
    (1, 2),
    (-1, 2),
    (1, -2),
    (-1, -2),
];

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: PieceColor,
}

impl Piece {
    pub fn new(piece_type: PieceType, color: PieceColor) -> Piece {
        Piece { piece_type, color }
    }

    /// Moves this piece could make from `from`, ignoring whether they leave
    /// its own king in check. Use `Board::legal_moves` for the filtered list.
    pub fn pseudo_legal_moves(self, from: Pos, board: &Board) -> Vec<Pos> {
        match self.piece_type {
            PieceType::Pawn => self.pawn_moves(from, board),
            PieceType::Rook => self.sliding_moves(from, board, &STRAIGHT),
            PieceType::Bishop => self.sliding_moves(from, board, &DIAGONAL),
            PieceType::Queen => self.sliding_moves(from, board, &ALL_DIRECTIONS),
            PieceType::Knight => self.step_moves(from, board, &KNIGHT_JUMPS),
            PieceType::King => self.step_moves(from, board, &ALL_DIRECTIONS),
        }
    }

    pub fn symbol(self) -> char {
        let symbol = match self.piece_type {
            PieceType::Pawn => 'P',
            PieceType::Rook => 'R',
            PieceType::Bishop => 'B',
            PieceType::Knight => 'N',
            PieceType::Queen => 'Q',
            PieceType::King => 'K',
        };
        match self.color {
            PieceColor::White => symbol,
            PieceColor::Black => symbol.to_ascii_lowercase(),
        }
    }

    fn pawn_moves(self, from: Pos, board: &Board) -> Vec<Pos> {
        let dir = self.color.pawn_direction();
        let mut moves = Vec::new();
        if let Some(one) = from.offset(dir, 0)
            && board.get(one).is_none()
        {
            moves.push(one);
            if from.row == self.color.pawn_start_row()
                && let Some(two) = from.offset(2 * dir, 0)
                && board.get(two).is_none()
            {
                moves.push(two);
            }
        }
        for d_col in [-1, 1] {
            if let Some(diag) = from.offset(dir, d_col)
                && board.get(diag).is_some_and(|p| p.color != self.color)
            {
                moves.push(diag);
            }
        }
        moves
    }

    /// Moves along each direction until blocked (rook, bishop, queen).
    fn sliding_moves(self, from: Pos, board: &Board, directions: &[(i8, i8)]) -> Vec<Pos> {
        let mut moves = Vec::new();
        for &(d_row, d_col) in directions {
            let mut pos = from;
            while let Some(next) = pos.offset(d_row, d_col) {
                pos = next;
                match board.get(pos) {
                    None => moves.push(pos),
                    Some(other) => {
                        if other.color != self.color {
                            moves.push(pos);
                        }
                        break;
                    }
                }
            }
        }
        moves
    }

    /// Single jumps to each offset (knight, king).
    fn step_moves(self, from: Pos, board: &Board, offsets: &[(i8, i8)]) -> Vec<Pos> {
        offsets
            .iter()
            .filter_map(|&(d_row, d_col)| from.offset(d_row, d_col))
            .filter(|&pos| board.get(pos).is_none_or(|p| p.color != self.color))
            .collect()
    }
}

pub type SpriteMap = HashMap<(PieceType, PieceColor), Texture2D>;

macro_rules! sprite {
    ($name:literal) => {
        include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/",
            $name,
            ".png"
        )) as &'static [u8]
    };
}

const SPRITES: [(PieceType, PieceColor, &[u8]); 12] = [
    (PieceType::Pawn, PieceColor::White, sprite!("pawn_white")),
    (PieceType::Pawn, PieceColor::Black, sprite!("pawn_black")),
    (PieceType::Rook, PieceColor::White, sprite!("rook_white")),
    (PieceType::Rook, PieceColor::Black, sprite!("rook_black")),
    (
        PieceType::Knight,
        PieceColor::White,
        sprite!("knight_white"),
    ),
    (
        PieceType::Knight,
        PieceColor::Black,
        sprite!("knight_black"),
    ),
    (
        PieceType::Bishop,
        PieceColor::White,
        sprite!("bishop_white"),
    ),
    (
        PieceType::Bishop,
        PieceColor::Black,
        sprite!("bishop_black"),
    ),
    (PieceType::Queen, PieceColor::White, sprite!("queen_white")),
    (PieceType::Queen, PieceColor::Black, sprite!("queen_black")),
    (PieceType::King, PieceColor::White, sprite!("king_white")),
    (PieceType::King, PieceColor::Black, sprite!("king_black")),
];

/// Decodes the embedded sprites, pre-scaled to `tile_size` for sharp downsampling.
pub fn load_all_textures(tile_size: f32) -> SpriteMap {
    let size = tile_size.floor() as u32;
    SPRITES
        .into_iter()
        .map(|(piece, color, bytes)| {
            let img = image::load_from_memory(bytes)
                .expect("embedded sprite is not a valid image")
                .to_rgba8();
            let img = image::imageops::resize(&img, size, size, FilterType::Lanczos3);
            let texture = Texture2D::from_rgba8(size as u16, size as u16, &img);
            texture.set_filter(FilterMode::Linear);
            ((piece, color), texture)
        })
        .collect()
}
