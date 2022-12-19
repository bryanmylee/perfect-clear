use crate::game::Move;
use crate::piece::PieceKind;
use crate::utils::direction::Direction;
use crate::utils::point::ISizePoint;
use crate::utils::rotation::{Orientation, Rotation};

#[derive(Debug, Clone)]
pub enum RotationSystem {
    SRS,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub rotation_system: RotationSystem,
}

pub mod srs {
    use super::*;

    pub const POSSIBLE_MOVES: [Move; 5] = [
        Move::Rotate(Rotation::Clockwise),
        Move::Rotate(Rotation::AntiClockwise),
        Move::Drop,
        Move::Translate(Direction::Left),
        Move::Translate(Direction::Right),
    ];

    pub fn kick_table(
        piece_kind: &PieceKind,
        from: &Orientation,
        to: &Orientation,
    ) -> Option<[ISizePoint; 4]> {
        match piece_kind {
            PieceKind::O => None,
            PieceKind::I => match (from, to) {
                (Orientation::North, Orientation::East) => Some([
                    ISizePoint::new(-2, 0),
                    ISizePoint::new(1, 0),
                    ISizePoint::new(-2, -1),
                    ISizePoint::new(1, 2),
                ]),
                (Orientation::East, Orientation::North) => Some([
                    ISizePoint::new(2, 0),
                    ISizePoint::new(-1, 0),
                    ISizePoint::new(2, 1),
                    ISizePoint::new(-1, -2),
                ]),
                (Orientation::East, Orientation::South) => Some([
                    ISizePoint::new(-1, 0),
                    ISizePoint::new(2, 0),
                    ISizePoint::new(-1, 2),
                    ISizePoint::new(2, -1),
                ]),
                (Orientation::South, Orientation::East) => Some([
                    ISizePoint::new(1, 0),
                    ISizePoint::new(-2, 0),
                    ISizePoint::new(1, -2),
                    ISizePoint::new(-2, 1),
                ]),
                (Orientation::South, Orientation::West) => Some([
                    ISizePoint::new(2, 0),
                    ISizePoint::new(-1, 0),
                    ISizePoint::new(2, 1),
                    ISizePoint::new(-1, -2),
                ]),
                (Orientation::West, Orientation::South) => Some([
                    ISizePoint::new(-2, 0),
                    ISizePoint::new(1, 0),
                    ISizePoint::new(-2, -1),
                    ISizePoint::new(1, 2),
                ]),
                (Orientation::West, Orientation::North) => Some([
                    ISizePoint::new(1, 0),
                    ISizePoint::new(-2, 0),
                    ISizePoint::new(1, -2),
                    ISizePoint::new(-2, 1),
                ]),
                (Orientation::North, Orientation::West) => Some([
                    ISizePoint::new(-1, 0),
                    ISizePoint::new(2, 0),
                    ISizePoint::new(-1, 2),
                    ISizePoint::new(2, -1),
                ]),
                _ => None,
            },
            _ => match (from, to) {
                (Orientation::North, Orientation::East) => Some([
                    ISizePoint::new(-1, 0),
                    ISizePoint::new(-1, 1),
                    ISizePoint::new(0, -2),
                    ISizePoint::new(-1, -2),
                ]),
                (Orientation::East, Orientation::North) => Some([
                    ISizePoint::new(1, 0),
                    ISizePoint::new(1, -1),
                    ISizePoint::new(0, 2),
                    ISizePoint::new(1, 2),
                ]),
                (Orientation::East, Orientation::South) => Some([
                    ISizePoint::new(1, 0),
                    ISizePoint::new(1, -1),
                    ISizePoint::new(0, 2),
                    ISizePoint::new(1, 2),
                ]),
                (Orientation::South, Orientation::East) => Some([
                    ISizePoint::new(-1, 0),
                    ISizePoint::new(-1, 1),
                    ISizePoint::new(0, -2),
                    ISizePoint::new(-1, -2),
                ]),
                (Orientation::South, Orientation::West) => Some([
                    ISizePoint::new(1, 0),
                    ISizePoint::new(1, 1),
                    ISizePoint::new(0, -2),
                    ISizePoint::new(1, -2),
                ]),
                (Orientation::West, Orientation::South) => Some([
                    ISizePoint::new(-1, 0),
                    ISizePoint::new(-1, -1),
                    ISizePoint::new(0, 2),
                    ISizePoint::new(-1, 2),
                ]),
                (Orientation::West, Orientation::North) => Some([
                    ISizePoint::new(-1, 0),
                    ISizePoint::new(-1, -1),
                    ISizePoint::new(0, 2),
                    ISizePoint::new(-1, 2),
                ]),
                (Orientation::North, Orientation::West) => Some([
                    ISizePoint::new(1, 0),
                    ISizePoint::new(1, 1),
                    ISizePoint::new(0, -2),
                    ISizePoint::new(1, -2),
                ]),
                _ => None,
            },
        }
    }
}
