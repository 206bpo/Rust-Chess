mod board;
pub mod pieces;

use crate::board::ChessField;
use std::collections::HashMap;

use crate::{
    board::Pos,
    pieces::{Piece, PieceColor, PieceTypes},
};
use board::Board;
use macroquad::prelude::*;
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

#[macroquad::main(window_conf)]
async fn main() {
    set_pc_assets_folder("assets");
    let mut board: Board = Board::new_ready_to_play();
    let tile_size: f32 = screen_width().min(screen_height()) / 8.0;
    let sprite_map: HashMap<(PieceTypes, PieceColor), Texture2D> =
        pieces::load_all_textures(tile_size).await;
    let mut legal_moves: Vec<Pos> = Vec::new();
    let mut active_piece: Option<ChessField> = None;
    let mut active_player = PieceColor::WHITE;
    let mut wait_for_promotion: bool = false;
    loop {
        draw_board_bg(tile_size);
        draw_pieces(&board, tile_size, &sprite_map);
        draw_legal_moves(&mut legal_moves, tile_size);
        if wait_for_promotion {
            draw_promotion_menu(
                active_piece.unwrap().position,
                &active_player,
                &sprite_map,
                tile_size,
            );
        }
        {
            if is_mouse_button_pressed(MouseButton::Left) {
                let (mx, my) = mouse_position();
                if wait_for_promotion {
                    handle_mouse_click_for_promotion(
                        &mut board,
                        active_piece.unwrap().position,
                        &mut active_player,
                        &mut wait_for_promotion,
                        tile_size,
                        mx,
                        my,
                    );
                } else {
                    handle_mouse_click_for_movement(
                        &mut board,
                        &mut legal_moves,
                        &mut active_piece,
                        &mut active_player,
                        &mut wait_for_promotion,
                        tile_size,
                        mx,
                        my,
                    );
                }
            }
        }
        next_frame().await;
    }
}

fn handle_mouse_click_for_movement(
    board: &mut Board,
    legal_moves: &mut Vec<Pos>,
    active_piece: &mut Option<ChessField>,
    active_player: &mut PieceColor,
    wait_for_promotion: &mut bool,
    tile_size: f32,
    mx: f32,
    my: f32,
) {
    let clicked_pos = Pos::new(
        (my / tile_size).floor() as i8,
        (mx / tile_size).floor() as i8,
    );
    if !clicked_pos.is_in_bounds() {
        return;
    }
    let clicked_field = &mut board.get_field(clicked_pos);

    if let Some(_) = active_piece
        && legal_moves.contains(&clicked_pos)
    {
        board.move_piece(clicked_field, clicked_pos);

        if let Some(p) = board.get_field(clicked_pos).piece
            && p.piece_type == PieceTypes::PAWN
            && (clicked_pos.y == 0 || clicked_pos.y == 7)
        {
            *wait_for_promotion = true;
            *active_piece = Some(board.get_field(clicked_pos));
            legal_moves.clear();
            return;
        }

        *active_piece = None;
        legal_moves.clear();
        match active_player {
            PieceColor::BLACK => *active_player = PieceColor::WHITE,
            PieceColor::WHITE => *active_player = PieceColor::BLACK,
            _ => {
                return;
            }
        }
        return;
    }

    legal_moves.clear();
    match board.get_field(clicked_pos).piece {
        Some(p) => {
            if p.color != *active_player {
                return;
            }
            *active_piece = Some(board.get_field(clicked_pos));
            legal_moves.append(&mut p.get_legal_moves(clicked_pos, board));
        }
        None => {
            *active_piece = None;
        }
    }
}

fn handle_mouse_click_for_promotion(
    board: &mut Board,
    promote_position: Pos,
    active_player: &mut PieceColor,
    wait_for_promotion: &mut bool,
    tile_size: f32,
    mx: f32,
    my: f32,
) {
    let clicked_position = Pos::new(
        (my / tile_size).floor() as i8,
        (mx / tile_size).floor() as i8,
    );
    if promote_position.x == clicked_position.x {
        let piece_type = match active_player {
            PieceColor::BLACK => match clicked_position.y {
                7 => PieceTypes::QUEEN,
                6 => PieceTypes::ROOK,
                5 => PieceTypes::BISHOP,
                4 => PieceTypes::KNIGHT,
                _ => {
                    return;
                }
            },
            PieceColor::WHITE => match clicked_position.y {
                1 => PieceTypes::QUEEN,
                2 => PieceTypes::ROOK,
                3 => PieceTypes::BISHOP,
                4 => PieceTypes::KNIGHT,
                _ => {
                    return;
                }
            },
            _ => {
                return;
            }
        };
        let new_piece: Piece = Piece::new(piece_type, *active_player);
        board.promote(promote_position, new_piece);
        *wait_for_promotion = false;
        *active_player = match active_player {
            PieceColor::BLACK => PieceColor::WHITE,
            PieceColor::WHITE => PieceColor::BLACK,
            _ => {
                return;
            }
        }
    }
}

fn draw_pieces(
    board: &Board,
    tile_size: f32,
    sprite_map: &HashMap<(PieceTypes, PieceColor), Texture2D>,
) {
    for row in board.field {
        for field in row {
            let piece: Piece = match field.piece {
                Some(p) => p,
                None => {
                    continue;
                }
            };
            let sprite = sprite_map.get(&(piece.piece_type, piece.color)).unwrap();
            draw_texture(
                sprite,
                field.position.x as f32 * tile_size,
                field.position.y as f32 * tile_size,
                WHITE,
            );
        }
    }
}
fn draw_legal_moves(legal_moves: &mut Vec<Pos>, tile_size: f32) {
    for position in legal_moves {
        draw_circle(
            position.x as f32 * tile_size + (tile_size / 2.0),
            position.y as f32 * tile_size + (tile_size / 2.0),
            tile_size / 8.0,
            GREEN,
        );
    }
}
fn draw_board_bg(tile_size: f32) {
    let mut curr_color: Color;
    for y in 0..=7 {
        for x in 0..=7 {
            if (x + y) % 2 == 0 {
                curr_color = GRAY;
            } else {
                curr_color = BROWN;
            }
            draw_rectangle(
                (x as f32) * tile_size,
                (y as f32) * tile_size,
                tile_size,
                tile_size,
                curr_color,
            );
        }
    }
}

fn draw_promotion_menu(
    position: Pos,
    active_player: &PieceColor,
    sprite_map: &HashMap<(PieceTypes, PieceColor), Texture2D>,
    tile_size: f32,
) {
    let types = [
        PieceTypes::QUEEN,
        PieceTypes::ROOK,
        PieceTypes::BISHOP,
        PieceTypes::KNIGHT,
    ];
    for (i, piece_type) in types.iter().enumerate() {
        let offset = match active_player {
            PieceColor::WHITE => 1,
            PieceColor::BLACK => -1,
            _ => 0,
        };
        let y = (position.y + (i as i8 * offset)) as f32 * tile_size;
        let x = position.x as f32 * tile_size;
        draw_rectangle(x, y, tile_size, tile_size, WHITE);
        draw_texture(
            sprite_map.get(&(*piece_type, *active_player)).unwrap(),
            x,
            y,
            WHITE,
        );
    }
}
