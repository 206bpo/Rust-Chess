use std::{io, ptr::read};

use crate::{board::Pos, input, pieces::PieceTypes};

#[derive(Debug)]
pub struct MoveInstruction {
    pub from: Pos,
    pub to: Pos,
}
#[derive(Debug)]
pub struct PromoteInstruction {
    pub position: Pos,
    pub new_type: PieceTypes,
}

impl PromoteInstruction {
    pub fn new(position: Pos, new_type: PieceTypes) -> PromoteInstruction {
        PromoteInstruction { position, new_type }
    }
    pub fn from_player_input(position: Pos) -> Option<PromoteInstruction> {
        println!("Enter the piece type you want to promote to:");
        println!("Q: Queen\tK: Knight\tR: Rook\tB: Bishop");
        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Could not read input");
        let mut instruction = PromoteInstruction::new(position, PieceTypes::EMPTY);
        match input.to_uppercase().trim() {
            "Q" => instruction.new_type = PieceTypes::QUEEN,
            "K" => instruction.new_type = PieceTypes::KNIGHT,
            "R" => instruction.new_type = PieceTypes::ROOK,
            "B" => instruction.new_type = PieceTypes::BISHOP,
            _ => return None,
        }
        Some(instruction)
    }
}

impl MoveInstruction {
    pub fn new(from: Pos, to: Pos) -> MoveInstruction {
        MoveInstruction { from, to }
    }
    pub fn from_player_input() -> Option<MoveInstruction> {
        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Something went wrong while reading line");
        let input = input.trim().to_lowercase();
        if input.len() != 4 {
            return None;
        };
        let from_str: &str = &input[0..2];
        let to_str: &str = &input[2..4];
        let from = convert_player_input_to_pos(from_str);
        let to = convert_player_input_to_pos(to_str);
        let instruction = MoveInstruction::new(from, to);
        Some(instruction)
    }
}

fn convert_player_input_to_pos(input: &str) -> Pos {
    let x: i8 = match input.get(0..1).unwrap() {
        "a" => 0,
        "b" => 1,
        "c" => 2,
        "d" => 3,
        "e" => 4,
        "f" => 5,
        "g" => 6,
        "h" => 7,
        _ => i8::MAX,
    };
    let y: i8 = input.get(1..2).unwrap().parse().unwrap();
    Pos::new(y - 1, x)
}
