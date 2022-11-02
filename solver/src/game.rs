use crate::board::Board;
use crate::config::Config;
use crate::piece::{Piece, PieceKind};

#[derive(Debug, Clone)]
pub struct NextProbability {
    i: f64,
    j: f64,
    l: f64,
    o: f64,
    s: f64,
    t: f64,
    z: f64,
}

#[derive(Debug, Clone)]
pub struct State {
    board: Board,
    piece: Piece,
    is_valid: bool,
    hold: Option<PieceKind>,
    is_hold_used: bool,
    queue: [Option<PieceKind>; 7], // fixed queue size to reduce heap allocations
    seen: [Option<PieceKind>; 14], // only to 2-bags needed to determine next piece probability
    next_prob: NextProbability,
    pc_prob: f64,
    next_pc_prob: f64,
}

impl State {
    pub fn reduce(&self, action: &Action, config: &Config) -> State {
        match action {
            Action::Spawn(piece_type) => self.spawned(piece_type, config),
            _ => self.clone(),
        }
    }

    fn spawned(&self, piece_type: &PieceKind, config: &Config) -> State {
        self.clone()
    }
}

#[derive(Debug, Clone)]
pub enum Action {
    Rotate(Rotation),
    Move(Direction),
    Drop,
    Hold,
    Place,
    Spawn(PieceKind),
}

#[derive(Debug, Clone)]
pub enum Rotation {
    Clockwise,
    AntiClockwise,
    Half,
}

#[derive(Debug, Clone)]
pub enum Direction {
    Left,
    Right,
    Down,
}
