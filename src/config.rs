use crate::game::Move;
use crate::piece::PieceKind;
use crate::utils::direction::Direction;
use crate::utils::point::Point;
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
    ) -> Option<[Point; 4]> {
        match piece_kind {
            PieceKind::O => None,
            PieceKind::I => match (from, to) {
                (Orientation::North, Orientation::East) => Some([
                    Point::new(-2, 0),
                    Point::new(1, 0),
                    Point::new(-2, -1),
                    Point::new(1, 2),
                ]),
                (Orientation::East, Orientation::North) => Some([
                    Point::new(2, 0),
                    Point::new(-1, 0),
                    Point::new(2, 1),
                    Point::new(-1, -2),
                ]),
                (Orientation::East, Orientation::South) => Some([
                    Point::new(-1, 0),
                    Point::new(2, 0),
                    Point::new(-1, 2),
                    Point::new(2, -1),
                ]),
                (Orientation::South, Orientation::East) => Some([
                    Point::new(1, 0),
                    Point::new(-2, 0),
                    Point::new(1, -2),
                    Point::new(-2, 1),
                ]),
                (Orientation::South, Orientation::West) => Some([
                    Point::new(2, 0),
                    Point::new(-1, 0),
                    Point::new(2, 1),
                    Point::new(-1, -2),
                ]),
                (Orientation::West, Orientation::South) => Some([
                    Point::new(-2, 0),
                    Point::new(1, 0),
                    Point::new(-2, -1),
                    Point::new(1, 2),
                ]),
                (Orientation::West, Orientation::North) => Some([
                    Point::new(1, 0),
                    Point::new(-2, 0),
                    Point::new(1, -2),
                    Point::new(-2, 1),
                ]),
                (Orientation::North, Orientation::West) => Some([
                    Point::new(-1, 0),
                    Point::new(2, 0),
                    Point::new(-1, 2),
                    Point::new(2, -1),
                ]),
                _ => None,
            },
            _ => match (from, to) {
                (Orientation::North, Orientation::East) => Some([
                    Point::new(-1, 0),
                    Point::new(-1, 1),
                    Point::new(0, -2),
                    Point::new(-1, -2),
                ]),
                (Orientation::East, Orientation::North) => Some([
                    Point::new(1, 0),
                    Point::new(1, -1),
                    Point::new(0, 2),
                    Point::new(1, 2),
                ]),
                (Orientation::East, Orientation::South) => Some([
                    Point::new(1, 0),
                    Point::new(1, -1),
                    Point::new(0, 2),
                    Point::new(1, 2),
                ]),
                (Orientation::South, Orientation::East) => Some([
                    Point::new(-1, 0),
                    Point::new(-1, 1),
                    Point::new(0, -2),
                    Point::new(-1, -2),
                ]),
                (Orientation::South, Orientation::West) => Some([
                    Point::new(1, 0),
                    Point::new(1, 1),
                    Point::new(0, -2),
                    Point::new(1, -2),
                ]),
                (Orientation::West, Orientation::South) => Some([
                    Point::new(-1, 0),
                    Point::new(-1, -1),
                    Point::new(0, 2),
                    Point::new(-1, 2),
                ]),
                (Orientation::West, Orientation::North) => Some([
                    Point::new(-1, 0),
                    Point::new(-1, -1),
                    Point::new(0, 2),
                    Point::new(-1, 2),
                ]),
                (Orientation::North, Orientation::West) => Some([
                    Point::new(1, 0),
                    Point::new(1, 1),
                    Point::new(0, -2),
                    Point::new(1, -2),
                ]),
                _ => None,
            },
        }
    }
}
