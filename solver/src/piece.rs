use crate::{
    config::{Config, RotationSystem},
    point::Point,
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

    fn get_unoriented_offsets(&self, config: &Config) -> PieceOffsets {
        PieceOffsets {
            offsets: self.get_position_offsets(config),
            bounding_box_size: self.get_bounding_box_size(config),
        }
    }

    fn get_position_offsets(&self, _config: &Config) -> [Point<isize>; 4] {
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

#[derive(Debug, Clone, PartialEq)]
pub enum Orientation {
    North,
    South,
    East,
    West,
}

/**
Pieces can be represented by four points from the bottom-left corner of a bounding box.
*/
struct PieceOffsets {
    offsets: [Point<isize>; 4],
    bounding_box_size: usize,
}

/**
The four points on the board represented by a piece.
*/
pub type PiecePoints = [Point<isize>; 4];

impl Piece {
    pub fn get_points(&self, config: &Config) -> PiecePoints {
        let mut offsets = self.kind.get_unoriented_offsets(config);
        orient_offsets(&mut offsets, &self.orientation);
        for offset in offsets.offsets.iter_mut() {
            *offset += self.position;
        }
        offsets.offsets
    }
}

fn orient_offsets(unoriented_offsets: &mut PieceOffsets, orientation: &Orientation) {
    let size_minus_one = (unoriented_offsets.bounding_box_size - 1) as isize;
    match orientation {
        Orientation::North => {}
        Orientation::South => {
            for offset in unoriented_offsets.offsets.iter_mut() {
                offset.x = size_minus_one - offset.x;
                offset.y = size_minus_one - offset.y;
            }
        }
        Orientation::East => {
            for offset in unoriented_offsets.offsets.iter_mut() {
                (offset.x, offset.y) = (offset.y, size_minus_one - offset.x);
            }
        }
        Orientation::West => {
            for offset in unoriented_offsets.offsets.iter_mut() {
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
                let mut piece_offsets = PieceKind::I.get_unoriented_offsets(&CONFIG);
                orient_offsets(&mut piece_offsets, &Orientation::North);
                assert!(piece_offsets.offsets.contains(&Point { x: 0, y: 2 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 1, y: 2 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 2, y: 2 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 3, y: 2 }));
            }

            #[test]
            fn j_piece() {
                let mut piece_offsets = PieceKind::J.get_unoriented_offsets(&CONFIG);
                orient_offsets(&mut piece_offsets, &Orientation::North);
                assert!(piece_offsets.offsets.contains(&Point { x: 0, y: 2 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 0, y: 1 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 1, y: 1 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 2, y: 1 }));
            }

            #[test]
            fn l_piece() {
                let mut piece_offsets = PieceKind::L.get_unoriented_offsets(&CONFIG);
                orient_offsets(&mut piece_offsets, &Orientation::North);
                assert!(piece_offsets.offsets.contains(&Point { x: 2, y: 2 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 0, y: 1 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 1, y: 1 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 2, y: 1 }));
            }

            #[test]
            fn o_piece() {
                let mut piece_offsets = PieceKind::O.get_unoriented_offsets(&CONFIG);
                orient_offsets(&mut piece_offsets, &Orientation::North);
                assert!(piece_offsets.offsets.contains(&Point { x: 1, y: 2 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 2, y: 2 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 1, y: 1 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 2, y: 1 }));
            }

            #[test]
            fn s_piece() {
                let mut piece_offsets = PieceKind::S.get_unoriented_offsets(&CONFIG);
                orient_offsets(&mut piece_offsets, &Orientation::North);
                assert!(piece_offsets.offsets.contains(&Point { x: 1, y: 2 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 2, y: 2 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 0, y: 1 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 1, y: 1 }));
            }

            #[test]
            fn t_piece() {
                let mut piece_offsets = PieceKind::T.get_unoriented_offsets(&CONFIG);
                orient_offsets(&mut piece_offsets, &Orientation::North);
                assert!(piece_offsets.offsets.contains(&Point { x: 1, y: 2 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 0, y: 1 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 1, y: 1 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 2, y: 1 }));
            }

            #[test]
            fn z_piece() {
                let mut piece_offsets = PieceKind::Z.get_unoriented_offsets(&CONFIG);
                orient_offsets(&mut piece_offsets, &Orientation::North);
                assert!(piece_offsets.offsets.contains(&Point { x: 0, y: 2 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 1, y: 2 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 1, y: 1 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 2, y: 1 }));
            }
        }

        mod south {
            use super::*;

            #[test]
            fn i_piece() {
                let mut piece_offsets = PieceKind::I.get_unoriented_offsets(&CONFIG);
                orient_offsets(&mut piece_offsets, &Orientation::South);
                assert!(piece_offsets.offsets.contains(&Point { x: 0, y: 1 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 1, y: 1 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 2, y: 1 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 3, y: 1 }));
            }

            #[test]
            fn j_piece() {
                let mut piece_offsets = PieceKind::J.get_unoriented_offsets(&CONFIG);
                orient_offsets(&mut piece_offsets, &Orientation::South);
                assert!(piece_offsets.offsets.contains(&Point { x: 0, y: 1 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 1, y: 1 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 2, y: 1 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 2, y: 0 }));
            }

            #[test]
            fn l_piece() {
                let mut piece_offsets = PieceKind::L.get_unoriented_offsets(&CONFIG);
                orient_offsets(&mut piece_offsets, &Orientation::South);
                assert!(piece_offsets.offsets.contains(&Point { x: 0, y: 1 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 1, y: 1 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 2, y: 1 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 0, y: 0 }));
            }

            #[test]
            fn o_piece() {
                let mut piece_offsets = PieceKind::O.get_unoriented_offsets(&CONFIG);
                orient_offsets(&mut piece_offsets, &Orientation::South);
                assert!(piece_offsets.offsets.contains(&Point { x: 1, y: 2 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 2, y: 2 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 1, y: 1 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 2, y: 1 }));
            }

            #[test]
            fn s_piece() {
                let mut piece_offsets = PieceKind::S.get_unoriented_offsets(&CONFIG);
                orient_offsets(&mut piece_offsets, &Orientation::South);
                assert!(piece_offsets.offsets.contains(&Point { x: 1, y: 1 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 2, y: 1 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 0, y: 0 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 1, y: 0 }));
            }

            #[test]
            fn t_piece() {
                let mut piece_offsets = PieceKind::T.get_unoriented_offsets(&CONFIG);
                orient_offsets(&mut piece_offsets, &Orientation::South);
                assert!(piece_offsets.offsets.contains(&Point { x: 0, y: 1 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 1, y: 1 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 2, y: 1 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 1, y: 0 }));
            }

            #[test]
            fn z_piece() {
                let mut piece_offsets = PieceKind::Z.get_unoriented_offsets(&CONFIG);
                orient_offsets(&mut piece_offsets, &Orientation::South);
                assert!(piece_offsets.offsets.contains(&Point { x: 0, y: 1 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 1, y: 1 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 1, y: 0 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 2, y: 0 }));
            }
        }

        mod east {
            use super::*;

            #[test]
            fn i_piece() {
                let mut piece_offsets = PieceKind::I.get_unoriented_offsets(&CONFIG);
                orient_offsets(&mut piece_offsets, &Orientation::East);
                assert!(piece_offsets.offsets.contains(&Point { x: 2, y: 3 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 2, y: 2 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 2, y: 1 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 2, y: 0 }));
            }

            #[test]
            fn j_piece() {
                let mut piece_offsets = PieceKind::J.get_unoriented_offsets(&CONFIG);
                orient_offsets(&mut piece_offsets, &Orientation::East);
                assert!(piece_offsets.offsets.contains(&Point { x: 1, y: 2 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 2, y: 2 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 1, y: 1 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 1, y: 0 }));
            }

            #[test]
            fn l_piece() {
                let mut piece_offsets = PieceKind::L.get_unoriented_offsets(&CONFIG);
                orient_offsets(&mut piece_offsets, &Orientation::East);
                assert!(piece_offsets.offsets.contains(&Point { x: 1, y: 2 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 1, y: 1 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 1, y: 0 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 2, y: 0 }));
            }

            #[test]
            fn o_piece() {
                let mut piece_offsets = PieceKind::O.get_unoriented_offsets(&CONFIG);
                orient_offsets(&mut piece_offsets, &Orientation::East);
                assert!(piece_offsets.offsets.contains(&Point { x: 1, y: 2 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 2, y: 2 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 1, y: 1 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 2, y: 1 }));
            }

            #[test]
            fn s_piece() {
                let mut piece_offsets = PieceKind::S.get_unoriented_offsets(&CONFIG);
                orient_offsets(&mut piece_offsets, &Orientation::East);
                assert!(piece_offsets.offsets.contains(&Point { x: 1, y: 2 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 1, y: 1 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 2, y: 1 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 2, y: 0 }));
            }

            #[test]
            fn t_piece() {
                let mut piece_offsets = PieceKind::T.get_unoriented_offsets(&CONFIG);
                orient_offsets(&mut piece_offsets, &Orientation::East);
                assert!(piece_offsets.offsets.contains(&Point { x: 1, y: 2 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 1, y: 1 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 2, y: 1 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 1, y: 0 }));
            }

            #[test]
            fn z_piece() {
                let mut piece_offsets = PieceKind::Z.get_unoriented_offsets(&CONFIG);
                orient_offsets(&mut piece_offsets, &Orientation::East);
                assert!(piece_offsets.offsets.contains(&Point { x: 2, y: 2 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 1, y: 1 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 2, y: 1 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 1, y: 0 }));
            }
        }

        mod west {
            use super::*;

            #[test]
            fn i_piece() {
                let mut piece_offsets = PieceKind::I.get_unoriented_offsets(&CONFIG);
                orient_offsets(&mut piece_offsets, &Orientation::West);
                assert!(piece_offsets.offsets.contains(&Point { x: 1, y: 3 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 1, y: 2 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 1, y: 1 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 1, y: 0 }));
            }

            #[test]
            fn j_piece() {
                let mut piece_offsets = PieceKind::J.get_unoriented_offsets(&CONFIG);
                orient_offsets(&mut piece_offsets, &Orientation::West);
                assert!(piece_offsets.offsets.contains(&Point { x: 1, y: 2 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 1, y: 1 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 0, y: 0 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 1, y: 0 }));
            }

            #[test]
            fn l_piece() {
                let mut piece_offsets = PieceKind::L.get_unoriented_offsets(&CONFIG);
                orient_offsets(&mut piece_offsets, &Orientation::West);
                assert!(piece_offsets.offsets.contains(&Point { x: 0, y: 2 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 1, y: 2 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 1, y: 1 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 1, y: 0 }));
            }

            #[test]
            fn o_piece() {
                let mut piece_offsets = PieceKind::O.get_unoriented_offsets(&CONFIG);
                orient_offsets(&mut piece_offsets, &Orientation::West);
                assert!(piece_offsets.offsets.contains(&Point { x: 1, y: 2 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 2, y: 2 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 1, y: 1 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 2, y: 1 }));
            }

            #[test]
            fn s_piece() {
                let mut piece_offsets = PieceKind::S.get_unoriented_offsets(&CONFIG);
                orient_offsets(&mut piece_offsets, &Orientation::West);
                assert!(piece_offsets.offsets.contains(&Point { x: 0, y: 2 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 0, y: 1 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 1, y: 1 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 1, y: 0 }));
            }

            #[test]
            fn t_piece() {
                let mut piece_offsets = PieceKind::T.get_unoriented_offsets(&CONFIG);
                orient_offsets(&mut piece_offsets, &Orientation::West);
                assert!(piece_offsets.offsets.contains(&Point { x: 1, y: 2 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 0, y: 1 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 1, y: 1 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 1, y: 0 }));
            }

            #[test]
            fn z_piece() {
                let mut piece_offsets = PieceKind::Z.get_unoriented_offsets(&CONFIG);
                orient_offsets(&mut piece_offsets, &Orientation::West);
                assert!(piece_offsets.offsets.contains(&Point { x: 1, y: 2 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 0, y: 1 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 1, y: 1 }));
                assert!(piece_offsets.offsets.contains(&Point { x: 0, y: 0 }));
            }
        }
    }
}
