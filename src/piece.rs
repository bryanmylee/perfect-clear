use crate::config::{Config, RotationSystem};
use crate::utils::point::Point;
use crate::utils::rotation::Orientation;
use std::convert::TryFrom;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PieceKind {
    I = 0,
    J = 1,
    L = 2,
    O = 3,
    S = 4,
    T = 5,
    Z = 6,
}

pub const PIECE_KINDS: [PieceKind; 7] = [
    PieceKind::I,
    PieceKind::J,
    PieceKind::L,
    PieceKind::O,
    PieceKind::S,
    PieceKind::T,
    PieceKind::Z,
];

impl TryFrom<u8> for PieceKind {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PieceKind::I),
            1 => Ok(PieceKind::J),
            2 => Ok(PieceKind::L),
            3 => Ok(PieceKind::O),
            4 => Ok(PieceKind::S),
            5 => Ok(PieceKind::T),
            6 => Ok(PieceKind::Z),
            _ => Err(()),
        }
    }
}

impl PieceKind {
    pub fn get_spawn_point(&self, config: &Config) -> Point {
        match config.rotation_system {
            RotationSystem::SRS => match self {
                PieceKind::I => Point::new(3, 18),
                PieceKind::J => Point::new(3, 19),
                PieceKind::L => Point::new(3, 19),
                PieceKind::O => Point::new(3, 19),
                PieceKind::S => Point::new(3, 19),
                PieceKind::T => Point::new(3, 19),
                PieceKind::Z => Point::new(3, 19),
            },
        }
    }

    fn get_unoriented_offset_box(&self, config: &Config) -> PieceOffsetBox {
        PieceOffsetBox {
            offsets: self.get_position_offsets(config),
            bounding_box_size: self.get_bounding_box_size(config),
        }
    }

    fn get_position_offsets(&self, _config: &Config) -> [Point; 4] {
        match self {
            PieceKind::I => [
                Point::new(0, 2),
                Point::new(1, 2),
                Point::new(2, 2),
                Point::new(3, 2),
            ],
            PieceKind::J => [
                Point::new(0, 2),
                Point::new(0, 1),
                Point::new(1, 1),
                Point::new(2, 1),
            ],
            PieceKind::L => [
                Point::new(2, 2),
                Point::new(0, 1),
                Point::new(1, 1),
                Point::new(2, 1),
            ],
            PieceKind::O => [
                Point::new(1, 2),
                Point::new(2, 2),
                Point::new(1, 1),
                Point::new(2, 1),
            ],
            PieceKind::S => [
                Point::new(1, 2),
                Point::new(2, 2),
                Point::new(0, 1),
                Point::new(1, 1),
            ],
            PieceKind::T => [
                Point::new(1, 2),
                Point::new(0, 1),
                Point::new(1, 1),
                Point::new(2, 1),
            ],
            PieceKind::Z => [
                Point::new(0, 2),
                Point::new(1, 2),
                Point::new(1, 1),
                Point::new(2, 1),
            ],
        }
    }

