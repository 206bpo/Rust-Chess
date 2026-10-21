use crate::board::Pos;

#[derive(Debug)]
pub struct MoveInstruction {
    pub from: Pos,
    pub to: Pos,
}

impl MoveInstruction {
    pub fn new(from: Pos, to: Pos) -> MoveInstruction {
        MoveInstruction { from, to }
    }
    pub fn from_player_input(input: String) -> Option<MoveInstruction> {
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
