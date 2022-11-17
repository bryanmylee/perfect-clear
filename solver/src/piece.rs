use crate::{
    config::{Config, RotationSystem},
    point::Point,
    rotation::Orientation,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PieceKind {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

type PieceOffsets = [Point<isize>; 4];

impl PieceKind {
    pub fn get_spawn_point(&self, config: &Config) -> Point<isize> {
        match config.rotation_system {
            RotationSystem::SRS => match self {
                PieceKind::I => Point { x: 3, y: 18 },
                PieceKind::J => Point { x: 3, y: 19 },
                PieceKind::L => Point { x: 3, y: 19 },
                PieceKind::O => Point { x: 3, y: 19 },
                PieceKind::S => Point { x: 3, y: 19 },
                PieceKind::T => Point { x: 3, y: 19 },
                PieceKind::Z => Point { x: 3, y: 19 },
            },
        }
    }

    fn get_unoriented_offset_box(&self, config: &Config) -> PieceOffsetBox {
        PieceOffsetBox {
            offsets: self.get_position_offsets(config),
            bounding_box_size: self.get_bounding_box_size(config),
        }
    }

    fn get_position_offsets(&self, _config: &Config) -> PieceOffsets {
        match self {
            PieceKind::I => [
                Point { x: 0, y: 2 },
                Point { x: 1, y: 2 },
                Point { x: 2, y: 2 },
                Point { x: 3, y: 2 },
            ],
            PieceKind::J => [
                Point { x: 0, y: 2 },
                Point { x: 0, y: 1 },
                Point { x: 1, y: 1 },
                Point { x: 2, y: 1 },
            ],
            PieceKind::L => [
                Point { x: 2, y: 2 },
                Point { x: 0, y: 1 },
                Point { x: 1, y: 1 },
                Point { x: 2, y: 1 },
            ],
            PieceKind::O => [
                Point { x: 1, y: 2 },
                Point { x: 2, y: 2 },
                Point { x: 1, y: 1 },
                Point { x: 2, y: 1 },
            ],
            PieceKind::S => [
                Point { x: 1, y: 2 },
                Point { x: 2, y: 2 },
                Point { x: 0, y: 1 },
                Point { x: 1, y: 1 },
            ],
            PieceKind::T => [
                Point { x: 1, y: 2 },
                Point { x: 0, y: 1 },
                Point { x: 1, y: 1 },
                Point { x: 2, y: 1 },
            ],
            PieceKind::Z => [
                Point { x: 0, y: 2 },
                Point { x: 1, y: 2 },
                Point { x: 1, y: 1 },
                Point { x: 2, y: 1 },
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

#[derive(Debug, Clone, PartialEq)]
pub struct Piece {
    pub kind: PieceKind,
    /**
    The bottom-left-most corner of the **bounding box**.
    */
    pub position: Point<isize>,
    pub orientation: Orientation,
}

impl Piece {
    pub fn spawn(kind: &PieceKind, config: &Config) -> Piece {
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
    offsets: PieceOffsets,
    bounding_box_size: usize,
}

/**
The four points on the board represented by a piece.
*/
pub type PiecePoints = [Point<isize>; 4];

impl Piece {
    pub fn get_points(&self, config: &Config) -> PiecePoints {
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
                position: Point { x: 3, y: 18 },
            };
            assert_eq!(
                piece.get_points(&CONFIG),
                [
                    Point { x: 3, y: 20 },
                    Point { x: 3, y: 19 },
                    Point { x: 4, y: 19 },
                    Point { x: 5, y: 19 },
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
                assert!(offset_box.offsets.contains(&Point { x: 0, y: 2 }));
                assert!(offset_box.offsets.contains(&Point { x: 1, y: 2 }));
                assert!(offset_box.offsets.contains(&Point { x: 2, y: 2 }));
                assert!(offset_box.offsets.contains(&Point { x: 3, y: 2 }));
            }

            #[test]
            fn j_piece() {
                let mut offset_box = PieceKind::J.get_unoriented_offset_box(&CONFIG);
                orient_offset_box(&mut offset_box, &Orientation::North);
                assert!(offset_box.offsets.contains(&Point { x: 0, y: 2 }));
                assert!(offset_box.offsets.contains(&Point { x: 0, y: 1 }));
                assert!(offset_box.offsets.contains(&Point { x: 1, y: 1 }));
                assert!(offset_box.offsets.contains(&Point { x: 2, y: 1 }));
            }

            #[test]
            fn l_piece() {
                let mut offset_box = PieceKind::L.get_unoriented_offset_box(&CONFIG);
                orient_offset_box(&mut offset_box, &Orientation::North);
                assert!(offset_box.offsets.contains(&Point { x: 2, y: 2 }));
                assert!(offset_box.offsets.contains(&Point { x: 0, y: 1 }));
                assert!(offset_box.offsets.contains(&Point { x: 1, y: 1 }));
                assert!(offset_box.offsets.contains(&Point { x: 2, y: 1 }));
            }

            #[test]
            fn o_piece() {
                let mut offset_box = PieceKind::O.get_unoriented_offset_box(&CONFIG);
                orient_offset_box(&mut offset_box, &Orientation::North);
                assert!(offset_box.offsets.contains(&Point { x: 1, y: 2 }));
                assert!(offset_box.offsets.contains(&Point { x: 2, y: 2 }));
                assert!(offset_box.offsets.contains(&Point { x: 1, y: 1 }));
                assert!(offset_box.offsets.contains(&Point { x: 2, y: 1 }));
            }

            #[test]
            fn s_piece() {
                let mut offset_box = PieceKind::S.get_unoriented_offset_box(&CONFIG);
                orient_offset_box(&mut offset_box, &Orientation::North);
                assert!(offset_box.offsets.contains(&Point { x: 1, y: 2 }));
                assert!(offset_box.offsets.contains(&Point { x: 2, y: 2 }));
                assert!(offset_box.offsets.contains(&Point { x: 0, y: 1 }));
                assert!(offset_box.offsets.contains(&Point { x: 1, y: 1 }));
            }

            #[test]
            fn t_piece() {
                let mut offset_box = PieceKind::T.get_unoriented_offset_box(&CONFIG);
                orient_offset_box(&mut offset_box, &Orientation::North);
                assert!(offset_box.offsets.contains(&Point { x: 1, y: 2 }));
                assert!(offset_box.offsets.contains(&Point { x: 0, y: 1 }));
                assert!(offset_box.offsets.contains(&Point { x: 1, y: 1 }));
                assert!(offset_box.offsets.contains(&Point { x: 2, y: 1 }));
            }

            #[test]
            fn z_piece() {
                let mut offset_box = PieceKind::Z.get_unoriented_offset_box(&CONFIG);
                orient_offset_box(&mut offset_box, &Orientation::North);
                assert!(offset_box.offsets.contains(&Point { x: 0, y: 2 }));
                assert!(offset_box.offsets.contains(&Point { x: 1, y: 2 }));
                assert!(offset_box.offsets.contains(&Point { x: 1, y: 1 }));
                assert!(offset_box.offsets.contains(&Point { x: 2, y: 1 }));
            }
        }

        mod south {
            use super::*;

            #[test]
            fn i_piece() {
                let mut offset_box = PieceKind::I.get_unoriented_offset_box(&CONFIG);
                orient_offset_box(&mut offset_box, &Orientation::South);
                assert!(offset_box.offsets.contains(&Point { x: 0, y: 1 }));
                assert!(offset_box.offsets.contains(&Point { x: 1, y: 1 }));
                assert!(offset_box.offsets.contains(&Point { x: 2, y: 1 }));
                assert!(offset_box.offsets.contains(&Point { x: 3, y: 1 }));
            }

            #[test]
            fn j_piece() {
                let mut offset_box = PieceKind::J.get_unoriented_offset_box(&CONFIG);
                orient_offset_box(&mut offset_box, &Orientation::South);
                assert!(offset_box.offsets.contains(&Point { x: 0, y: 1 }));
                assert!(offset_box.offsets.contains(&Point { x: 1, y: 1 }));
                assert!(offset_box.offsets.contains(&Point { x: 2, y: 1 }));
                assert!(offset_box.offsets.contains(&Point { x: 2, y: 0 }));
            }

            #[test]
            fn l_piece() {
                let mut offset_box = PieceKind::L.get_unoriented_offset_box(&CONFIG);
                orient_offset_box(&mut offset_box, &Orientation::South);
                assert!(offset_box.offsets.contains(&Point { x: 0, y: 1 }));
                assert!(offset_box.offsets.contains(&Point { x: 1, y: 1 }));
                assert!(offset_box.offsets.contains(&Point { x: 2, y: 1 }));
                assert!(offset_box.offsets.contains(&Point { x: 0, y: 0 }));
            }

            #[test]
            fn o_piece() {
                let mut offset_box = PieceKind::O.get_unoriented_offset_box(&CONFIG);
                orient_offset_box(&mut offset_box, &Orientation::South);
                assert!(offset_box.offsets.contains(&Point { x: 1, y: 2 }));
                assert!(offset_box.offsets.contains(&Point { x: 2, y: 2 }));
                assert!(offset_box.offsets.contains(&Point { x: 1, y: 1 }));
                assert!(offset_box.offsets.contains(&Point { x: 2, y: 1 }));
            }

            #[test]
            fn s_piece() {
                let mut offset_box = PieceKind::S.get_unoriented_offset_box(&CONFIG);
                orient_offset_box(&mut offset_box, &Orientation::South);
                assert!(offset_box.offsets.contains(&Point { x: 1, y: 1 }));
                assert!(offset_box.offsets.contains(&Point { x: 2, y: 1 }));
                assert!(offset_box.offsets.contains(&Point { x: 0, y: 0 }));
                assert!(offset_box.offsets.contains(&Point { x: 1, y: 0 }));
            }

            #[test]
            fn t_piece() {
                let mut offset_box = PieceKind::T.get_unoriented_offset_box(&CONFIG);
                orient_offset_box(&mut offset_box, &Orientation::South);
                assert!(offset_box.offsets.contains(&Point { x: 0, y: 1 }));
                assert!(offset_box.offsets.contains(&Point { x: 1, y: 1 }));
                assert!(offset_box.offsets.contains(&Point { x: 2, y: 1 }));
                assert!(offset_box.offsets.contains(&Point { x: 1, y: 0 }));
            }

            #[test]
            fn z_piece() {
                let mut offset_box = PieceKind::Z.get_unoriented_offset_box(&CONFIG);
                orient_offset_box(&mut offset_box, &Orientation::South);
                assert!(offset_box.offsets.contains(&Point { x: 0, y: 1 }));
                assert!(offset_box.offsets.contains(&Point { x: 1, y: 1 }));
                assert!(offset_box.offsets.contains(&Point { x: 1, y: 0 }));
                assert!(offset_box.offsets.contains(&Point { x: 2, y: 0 }));
            }
        }

        mod east {
            use super::*;

            #[test]
            fn i_piece() {
                let mut offset_box = PieceKind::I.get_unoriented_offset_box(&CONFIG);
                orient_offset_box(&mut offset_box, &Orientation::East);
                assert!(offset_box.offsets.contains(&Point { x: 2, y: 3 }));
                assert!(offset_box.offsets.contains(&Point { x: 2, y: 2 }));
                assert!(offset_box.offsets.contains(&Point { x: 2, y: 1 }));
                assert!(offset_box.offsets.contains(&Point { x: 2, y: 0 }));
            }

            #[test]
            fn j_piece() {
                let mut offset_box = PieceKind::J.get_unoriented_offset_box(&CONFIG);
                orient_offset_box(&mut offset_box, &Orientation::East);
                assert!(offset_box.offsets.contains(&Point { x: 1, y: 2 }));
                assert!(offset_box.offsets.contains(&Point { x: 2, y: 2 }));
                assert!(offset_box.offsets.contains(&Point { x: 1, y: 1 }));
                assert!(offset_box.offsets.contains(&Point { x: 1, y: 0 }));
            }

            #[test]
            fn l_piece() {
                let mut offset_box = PieceKind::L.get_unoriented_offset_box(&CONFIG);
                orient_offset_box(&mut offset_box, &Orientation::East);
                assert!(offset_box.offsets.contains(&Point { x: 1, y: 2 }));
                assert!(offset_box.offsets.contains(&Point { x: 1, y: 1 }));
                assert!(offset_box.offsets.contains(&Point { x: 1, y: 0 }));
                assert!(offset_box.offsets.contains(&Point { x: 2, y: 0 }));
            }

            #[test]
            fn o_piece() {
                let mut offset_box = PieceKind::O.get_unoriented_offset_box(&CONFIG);
                orient_offset_box(&mut offset_box, &Orientation::East);
                assert!(offset_box.offsets.contains(&Point { x: 1, y: 2 }));
                assert!(offset_box.offsets.contains(&Point { x: 2, y: 2 }));
                assert!(offset_box.offsets.contains(&Point { x: 1, y: 1 }));
                assert!(offset_box.offsets.contains(&Point { x: 2, y: 1 }));
            }

            #[test]
            fn s_piece() {
                let mut offset_box = PieceKind::S.get_unoriented_offset_box(&CONFIG);
                orient_offset_box(&mut offset_box, &Orientation::East);
                assert!(offset_box.offsets.contains(&Point { x: 1, y: 2 }));
                assert!(offset_box.offsets.contains(&Point { x: 1, y: 1 }));
                assert!(offset_box.offsets.contains(&Point { x: 2, y: 1 }));
                assert!(offset_box.offsets.contains(&Point { x: 2, y: 0 }));
            }

            #[test]
            fn t_piece() {
                let mut offset_box = PieceKind::T.get_unoriented_offset_box(&CONFIG);
                orient_offset_box(&mut offset_box, &Orientation::East);
                assert!(offset_box.offsets.contains(&Point { x: 1, y: 2 }));
                assert!(offset_box.offsets.contains(&Point { x: 1, y: 1 }));
                assert!(offset_box.offsets.contains(&Point { x: 2, y: 1 }));
                assert!(offset_box.offsets.contains(&Point { x: 1, y: 0 }));
            }

            #[test]
            fn z_piece() {
                let mut offset_box = PieceKind::Z.get_unoriented_offset_box(&CONFIG);
                orient_offset_box(&mut offset_box, &Orientation::East);
                assert!(offset_box.offsets.contains(&Point { x: 2, y: 2 }));
                assert!(offset_box.offsets.contains(&Point { x: 1, y: 1 }));
                assert!(offset_box.offsets.contains(&Point { x: 2, y: 1 }));
                assert!(offset_box.offsets.contains(&Point { x: 1, y: 0 }));
            }
        }

        mod west {
            use super::*;

            #[test]
            fn i_piece() {
                let mut offset_box = PieceKind::I.get_unoriented_offset_box(&CONFIG);
                orient_offset_box(&mut offset_box, &Orientation::West);
                assert!(offset_box.offsets.contains(&Point { x: 1, y: 3 }));
                assert!(offset_box.offsets.contains(&Point { x: 1, y: 2 }));
                assert!(offset_box.offsets.contains(&Point { x: 1, y: 1 }));
                assert!(offset_box.offsets.contains(&Point { x: 1, y: 0 }));
            }

            #[test]
            fn j_piece() {
                let mut offset_box = PieceKind::J.get_unoriented_offset_box(&CONFIG);
                orient_offset_box(&mut offset_box, &Orientation::West);
                assert!(offset_box.offsets.contains(&Point { x: 1, y: 2 }));
                assert!(offset_box.offsets.contains(&Point { x: 1, y: 1 }));
                assert!(offset_box.offsets.contains(&Point { x: 0, y: 0 }));
                assert!(offset_box.offsets.contains(&Point { x: 1, y: 0 }));
            }

            #[test]
            fn l_piece() {
                let mut offset_box = PieceKind::L.get_unoriented_offset_box(&CONFIG);
                orient_offset_box(&mut offset_box, &Orientation::West);
                assert!(offset_box.offsets.contains(&Point { x: 0, y: 2 }));
                assert!(offset_box.offsets.contains(&Point { x: 1, y: 2 }));
                assert!(offset_box.offsets.contains(&Point { x: 1, y: 1 }));
                assert!(offset_box.offsets.contains(&Point { x: 1, y: 0 }));
            }

            #[test]
            fn o_piece() {
                let mut offset_box = PieceKind::O.get_unoriented_offset_box(&CONFIG);
                orient_offset_box(&mut offset_box, &Orientation::West);
                assert!(offset_box.offsets.contains(&Point { x: 1, y: 2 }));
                assert!(offset_box.offsets.contains(&Point { x: 2, y: 2 }));
                assert!(offset_box.offsets.contains(&Point { x: 1, y: 1 }));
                assert!(offset_box.offsets.contains(&Point { x: 2, y: 1 }));
            }

            #[test]
            fn s_piece() {
                let mut offset_box = PieceKind::S.get_unoriented_offset_box(&CONFIG);
                orient_offset_box(&mut offset_box, &Orientation::West);
                assert!(offset_box.offsets.contains(&Point { x: 0, y: 2 }));
                assert!(offset_box.offsets.contains(&Point { x: 0, y: 1 }));
                assert!(offset_box.offsets.contains(&Point { x: 1, y: 1 }));
                assert!(offset_box.offsets.contains(&Point { x: 1, y: 0 }));
            }

            #[test]
            fn t_piece() {
                let mut offset_box = PieceKind::T.get_unoriented_offset_box(&CONFIG);
                orient_offset_box(&mut offset_box, &Orientation::West);
                assert!(offset_box.offsets.contains(&Point { x: 1, y: 2 }));
                assert!(offset_box.offsets.contains(&Point { x: 0, y: 1 }));
                assert!(offset_box.offsets.contains(&Point { x: 1, y: 1 }));
                assert!(offset_box.offsets.contains(&Point { x: 1, y: 0 }));
            }

            #[test]
            fn z_piece() {
                let mut offset_box = PieceKind::Z.get_unoriented_offset_box(&CONFIG);
                orient_offset_box(&mut offset_box, &Orientation::West);
                assert!(offset_box.offsets.contains(&Point { x: 1, y: 2 }));
                assert!(offset_box.offsets.contains(&Point { x: 0, y: 1 }));
                assert!(offset_box.offsets.contains(&Point { x: 1, y: 1 }));
                assert!(offset_box.offsets.contains(&Point { x: 0, y: 0 }));
            }
        }
    }
}
