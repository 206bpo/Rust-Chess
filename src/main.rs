mod board;
mod pieces;

use macroquad::prelude::*;

use crate::board::{Board, Pos};
use crate::pieces::{Piece, PieceColor, PieceType, SpriteMap};

const PROMOTION_CHOICES: [PieceType; 4] = [
    PieceType::Queen,
    PieceType::Rook,
    PieceType::Bishop,
    PieceType::Knight,
];

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Chess"),
        window_width: 1000,
        window_height: 1000,
        window_resizable: false,
        platform: miniquad::conf::Platform {
            swap_interval: Some(1),
            ..Default::default()
        },
        ..Default::default()
    }
}

enum Selection {
    Nothing,
    Piece {
        from: Pos,
        moves: Vec<Pos>,
    },
    Promotion(Pos),
}

struct Game {
    board: Board,
    to_move: PieceColor,
    selection: Selection,
}

impl Game {
    fn new() -> Game {
        Game {
            board: Board::new_ready_to_play(),
            to_move: PieceColor::White,
            selection: Selection::Nothing,
        }
    }

    fn handle_click(&mut self, clicked: Pos) {
        match std::mem::replace(&mut self.selection, Selection::Nothing) {
            Selection::Promotion(at) => self.handle_promotion_click(at, clicked),
            Selection::Piece { from, moves } if moves.contains(&clicked) => {
                self.make_move(from, clicked)
            }
            _ => self.select(clicked),
        }
    }

    fn select(&mut self, pos: Pos) {
        if pos.is_in_bounds()
            && let Some(piece) = self.board.get(pos)
            && piece.color == self.to_move
        {
            self.selection = Selection::Piece {
                from: pos,
                moves: self.board.legal_moves(pos),
            };
        }
    }

    fn make_move(&mut self, from: Pos, to: Pos) {
        self.board.move_piece(from, to);
        let is_pawn = self
            .board
            .get(to)
            .is_some_and(|p| p.piece_type == PieceType::Pawn);
        if is_pawn && (to.row == 0 || to.row == 7) {
            self.selection = Selection::Promotion(to);
        } else {
            self.end_turn();
        }
    }

    fn handle_promotion_click(&mut self, at: Pos, clicked: Pos) {
        let choice = PROMOTION_CHOICES
            .into_iter()
            .enumerate()
            .find(|&(i, _)| promotion_menu_square(at, i) == clicked);
        match choice {
            Some((_, piece_type)) => {
                self.board
                    .set(at, Some(Piece::new(piece_type, self.to_move)));
                self.end_turn();
            }
            // clicked outside the menu: keep waiting
            None => self.selection = Selection::Promotion(at),
        }
    }

    fn end_turn(&mut self) {
        self.to_move = self.to_move.opposite();
    }

    fn draw(&self, tile_size: f32, sprites: &SpriteMap) {
        draw_board_bg(tile_size);
        for (pos, piece) in self.board.pieces() {
            draw_piece(piece, pos, tile_size, sprites);
        }
        match &self.selection {
            Selection::Nothing => {}
            Selection::Piece { moves, .. } => draw_legal_moves(moves, tile_size),
            Selection::Promotion(at) => draw_promotion_menu(*at, self.to_move, tile_size, sprites),
        }
    }
}

fn promotion_menu_square(at: Pos, index: usize) -> Pos {
    let step = if at.row == 0 { 1 } else { -1 };
    Pos::new(at.row + index as i8 * step, at.col)
}

fn current_tile_size() -> f32 {
    screen_width().min(screen_height()) / 8.0
}

fn pos_at_screen(x: f32, y: f32, tile_size: f32) -> Pos {
    Pos::new((y / tile_size).floor() as i8, (x / tile_size).floor() as i8)
}

#[macroquad::main(window_conf)]
async fn main() {
    let sprites = pieces::load_all_textures(current_tile_size());
    let mut game = Game::new();
    loop {
        // recomputed every frame so a resized (e.g. tiled) window stays correct
        let tile_size = current_tile_size();
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mx, my) = mouse_position();
            game.handle_click(pos_at_screen(mx, my, tile_size));
        }
        game.draw(tile_size, &sprites);
        next_frame().await;
    }
}

fn draw_piece(piece: Piece, pos: Pos, tile_size: f32, sprites: &SpriteMap) {
    let sprite = &sprites[&(piece.piece_type, piece.color)];
    draw_texture_ex(
        sprite,
        pos.col as f32 * tile_size,
        pos.row as f32 * tile_size,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(tile_size, tile_size)),
            ..Default::default()
        },
    );
}

fn draw_legal_moves(moves: &[Pos], tile_size: f32) {
    for pos in moves {
        draw_circle(
            (pos.col as f32 + 0.5) * tile_size,
            (pos.row as f32 + 0.5) * tile_size,
            tile_size / 8.0,
            GREEN,
        );
    }
}

fn draw_board_bg(tile_size: f32) {
    for row in 0..8 {
        for col in 0..8 {
            let color = if (row + col) % 2 == 0 { GRAY } else { BROWN };
            draw_rectangle(
                col as f32 * tile_size,
                row as f32 * tile_size,
                tile_size,
                tile_size,
                color,
            );
        }
    }
}

fn draw_promotion_menu(at: Pos, color: PieceColor, tile_size: f32, sprites: &SpriteMap) {
    for (i, piece_type) in PROMOTION_CHOICES.into_iter().enumerate() {
        let square = promotion_menu_square(at, i);
        draw_rectangle(
            square.col as f32 * tile_size,
            square.row as f32 * tile_size,
            tile_size,
            tile_size,
            WHITE,
        );
        draw_piece(Piece::new(piece_type, color), square, tile_size, sprites);
    }
}
