use crate::{
    board::BoardFill,
    config::{Config, RotationSystem},
    point::Point,
};

#[derive(Debug, Clone)]
pub enum PieceKind {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

#[derive(Debug, Clone)]
pub struct Piece {
    pub kind: PieceKind,
    pub center: Point<isize>,
    pub orientation: Orientation,
}

#[derive(Debug, Clone)]
pub enum Orientation {
    North,
    South,
    East,
    West,
}

impl Piece {
    pub fn get_fill(&self, config: &Config) -> BoardFill {
        match config.rotation_system {
            RotationSystem::SRS => self.get_srs_piece_fill(config),
        }
    }

    fn get_srs_piece_fill(&self, config: &Config) -> BoardFill {}
}
