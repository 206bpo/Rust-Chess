use std::collections::HashMap;

use image::imageops::FilterType;
use macroquad::texture::{FilterMode, Texture2D};

use crate::board::{Board, ChessField, Pos};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum PieceTypes {
    PAWN,
    ROOK,
    BISHOP,
    KNIGHT,
    QUEEN,
    KING,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum PieceColor {
    BLACK,
    WHITE,
}

impl PieceColor {
    pub fn opposite(color: PieceColor) -> PieceColor {
        match color {
            Self::BLACK => Self::WHITE,
            Self::WHITE => Self::BLACK,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Piece {
    pub piece_type: PieceTypes,
    pub color: PieceColor,
}

impl Piece {
    pub fn new(piece_type: PieceTypes, color: PieceColor) -> Piece {
        Piece { piece_type, color }
    }
    pub fn get_legal_moves(&self, curr_pos: Pos, board: &Board) -> Vec<Pos> {
        let color = self.color;
        let mut res: Vec<Pos> = Vec::new();
        res.append(&mut match self.piece_type {
            PieceTypes::PAWN => pawn_get_legal_moves(curr_pos, color, board),
            PieceTypes::ROOK => rook_get_legal_moves(curr_pos, color, board),
            PieceTypes::BISHOP => bishop_get_legal_moves(curr_pos, color, board),
            PieceTypes::KNIGHT => knight_get_legal_moves(curr_pos, color, board),
            PieceTypes::QUEEN => queen_get_legal_moves(curr_pos, color, board),
            PieceTypes::KING => king_get_legal_moves(curr_pos, color, board),
        });
        res
    }
    pub fn get_display(self) -> char {
        match self.color {
            PieceColor::WHITE => match self.piece_type {
                PieceTypes::PAWN => 'P',
                PieceTypes::ROOK => 'R',
                PieceTypes::BISHOP => 'B',
                PieceTypes::KNIGHT => 'N',
                PieceTypes::QUEEN => 'Q',
                PieceTypes::KING => 'K',
            },
            PieceColor::BLACK => match self.piece_type {
                PieceTypes::PAWN => 'p',
                PieceTypes::ROOK => 'r',
                PieceTypes::BISHOP => 'b',
                PieceTypes::KNIGHT => 'n',
                PieceTypes::QUEEN => 'q',
                PieceTypes::KING => 'k',
            },
        }
    }
}

pub fn remove_moves_that_result_in_check(moves: &mut Vec<Pos>, field: ChessField, board: &Board) {
    let mut curr_board: Board;
    let color = field.piece.unwrap().color;
    let curr_pos = field.position;
    let mut to_pop: Vec<usize> = Vec::new();
    for (i, possible) in moves.clone().iter().enumerate() {
        curr_board = board.clone();
        curr_board.move_piece(curr_pos, *possible);
        let king_pos: Pos = curr_board.get_king_field(color);
        for curr_field in curr_board.get_all_enemy_pieces(color) {
            let curr_piece = curr_field.piece.unwrap();
            let curr_piece_moves = curr_piece.get_legal_moves(curr_field.position, &curr_board);
            if curr_piece_moves.contains(&king_pos) {
                to_pop.push(i);
                break;
            }
        }
    }
    for i in to_pop.iter().rev() {
        moves.remove(*i);
    }
}

pub fn check_for_checkamte() {
    unimplemented!()
}

// Legal Moves
fn pawn_get_legal_moves(curr_pos: Pos, color: PieceColor, board: &Board) -> Vec<Pos> {
    let mut res: Vec<Pos> = vec![];
    let curr_y = curr_pos.y;
    let curr_x = curr_pos.x;
    // get all moves a pawn generally can do
    match color {
        PieceColor::WHITE => {
            // standart one front
            let front = Pos::new(curr_y - 1, curr_x);
            if board.get_field(front).piece.is_none() {
                res.push(front);
                // double on first move
                if curr_y == 6 {
                    let two_step = Pos::new(curr_y - 2, curr_x);
                    if board.get_field(two_step).piece.is_none() {
                        res.push(two_step);
                    }
                }
            }
            // diagonals
            let diag_left = Pos::new(curr_y - 1, curr_x - 1);
            let diag_right = Pos::new(curr_y - 1, curr_x + 1);
            if diag_left.is_in_bounds()
                && let Some(piece) = board.get_field(diag_left).piece
                && piece.color != color
            {
                res.push(diag_left);
            }
            if diag_right.is_in_bounds()
                && let Some(piece) = board.get_field(diag_right).piece
                && piece.color != color
            {
                res.push(diag_right);
            }
        }
        PieceColor::BLACK => {
            // standart one front
            let front = Pos::new(curr_y + 1, curr_x);
            if board.get_field(front).piece.is_none() {
                res.push(front);
                // double on first move
                if curr_y == 1 {
                    let two_step = Pos::new(curr_y + 2, curr_x);
                    if board.get_field(two_step).piece.is_none() {
                        res.push(two_step);
                    }
                }
            }
            // diagonals
            let diag_left = Pos::new(curr_y + 1, curr_x - 1);
            let diag_right = Pos::new(curr_y + 1, curr_x + 1);
            if diag_left.is_in_bounds()
                && let Some(piece) = board.get_field(diag_left).piece
                && piece.color != color
            {
                res.push(diag_left);
            }
            if diag_right.is_in_bounds()
                && let Some(piece) = board.get_field(diag_right).piece
                && piece.color != color
            {
                res.push(diag_right);
            }
        }
    };
    res
}
fn rook_get_legal_moves(curr_pos: Pos, color: PieceColor, board: &Board) -> Vec<Pos> {
    let mut res: Vec<Pos> = vec![];
    let curr_y = curr_pos.y;
    let curr_x = curr_pos.x;
    // x-axis +
    for i in (curr_x + 1)..=7 {
        let this_pos = Pos::new(curr_y, i);
        let state = board.get_field(this_pos);
        match state.piece {
            None => res.push(this_pos),
            Some(f) => {
                if f.color != color {
                    res.push(this_pos);
                }
                break;
            }
        }
    }
    // x-axis -
    for i in (0..curr_x).rev() {
        let this_pos = Pos::new(curr_y, i);
        let state = board.get_field(this_pos);
        match state.piece {
            None => res.push(this_pos),
            Some(f) => {
                if f.color != color {
                    res.push(this_pos);
                }
                break;
            }
        }
    }
    // y-axis +
    for i in (curr_y + 1)..=7 {
        let this_pos = Pos::new(i, curr_x);
        let state = board.get_field(this_pos);
        match state.piece {
            None => res.push(this_pos),
            Some(f) => {
                if f.color != color {
                    res.push(this_pos);
                }
                break;
            }
        }
    }
    // y-axis -
    for i in (0..curr_y).rev() {
        let this_pos = Pos::new(i, curr_x);
        let state = board.get_field(this_pos);
        match state.piece {
            None => res.push(this_pos),
            Some(f) => {
                if f.color != color {
                    res.push(this_pos);
                }
                break;
            }
        }
    }

    res
}
fn bishop_get_legal_moves(curr_pos: Pos, color: PieceColor, board: &Board) -> Vec<Pos> {
    let mut res: Vec<Pos> = vec![];
    let curr_y = curr_pos.y;
    let curr_x = curr_pos.x;
    let mut this_pos: Pos;

    let mut top_right_break = false;
    let mut top_left_break = false;
    let mut bottom_right_break = false;
    let mut bottom_left_break = false;
    for i in 1..=7 {
        // top right
        if !top_right_break {
            this_pos = Pos::new(curr_y - i, curr_x + i);
            if this_pos.is_in_bounds() {
                match board.get_field(this_pos).piece {
                    None => res.push(this_pos),
                    Some(f) => {
                        if f.color != color {
                            res.push(this_pos);
                        }
                        top_right_break = true;
                    }
                }
            }
        }
        // top left
        if !top_left_break {
            this_pos = Pos::new(curr_y - i, curr_x - i);
            if this_pos.is_in_bounds() {
                match board.get_field(this_pos).piece {
                    None => res.push(this_pos),
                    Some(f) => {
                        if f.color != color {
                            res.push(this_pos);
                        }
                        top_left_break = true;
                    }
                }
            }
        }
        // bottom right
        if !bottom_right_break {
            this_pos = Pos::new(curr_y + i, curr_x + i);
            if this_pos.is_in_bounds() {
                match board.get_field(this_pos).piece {
                    None => res.push(this_pos),
                    Some(f) => {
                        if f.color != color {
                            res.push(this_pos);
                        }
                        bottom_right_break = true;
                    }
                }
            }
        }
        if !bottom_left_break {
            this_pos = Pos::new(curr_y + i, curr_x - i);
            if this_pos.is_in_bounds() {
                match board.get_field(this_pos).piece {
                    None => res.push(this_pos),
                    Some(f) => {
                        if f.color != color {
                            res.push(this_pos);
                        }
                        bottom_left_break = true;
                    }
                }
            }
        }
    }
    res
}
fn knight_get_legal_moves(curr_pos: Pos, color: PieceColor, board: &Board) -> Vec<Pos> {
    let mut res: Vec<Pos> = vec![];
    let curr_x = curr_pos.x;
    let curr_y = curr_pos.y;
    let possible_moves = vec![
        Pos::new(curr_y - 2, curr_x + 1),
        Pos::new(curr_y - 2, curr_x - 1),
        Pos::new(curr_y + 2, curr_x + 1),
        Pos::new(curr_y + 2, curr_x - 1),
        Pos::new(curr_y + 1, curr_x + 2),
        Pos::new(curr_y - 1, curr_x + 2),
        Pos::new(curr_y + 1, curr_x - 2),
        Pos::new(curr_y - 1, curr_x - 2),
    ];
    for this_pos in possible_moves {
        if this_pos.is_in_bounds() {
            match board.get_field(this_pos).piece {
                None => res.push(this_pos),
                Some(f) => {
                    if f.color != color {
                        res.push(this_pos);
                    }
                }
            }
        }
    }
    res
}
fn queen_get_legal_moves(curr_pos: Pos, color: PieceColor, board: &Board) -> Vec<Pos> {
    let mut res: Vec<Pos> = vec![];
    res.append(&mut bishop_get_legal_moves(curr_pos, color, board));
    res.append(&mut rook_get_legal_moves(curr_pos, color, board));
    res
}
fn king_get_legal_moves(curr_pos: Pos, color: PieceColor, board: &Board) -> Vec<Pos> {
    let mut res: Vec<Pos> = vec![];
    let curr_x = curr_pos.x;
    let curr_y = curr_pos.y;
    let possible_moves = vec![
        Pos::new(curr_y, curr_x + 1),
        Pos::new(curr_y, curr_x - 1),
        Pos::new(curr_y + 1, curr_x),
        Pos::new(curr_y + 1, curr_x + 1),
        Pos::new(curr_y + 1, curr_x - 1),
        Pos::new(curr_y - 1, curr_x),
        Pos::new(curr_y - 1, curr_x + 1),
        Pos::new(curr_y - 1, curr_x - 1),
    ];
    let other_pieces = board.get_all_enemy_pieces_without_king(color);
    let mut attacked_positions: Vec<Pos> = Vec::new();
    for enemy_field in other_pieces {
        attacked_positions.append(
            &mut enemy_field
                .piece
                .unwrap()
                .get_legal_moves(enemy_field.position, board),
        );
    }
    for this_pos in possible_moves {
        if this_pos.is_in_bounds() {
            match board.get_field(this_pos).piece {
                None => {
                    if !attacked_positions.contains(&this_pos) {
                        res.push(this_pos);
                    }
                }
                Some(p) => {
                    if p.color != color && !attacked_positions.contains(&this_pos) {
                        res.push(this_pos);
                    }
                }
            }
        }
    }

    res
}

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

const SPRITES: [(PieceTypes, PieceColor, &[u8]); 12] = [
    (PieceTypes::PAWN, PieceColor::WHITE, sprite!("pawn_white")),
    (PieceTypes::PAWN, PieceColor::BLACK, sprite!("pawn_black")),
    (PieceTypes::ROOK, PieceColor::WHITE, sprite!("rook_white")),
    (PieceTypes::ROOK, PieceColor::BLACK, sprite!("rook_black")),
    (
        PieceTypes::KNIGHT,
        PieceColor::WHITE,
        sprite!("knight_white"),
    ),
    (
        PieceTypes::KNIGHT,
        PieceColor::BLACK,
        sprite!("knight_black"),
    ),
    (
        PieceTypes::BISHOP,
        PieceColor::WHITE,
        sprite!("bishop_white"),
    ),
    (
        PieceTypes::BISHOP,
        PieceColor::BLACK,
        sprite!("bishop_black"),
    ),
    (PieceTypes::QUEEN, PieceColor::WHITE, sprite!("queen_white")),
    (PieceTypes::QUEEN, PieceColor::BLACK, sprite!("queen_black")),
    (PieceTypes::KING, PieceColor::WHITE, sprite!("king_white")),
    (PieceTypes::KING, PieceColor::BLACK, sprite!("king_black")),
];

pub fn load_all_textures(tile_size: f32) -> HashMap<(PieceTypes, PieceColor), Texture2D> {
    let size = tile_size.floor() as u32;
    let mut map = HashMap::new();

    for (piece, color, bytes) in SPRITES {
        let img = image::load_from_memory(bytes)
            .expect("embedded sprite is not a valid image")
            .to_rgba8();
        let img = image::imageops::resize(&img, size, size, FilterType::Lanczos3);

        let texture = Texture2D::from_rgba8(size as u16, size as u16, &img);
        texture.set_filter(FilterMode::Linear);
        map.insert((piece, color), texture);
    }
    map
}
