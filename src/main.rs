mod board;
mod input;
pub mod pieces;

use std::collections::HashMap;

use crate::{
    board::{
        FieldState::{self, Occupied},
        Pos,
    },
    input::PromoteInstruction,
    pieces::{Piece, PieceColor, PieceTypes},
};
use board::Board;
use input::MoveInstruction;
use macroquad::{prelude::*, ui::Drag::No};
fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Chess"),
        window_width: 800,
        window_height: 800,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    set_pc_assets_folder("assets");
    let mut board: Board = Board::new_ready_to_play();
    board.draw();
    let tile_size: f32 = screen_width().min(screen_height()) / 8.0;
    let sprite_map: HashMap<(PieceTypes, PieceColor), Texture2D> =
        pieces::load_all_textures(tile_size).await;
    let mut legal_moves: Vec<Pos> = Vec::new();
    let mut active_piece: Option<Pos> = None;
    let mut active_player = PieceColor::WHITE;
    loop {
        draw_board_bg(tile_size);
        draw_pieces(&board, tile_size, &sprite_map);
        draw_legal_moves(&mut legal_moves, tile_size);
        {
            if is_mouse_button_pressed(MouseButton::Left) {
                let (mx, my) = mouse_position();
                handle_mouse_click(
                    &mut board,
                    &mut legal_moves,
                    &mut active_piece,
                    &mut active_player,
                    tile_size,
                    mx,
                    my,
                );
            }
        }
        next_frame().await;
    }
}

fn handle_mouse_click(
    board: &mut Board,
    legal_moves: &mut Vec<Pos>,
    active_piece: &mut Option<Pos>,
    active_player: &mut PieceColor,
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

    if let Some(pice_pos) = active_piece.clone() {
        if legal_moves.contains(&clicked_pos) {
            board.move_piece(pice_pos, clicked_pos);
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
    }

    legal_moves.clear();
    match board.get_state_at_pos(clicked_pos) {
        FieldState::Occupied(p) => {
            if p.color != *active_player {
                return;
            }
            *active_piece = Some(clicked_pos);
            legal_moves.append(&mut p.get_legal_moves(clicked_pos, board));
        }
        FieldState::Empty => {
            *active_piece = None;
        }
    }
}

fn next_player_move(board: &mut Board) {
    let instruction: MoveInstruction =
        MoveInstruction::from_player_input().expect("can't parse instruction");
    let state_from = board.get_state_at_pos(instruction.from);
    let piece_from = match state_from {
        FieldState::Occupied(p) => p,
        FieldState::Empty => {
            return;
        }
    };
    let legal_moves = piece_from.get_legal_moves(instruction.from, board);
    if legal_moves.contains(&instruction.to) {
        if (piece_from.piece_type == PieceTypes::PAWN)
            && (instruction.to.y == 0 || instruction.to.y == 7)
        {
            dbg!("promotion detected");
            let promote_instruction: PromoteInstruction =
                match PromoteInstruction::from_player_input(instruction.to) {
                    Some(instruction) => instruction,
                    None => {
                        return;
                    }
                };
            board.move_piece(instruction.from, instruction.to);
            board.promote(promote_instruction);
        } else {
            board.move_piece(instruction.from, instruction.to);
        }
    }
}
fn draw_pieces(
    board: &Board,
    tile_size: f32,
    sprite_map: &HashMap<(PieceTypes, PieceColor), Texture2D>,
) {
    for (y, row) in board.field.iter().enumerate() {
        for (x, field) in row.iter().enumerate() {
            let piece: Piece = match *field {
                FieldState::Occupied(p) => p,
                FieldState::Empty => {
                    continue;
                }
            };
            let sprite = sprite_map.get(&(piece.piece_type, piece.color)).unwrap();
            draw_texture(sprite, x as f32 * tile_size, y as f32 * tile_size, WHITE);
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