    fn get_bounding_box_size(&self, _config: &Config) -> usize {
        match self {
            PieceKind::I => 4,
            PieceKind::J => 3,
            PieceKind::L => 3,
            PieceKind::O => 4,
            PieceKind::S => 3,
            PieceKind::T => 3,
            PieceKind::Z => 3,
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Piece {
    pub kind: PieceKind,
    /**
    The bottom-left-most corner of the **bounding box**.
    */
    pub position: Point,
    pub orientation: Orientation,
}

impl Piece {
    pub fn spawn(config: &Config, kind: &PieceKind) -> Piece {
        Piece {
            kind: kind.clone(),
            position: kind.get_spawn_point(config),
            orientation: Orientation::North,
        }
    }
}

/**
Pieces can be represented by four points from the bottom-left corner of a bounding box.
*/
struct PieceOffsetBox {
    offsets: [Point; 4],
    bounding_box_size: usize,
}

impl Piece {
    pub fn get_points(&self, config: &Config) -> [Point; 4] {
        let mut unoriented_offset_box = self.kind.get_unoriented_offset_box(config);
        orient_offset_box(&mut unoriented_offset_box, &self.orientation);
        unoriented_offset_box
            .offsets
            .map(|offset| offset + self.position)
    }
}

fn orient_offset_box(unoriented_offset_box: &mut PieceOffsetBox, orientation: &Orientation) {
    let size_minus_one = (unoriented_offset_box.bounding_box_size - 1) as isize;
    match orientation {
        Orientation::North => {}
        Orientation::South => {
            for offset in unoriented_offset_box.offsets.iter_mut() {
                offset.x = size_minus_one - offset.x;
                offset.y = size_minus_one - offset.y;
            }
        }
        Orientation::East => {
            for offset in unoriented_offset_box.offsets.iter_mut() {
                (offset.x, offset.y) = (offset.y, size_minus_one - offset.x);
            }
        }
        Orientation::West => {
            for offset in unoriented_offset_box.offsets.iter_mut() {
                (offset.x, offset.y) = (size_minus_one - offset.y, offset.x);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::RotationSystem;

    const CONFIG: Config = Config {
        rotation_system: RotationSystem::SRS,
    };

    mod get_points {
        use super::*;

        #[test]
        fn j_piece_no_orientation() {
            let piece = Piece {
                kind: PieceKind::J,
                orientation: Orientation::North,
                position: Point::new(3, 18),
            };
            assert_eq!(
                piece.get_points(&CONFIG),
                [
                    Point::new(3, 20),
                    Point::new(3, 19),
                    Point::new(4, 19),
                    Point::new(5, 19),
                ]
            );
        }
    }

    mod orient_offsets {
        use super::*;

        mod north {
            use super::*;

            #[test]
            fn i_piece() {
                let mut offset_box = PieceKind::I.get_unoriented_offset_box(&CONFIG);
                orient_offset_box(&mut offset_box, &Orientation::North);
                assert!(offset_box.offsets.contains(&Point::new(0, 2)));
                assert!(offset_box.offsets.contains(&Point::new(1, 2)));
                assert!(offset_box.offsets.contains(&Point::new(2, 2)));
                assert!(offset_box.offsets.contains(&Point::new(3, 2)));
            }

            #[test]
            fn j_piece() {
                let mut offset_box = PieceKind::J.get_unoriented_offset_box(&CONFIG);
                orient_offset_box(&mut offset_box, &Orientation::North);
                assert!(offset_box.offsets.contains(&Point::new(0, 2)));
                assert!(offset_box.offsets.contains(&Point::new(0, 1)));
                assert!(offset_box.offsets.contains(&Point::new(1, 1)));
                assert!(offset_box.offsets.contains(&Point::new(2, 1)));
            }

            #[test]
            fn l_piece() {
                let mut offset_box = PieceKind::L.get_unoriented_offset_box(&CONFIG);
                orient_offset_box(&mut offset_box, &Orientation::North);
                assert!(offset_box.offsets.contains(&Point::new(2, 2)));
                assert!(offset_box.offsets.contains(&Point::new(0, 1)));
                assert!(offset_box.offsets.contains(&Point::new(1, 1)));
                assert!(offset_box.offsets.contains(&Point::new(2, 1)));
            }

            #[test]
            fn o_piece() {
                let mut offset_box = PieceKind::O.get_unoriented_offset_box(&CONFIG);
                orient_offset_box(&mut offset_box, &Orientation::North);
                assert!(offset_box.offsets.contains(&Point::new(1, 2)));
                assert!(offset_box.offsets.contains(&Point::new(2, 2)));
                assert!(offset_box.offsets.contains(&Point::new(1, 1)));
                assert!(offset_box.offsets.contains(&Point::new(2, 1)));
            }

            #[test]
            fn s_piece() {
                let mut offset_box = PieceKind::S.get_unoriented_offset_box(&CONFIG);
                orient_offset_box(&mut offset_box, &Orientation::North);
                assert!(offset_box.offsets.contains(&Point::new(1, 2)));
                assert!(offset_box.offsets.contains(&Point::new(2, 2)));
                assert!(offset_box.offsets.contains(&Point::new(0, 1)));
                assert!(offset_box.offsets.contains(&Point::new(1, 1)));
            }

            #[test]
            fn t_piece() {
                let mut offset_box = PieceKind::T.get_unoriented_offset_box(&CONFIG);
                orient_offset_box(&mut offset_box, &Orientation::North);
                assert!(offset_box.offsets.contains(&Point::new(1, 2)));
                assert!(offset_box.offsets.contains(&Point::new(0, 1)));
                assert!(offset_box.offsets.contains(&Point::new(1, 1)));
                assert!(offset_box.offsets.contains(&Point::new(2, 1)));
            }

            #[test]
            fn z_piece() {
                let mut offset_box = PieceKind::Z.get_unoriented_offset_box(&CONFIG);
                orient_offset_box(&mut offset_box, &Orientation::North);
                assert!(offset_box.offsets.contains(&Point::new(0, 2)));
                assert!(offset_box.offsets.contains(&Point::new(1, 2)));
                assert!(offset_box.offsets.contains(&Point::new(1, 1)));
                assert!(offset_box.offsets.contains(&Point::new(2, 1)));
            }
        }

        mod south {
            use super::*;

            #[test]
            fn i_piece() {
                let mut offset_box = PieceKind::I.get_unoriented_offset_box(&CONFIG);
                orient_offset_box(&mut offset_box, &Orientation::South);
                assert!(offset_box.offsets.contains(&Point::new(0, 1)));
                assert!(offset_box.offsets.contains(&Point::new(1, 1)));
                assert!(offset_box.offsets.contains(&Point::new(2, 1)));
                assert!(offset_box.offsets.contains(&Point::new(3, 1)));
            }

            #[test]
            fn j_piece() {
                let mut offset_box = PieceKind::J.get_unoriented_offset_box(&CONFIG);
                orient_offset_box(&mut offset_box, &Orientation::South);
                assert!(offset_box.offsets.contains(&Point::new(0, 1)));
                assert!(offset_box.offsets.contains(&Point::new(1, 1)));
                assert!(offset_box.offsets.contains(&Point::new(2, 1)));
                assert!(offset_box.offsets.contains(&Point::new(2, 0)));
            }

            #[test]
            fn l_piece() {
                let mut offset_box = PieceKind::L.get_unoriented_offset_box(&CONFIG);
                orient_offset_box(&mut offset_box, &Orientation::South);
                assert!(offset_box.offsets.contains(&Point::new(0, 1)));
                assert!(offset_box.offsets.contains(&Point::new(1, 1)));
                assert!(offset_box.offsets.contains(&Point::new(2, 1)));
                assert!(offset_box.offsets.contains(&Point::new(0, 0)));
            }

            #[test]
            fn o_piece() {
                let mut offset_box = PieceKind::O.get_unoriented_offset_box(&CONFIG);
                orient_offset_box(&mut offset_box, &Orientation::South);
                assert!(offset_box.offsets.contains(&Point::new(1, 2)));
                assert!(offset_box.offsets.contains(&Point::new(2, 2)));
                assert!(offset_box.offsets.contains(&Point::new(1, 1)));
                assert!(offset_box.offsets.contains(&Point::new(2, 1)));
            }

            #[test]
            fn s_piece() {
                let mut offset_box = PieceKind::S.get_unoriented_offset_box(&CONFIG);
                orient_offset_box(&mut offset_box, &Orientation::South);
                assert!(offset_box.offsets.contains(&Point::new(1, 1)));
                assert!(offset_box.offsets.contains(&Point::new(2, 1)));
                assert!(offset_box.offsets.contains(&Point::new(0, 0)));
                assert!(offset_box.offsets.contains(&Point::new(1, 0)));
            }

            #[test]
            fn t_piece() {
                let mut offset_box = PieceKind::T.get_unoriented_offset_box(&CONFIG);
                orient_offset_box(&mut offset_box, &Orientation::South);
                assert!(offset_box.offsets.contains(&Point::new(0, 1)));
                assert!(offset_box.offsets.contains(&Point::new(1, 1)));
                assert!(offset_box.offsets.contains(&Point::new(2, 1)));
                assert!(offset_box.offsets.contains(&Point::new(1, 0)));
            }

            #[test]
            fn z_piece() {
                let mut offset_box = PieceKind::Z.get_unoriented_offset_box(&CONFIG);
                orient_offset_box(&mut offset_box, &Orientation::South);
                assert!(offset_box.offsets.contains(&Point::new(0, 1)));
                assert!(offset_box.offsets.contains(&Point::new(1, 1)));
                assert!(offset_box.offsets.contains(&Point::new(1, 0)));
                assert!(offset_box.offsets.contains(&Point::new(2, 0)));
            }
        }

        mod east {
            use super::*;

            #[test]
            fn i_piece() {
                let mut offset_box = PieceKind::I.get_unoriented_offset_box(&CONFIG);
                orient_offset_box(&mut offset_box, &Orientation::East);
                assert!(offset_box.offsets.contains(&Point::new(2, 3)));
                assert!(offset_box.offsets.contains(&Point::new(2, 2)));
                assert!(offset_box.offsets.contains(&Point::new(2, 1)));
                assert!(offset_box.offsets.contains(&Point::new(2, 0)));
            }

            #[test]
            fn j_piece() {
                let mut offset_box = PieceKind::J.get_unoriented_offset_box(&CONFIG);
                orient_offset_box(&mut offset_box, &Orientation::East);
                assert!(offset_box.offsets.contains(&Point::new(1, 2)));
                assert!(offset_box.offsets.contains(&Point::new(2, 2)));
                assert!(offset_box.offsets.contains(&Point::new(1, 1)));
                assert!(offset_box.offsets.contains(&Point::new(1, 0)));
            }

            #[test]
            fn l_piece() {
                let mut offset_box = PieceKind::L.get_unoriented_offset_box(&CONFIG);
                orient_offset_box(&mut offset_box, &Orientation::East);
                assert!(offset_box.offsets.contains(&Point::new(1, 2)));
                assert!(offset_box.offsets.contains(&Point::new(1, 1)));
                assert!(offset_box.offsets.contains(&Point::new(1, 0)));
                assert!(offset_box.offsets.contains(&Point::new(2, 0)));
            }

            #[test]
            fn o_piece() {
                let mut offset_box = PieceKind::O.get_unoriented_offset_box(&CONFIG);
                orient_offset_box(&mut offset_box, &Orientation::East);
                assert!(offset_box.offsets.contains(&Point::new(1, 2)));
                assert!(offset_box.offsets.contains(&Point::new(2, 2)));
                assert!(offset_box.offsets.contains(&Point::new(1, 1)));
                assert!(offset_box.offsets.contains(&Point::new(2, 1)));
            }

            #[test]
            fn s_piece() {
                let mut offset_box = PieceKind::S.get_unoriented_offset_box(&CONFIG);
                orient_offset_box(&mut offset_box, &Orientation::East);
                assert!(offset_box.offsets.contains(&Point::new(1, 2)));
                assert!(offset_box.offsets.contains(&Point::new(1, 1)));
                assert!(offset_box.offsets.contains(&Point::new(2, 1)));
                assert!(offset_box.offsets.contains(&Point::new(2, 0)));
            }

            #[test]
            fn t_piece() {
                let mut offset_box = PieceKind::T.get_unoriented_offset_box(&CONFIG);
                orient_offset_box(&mut offset_box, &Orientation::East);
                assert!(offset_box.offsets.contains(&Point::new(1, 2)));
                assert!(offset_box.offsets.contains(&Point::new(1, 1)));
                assert!(offset_box.offsets.contains(&Point::new(2, 1)));
                assert!(offset_box.offsets.contains(&Point::new(1, 0)));
            }

            #[test]
            fn z_piece() {
                let mut offset_box = PieceKind::Z.get_unoriented_offset_box(&CONFIG);
                orient_offset_box(&mut offset_box, &Orientation::East);
                assert!(offset_box.offsets.contains(&Point::new(2, 2)));
                assert!(offset_box.offsets.contains(&Point::new(1, 1)));
                assert!(offset_box.offsets.contains(&Point::new(2, 1)));
                assert!(offset_box.offsets.contains(&Point::new(1, 0)));
            }
        }

        mod west {
            use super::*;

            #[test]
            fn i_piece() {
                let mut offset_box = PieceKind::I.get_unoriented_offset_box(&CONFIG);
                orient_offset_box(&mut offset_box, &Orientation::West);
                assert!(offset_box.offsets.contains(&Point::new(1, 3)));
                assert!(offset_box.offsets.contains(&Point::new(1, 2)));
                assert!(offset_box.offsets.contains(&Point::new(1, 1)));
                assert!(offset_box.offsets.contains(&Point::new(1, 0)));
            }

            #[test]
            fn j_piece() {
                let mut offset_box = PieceKind::J.get_unoriented_offset_box(&CONFIG);
                orient_offset_box(&mut offset_box, &Orientation::West);
                assert!(offset_box.offsets.contains(&Point::new(1, 2)));
                assert!(offset_box.offsets.contains(&Point::new(1, 1)));
                assert!(offset_box.offsets.contains(&Point::new(0, 0)));
                assert!(offset_box.offsets.contains(&Point::new(1, 0)));
            }

            #[test]
            fn l_piece() {
                let mut offset_box = PieceKind::L.get_unoriented_offset_box(&CONFIG);
                orient_offset_box(&mut offset_box, &Orientation::West);
                assert!(offset_box.offsets.contains(&Point::new(0, 2)));
                assert!(offset_box.offsets.contains(&Point::new(1, 2)));
                assert!(offset_box.offsets.contains(&Point::new(1, 1)));
                assert!(offset_box.offsets.contains(&Point::new(1, 0)));
            }

            #[test]
            fn o_piece() {
                let mut offset_box = PieceKind::O.get_unoriented_offset_box(&CONFIG);
                orient_offset_box(&mut offset_box, &Orientation::West);
                assert!(offset_box.offsets.contains(&Point::new(1, 2)));
                assert!(offset_box.offsets.contains(&Point::new(2, 2)));
                assert!(offset_box.offsets.contains(&Point::new(1, 1)));
                assert!(offset_box.offsets.contains(&Point::new(2, 1)));
            }

            #[test]
            fn s_piece() {
                let mut offset_box = PieceKind::S.get_unoriented_offset_box(&CONFIG);
                orient_offset_box(&mut offset_box, &Orientation::West);
                assert!(offset_box.offsets.contains(&Point::new(0, 2)));
                assert!(offset_box.offsets.contains(&Point::new(0, 1)));
                assert!(offset_box.offsets.contains(&Point::new(1, 1)));
                assert!(offset_box.offsets.contains(&Point::new(1, 0)));
            }

            #[test]
            fn t_piece() {
                let mut offset_box = PieceKind::T.get_unoriented_offset_box(&CONFIG);
                orient_offset_box(&mut offset_box, &Orientation::West);
                assert!(offset_box.offsets.contains(&Point::new(1, 2)));
                assert!(offset_box.offsets.contains(&Point::new(0, 1)));
                assert!(offset_box.offsets.contains(&Point::new(1, 1)));
                assert!(offset_box.offsets.contains(&Point::new(1, 0)));
            }

            #[test]
            fn z_piece() {
                let mut offset_box = PieceKind::Z.get_unoriented_offset_box(&CONFIG);
                orient_offset_box(&mut offset_box, &Orientation::West);
                assert!(offset_box.offsets.contains(&Point::new(1, 2)));
                assert!(offset_box.offsets.contains(&Point::new(0, 1)));
                assert!(offset_box.offsets.contains(&Point::new(1, 1)));
                assert!(offset_box.offsets.contains(&Point::new(0, 0)));
            }
        }
    }
}
