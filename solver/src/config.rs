#[derive(Debug, Clone)]
pub enum RotationSystem {
    SRS,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub rotation_system: RotationSystem,
}

pub mod srs {
    use crate::{piece::PieceKind, point::Point, rotation::Orientation};

    pub fn kick_table(
        piece_kind: &PieceKind,
        from: &Orientation,
        to: &Orientation,
    ) -> Option<[Point<isize>; 4]> {
        match piece_kind {
            PieceKind::O => None,
            PieceKind::I => match (from, to) {
                (Orientation::North, Orientation::East) => Some([
                    Point { x: -2, y: 0 },
                    Point { x: 1, y: 0 },
                    Point { x: -2, y: -1 },
                    Point { x: 1, y: 2 },
                ]),
                (Orientation::East, Orientation::North) => Some([
                    Point { x: 2, y: 0 },
                    Point { x: -1, y: 0 },
                    Point { x: 2, y: 1 },
                    Point { x: -1, y: -2 },
                ]),
                (Orientation::East, Orientation::South) => Some([
                    Point { x: -1, y: 0 },
                    Point { x: 2, y: 0 },
                    Point { x: -1, y: 2 },
                    Point { x: 2, y: -1 },
                ]),
                (Orientation::South, Orientation::East) => Some([
                    Point { x: 1, y: 0 },
                    Point { x: -2, y: 0 },
                    Point { x: 1, y: -2 },
                    Point { x: -2, y: 1 },
                ]),
                (Orientation::South, Orientation::West) => Some([
                    Point { x: 2, y: 0 },
                    Point { x: -1, y: 0 },
                    Point { x: 2, y: 1 },
                    Point { x: -1, y: -2 },
                ]),
                (Orientation::West, Orientation::South) => Some([
                    Point { x: -2, y: 0 },
                    Point { x: 1, y: 0 },
                    Point { x: -2, y: -1 },
                    Point { x: 1, y: 2 },
                ]),
                (Orientation::West, Orientation::North) => Some([
                    Point { x: 1, y: 0 },
                    Point { x: -2, y: 0 },
                    Point { x: 1, y: -2 },
                    Point { x: -2, y: 1 },
                ]),
                (Orientation::North, Orientation::West) => Some([
                    Point { x: -1, y: 0 },
                    Point { x: 2, y: 0 },
                    Point { x: -1, y: 2 },
                    Point { x: 2, y: -1 },
                ]),
                _ => None,
            },
            _ => match (from, to) {
                (Orientation::North, Orientation::East) => Some([
                    Point { x: -1, y: 0 },
                    Point { x: -1, y: 1 },
                    Point { x: 0, y: -2 },
                    Point { x: -1, y: -2 },
                ]),
                (Orientation::East, Orientation::North) => Some([
                    Point { x: 1, y: 0 },
                    Point { x: 1, y: -1 },
                    Point { x: 0, y: 2 },
                    Point { x: 1, y: 2 },
                ]),
                (Orientation::East, Orientation::South) => Some([
                    Point { x: 1, y: 0 },
                    Point { x: 1, y: -1 },
                    Point { x: 0, y: 2 },
                    Point { x: 1, y: 2 },
                ]),
                (Orientation::South, Orientation::East) => Some([
                    Point { x: -1, y: 0 },
                    Point { x: -1, y: 1 },
                    Point { x: 0, y: -2 },
                    Point { x: -1, y: -2 },
                ]),
                (Orientation::South, Orientation::West) => Some([
                    Point { x: 1, y: 0 },
                    Point { x: 1, y: 1 },
                    Point { x: 0, y: -2 },
                    Point { x: 1, y: -2 },
                ]),
                (Orientation::West, Orientation::South) => Some([
                    Point { x: -1, y: 0 },
                    Point { x: -1, y: -1 },
                    Point { x: 0, y: 2 },
                    Point { x: -1, y: 2 },
                ]),
                (Orientation::West, Orientation::North) => Some([
                    Point { x: -1, y: 0 },
                    Point { x: -1, y: -1 },
                    Point { x: 0, y: 2 },
                    Point { x: -1, y: 2 },
                ]),
                (Orientation::North, Orientation::West) => Some([
                    Point { x: 1, y: 0 },
                    Point { x: 1, y: 1 },
                    Point { x: 0, y: -2 },
                    Point { x: 1, y: -2 },
                ]),
                _ => None,
            },
        }
    }
}
