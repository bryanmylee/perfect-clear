use crate::board::Board;
use crate::piece::{Piece, PieceType};

#[derive(Debug)]
pub struct NextProbability {
    i: f64,
    j: f64,
    l: f64,
    o: f64,
    s: f64,
    t: f64,
    z: f64,
}

#[derive(Debug)]
pub struct State {
    board: Board,
    piece: Piece,
    hold: Option<PieceType>,
    is_hold_used: bool,
    queue: [Option<PieceType>; 7], // fixed queue size to reduce heap allocations
    seen: [Option<PieceType>; 14], // only to 2-bags needed to determine next piece probability
    next_prob: NextProbability,
    pc_prob: f64,
    next_pc_prob: f64,
}

impl State {
    fn reduce(&self, action: &Action) -> State {
        self.clone()
    }
}

pub enum Action {
    Rotate(Rotation),
    Move(Direction),
    Drop,
    Hold,
    Place,
}

pub enum Rotation {
    Clockwise,
    AntiClockwise,
    Half,
}

pub enum Direction {
    Left,
    Right,
    Down,
}
