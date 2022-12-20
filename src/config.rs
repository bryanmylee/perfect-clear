use crate::game::Move;
use crate::piece::PieceKind;
use crate::utils::direction::Direction;
use crate::utils::point::Point;
use crate::utils::rotation::{Orientation, Rotation};

#[derive(Debug, Clone)]
pub enum Kick {
    SRS,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub kick: Kick,

    pub soft_drop_allowed: bool,
}

impl Config {
    pub const fn default() -> Config {
        Config {
            kick: Kick::SRS,
            soft_drop_allowed: false,
        }
    }

    pub fn kick_table(
        &self,
        piece_kind: &PieceKind,
        from: &Orientation,
        to: &Orientation,
    ) -> Option<[Point; 4]> {
        match self.kick {
            Kick::SRS => match piece_kind {
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
            },
        }
    }

    pub fn possible_moves(&self) -> Vec<Move> {
        let mut moves = vec![
            Move::Rotate(Rotation::Clockwise),
            Move::Rotate(Rotation::AntiClockwise),
            Move::Drop,
            Move::Translate(Direction::Left),
            Move::Translate(Direction::Right),
        ];
        if self.soft_drop_allowed {
            moves.push(Move::Translate(Direction::Down));
        }
        moves
    }
}
