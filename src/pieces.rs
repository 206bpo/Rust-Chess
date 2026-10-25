use crate::board::{Board, FieldState, Pos};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PieceTypes {
    PAWN,
    ROOK,
    BISHOP,
    KNIGHT,
    QUEEN,
    KING,
    EMPTY,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Color {
    BLACK,
    WHITE,
}

#[derive(Copy, Clone, Debug)]
pub struct Piece {
    pub piece_type: PieceTypes,
    pub color: Color,
}

impl Piece {
    pub fn new(piece_type: PieceTypes, color: Color) -> Piece {
        Piece { piece_type, color }
    }
    pub fn get_legal_moves(&self, curr_pos: Pos, board: &Board) -> Vec<Pos> {
        let color = self.color;
        match self.piece_type {
            PieceTypes::PAWN => pawn_get_legal_moves(curr_pos, color, board),
            PieceTypes::ROOK => rook_get_legal_moves(curr_pos, color, board),
            PieceTypes::BISHOP => bishop_get_legal_moves(curr_pos, color, board),
            PieceTypes::KNIGHT => knight_get_legal_moves(curr_pos, color, board),
            PieceTypes::QUEEN => queen_get_legal_moves(curr_pos, color, board),
            PieceTypes::KING => king_get_legal_moves(curr_pos, color, board),
            PieceTypes::EMPTY => vec![],
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
fn pawn_get_legal_moves(curr_pos: Pos, color: Color, board: &Board) -> Vec<Pos> {
    let mut res: Vec<Pos> = vec![];
    let curr_y = curr_pos.y;
    let curr_x = curr_pos.x;
    // get all moves a pawn generally can do
    match color {
        Color::WHITE => {
            // standart one front
            let front = Pos::new(curr_y - 1, curr_x);
            match board.get_state_at_pos(front) {
                FieldState::Occupied(_) => {}
                FieldState::Empty => res.push(front),
            }
            // diagonals
            let diag_left = Pos::new(curr_y - 1, curr_x - 1);
            let diag_right = Pos::new(curr_y - 1, curr_x + 1);
            if diag_left.is_in_bounds() {
                match board.get_state_at_pos(diag_left) {
                    FieldState::Occupied(_) => res.push(diag_left),
                    FieldState::Empty => {}
                }
            }
            if diag_right.is_in_bounds() {
                match board.get_state_at_pos(diag_right) {
                    FieldState::Occupied(_) => res.push(diag_right),
                    FieldState::Empty => {}
                }
            }
            // double on first move
            if curr_y == 6 {
                let two_step = Pos::new(curr_y - 2, curr_x);
                match board.get_state_at_pos(two_step) {
                    FieldState::Occupied(_) => {}
                    FieldState::Empty => res.push(two_step),
                }
            }
        }
        Color::BLACK => {
            // standart one front
            let front = Pos::new(curr_y + 1, curr_x);
            match board.get_state_at_pos(front) {
                FieldState::Occupied(_) => {}
                FieldState::Empty => res.push(front),
            }
            // diagonals
            let diag_left = Pos::new(curr_y + 1, curr_x - 1);
            let diag_right = Pos::new(curr_y + 1, curr_x + 1);
            if diag_left.is_in_bounds() {
                match board.get_state_at_pos(diag_left) {
                    FieldState::Occupied(_) => res.push(diag_left),
                    FieldState::Empty => {}
                }
            }
            if diag_right.is_in_bounds() {
                match board.get_state_at_pos(diag_right) {
                    FieldState::Occupied(_) => res.push(diag_right),
                    FieldState::Empty => {}
                }
            }
            // double on first move
            if curr_y == 1 {
                let two_step = Pos::new(curr_y + 2, curr_x);
                match board.get_state_at_pos(two_step) {
                    FieldState::Occupied(_) => {}
                    FieldState::Empty => res.push(two_step),
                }
            }
        }
    };
    res
}
fn rook_get_legal_moves(curr_pos: Pos, color: Color, board: &Board) -> Vec<Pos> {
    let mut res: Vec<Pos> = vec![];
    let curr_y = curr_pos.y;
    let curr_x = curr_pos.x;
    // x-axis +
    for i in (curr_x + 1)..=7 {
        let this_pos = Pos::new(curr_y, i);
        let state = board.get_state_at_pos(this_pos);
        match state {
            FieldState::Empty => res.push(this_pos),
            FieldState::Occupied(p) => {
                if p.color != color {
                    res.push(this_pos);
                }
                break;
            }
        }
    }
    // x-axis -
    for i in (0..curr_x).rev() {
        let this_pos = Pos::new(curr_y, i);
        let state = board.get_state_at_pos(this_pos);
        match state {
            FieldState::Empty => res.push(this_pos),
            FieldState::Occupied(p) => {
                if p.color != color {
                    res.push(this_pos);
                }
                break;
            }
        }
    }
    // y-axis +
    for i in (curr_y + 1)..=7 {
        let this_pos = Pos::new(i, curr_x);
        let state = board.get_state_at_pos(this_pos);
        match state {
            FieldState::Empty => res.push(this_pos),
            FieldState::Occupied(p) => {
                if p.color != color {
                    res.push(this_pos);
                }
                break;
            }
        }
    }
    // y-axis -
    for i in (0..curr_y).rev() {
        let this_pos = Pos::new(i, curr_x);
        let state = board.get_state_at_pos(this_pos);
        match state {
            FieldState::Empty => res.push(this_pos),
            FieldState::Occupied(p) => {
                if p.color != color {
                    res.push(this_pos);
                }
                break;
            }
        }
    }

    res
}
fn bishop_get_legal_moves(curr_pos: Pos, color: Color, board: &Board) -> Vec<Pos> {
    let mut res: Vec<Pos> = vec![];
    let curr_y = curr_pos.y;
    let curr_x = curr_pos.x;
    let mut this_pos: Pos;
    let mut this_state: FieldState;

    let mut top_right_break = false;
    let mut top_left_break = false;
    let mut bottom_right_break = false;
    let mut botom_left_break = false;
    for i in 1..=7 {
        // top right
        if !top_right_break {
            this_pos = Pos::new(curr_y - i, curr_x + i);
            if this_pos.is_in_bounds() {
                this_state = board.get_state_at_pos(this_pos);
                match this_state {
                    FieldState::Empty => res.push(this_pos),
                    FieldState::Occupied(p) => {
                        if p.color != color {
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
                this_state = board.get_state_at_pos(this_pos);
                match this_state {
                    FieldState::Empty => res.push(this_pos),
                    FieldState::Occupied(p) => {
                        if p.color != color {
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
                this_state = board.get_state_at_pos(this_pos);
                match this_state {
                    FieldState::Empty => res.push(this_pos),
                    FieldState::Occupied(p) => {
                        if p.color != color {
                            res.push(this_pos);
                        }
                        bottom_right_break = true;
                    }
                }
            }
        }
        if !botom_left_break {
            this_pos = Pos::new(curr_y + i, curr_x - i);
            if this_pos.is_in_bounds() {
                this_state = board.get_state_at_pos(this_pos);
                match this_state {
                    FieldState::Empty => res.push(this_pos),
                    FieldState::Occupied(p) => {
                        if p.color != color {
                            res.push(this_pos);
                        }
                        botom_left_break = true;
                    }
                }
            }
        }
    }
    res
}
fn knight_get_legal_moves(curr_pos: Pos, color: Color, board: &Board) -> Vec<Pos> {
    let mut res: Vec<Pos> = vec![];
    let curr_x = curr_pos.x;
    let curr_y = curr_pos.y;
    let mut this_state: FieldState;
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
            this_state = board.get_state_at_pos(this_pos);
            match this_state {
                FieldState::Empty => res.push(this_pos),
                FieldState::Occupied(p) => {
                    if p.color != color {
                        res.push(this_pos);
                    }
                }
            }
        }
    }
    res
}
fn queen_get_legal_moves(curr_pos: Pos, color: Color, board: &Board) -> Vec<Pos> {
    let mut res: Vec<Pos> = vec![];
    let curr_x = curr_pos.x;
    let curr_y = curr_pos.y;
    let mut this_state: FieldState;
    res.append(&mut bishop_get_legal_moves(curr_pos, color, board));
    res.append(&mut rook_get_legal_moves(curr_pos, color, board));
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
    for this_pos in possible_moves {
        if this_pos.is_in_bounds() {
            this_state = board.get_state_at_pos(this_pos);
            match this_state {
                FieldState::Empty => res.push(this_pos),
                FieldState::Occupied(p) => {
                    if p.color != color {
                        res.push(this_pos);
                    }
                }
            }
        }
    }

    res
}
fn king_get_legal_moves(curr_pos: Pos, color: Color, board: &Board) -> Vec<Pos> {
    let mut res: Vec<Pos> = vec![];
    let curr_x = curr_pos.x;
    let curr_y = curr_pos.y;
    let mut this_state: FieldState;
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
    for this_pos in possible_moves {
        if this_pos.is_in_bounds() {
            this_state = board.get_state_at_pos(this_pos);
            match this_state {
                FieldState::Empty => res.push(this_pos),
                FieldState::Occupied(p) => {
                    if p.color != color {
                        res.push(this_pos);
                    }
                }
            }
        }
    }

    res
}
