use crate::{board::Board, config::Config, point::Point};

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
    /**
    The top-left-most corner of the **bounding box**.
    */
    pub position: Point<isize>,
    pub orientation: Orientation,
}

#[derive(Debug, Clone)]
pub enum Orientation {
    North,
    South,
    East,
    West,
}

/**
Pieces can be represented by four points from the top-left corner of a bounding box.
*/
pub struct PieceFill {
    position_offsets: [Point<isize>; 4],
    bounding_box_size: usize,
}

impl Piece {
    pub fn get_board_fill(&self, config: &Config) -> Board {
        let mut piece_fill = get_piece_fill(&self.kind, config);
        orient_piece_fill(&mut piece_fill, &self.orientation);
        let mut board = Board::empty_board();
        for piece_offset in piece_fill.position_offsets {
            let piece_position = self.position + piece_offset;
            board.fill(&piece_position);
        }
        board
    }
}

fn get_piece_fill(kind: &PieceKind, config: &Config) -> PieceFill {
    PieceFill {
        position_offsets: get_position_offsets(kind, config),
        bounding_box_size: get_bounding_box_size(kind, config),
    }
}

fn get_position_offsets(kind: &PieceKind, _config: &Config) -> [Point<isize>; 4] {
    match kind {
        PieceKind::I => [
            Point { x: 0, y: 1 },
            Point { x: 1, y: 1 },
            Point { x: 2, y: 1 },
            Point { x: 3, y: 1 },
        ],
        PieceKind::J => [
            Point { x: 0, y: 0 },
            Point { x: 0, y: 1 },
            Point { x: 1, y: 1 },
            Point { x: 2, y: 1 },
        ],
        PieceKind::L => [
            Point { x: 2, y: 0 },
            Point { x: 0, y: 1 },
            Point { x: 1, y: 1 },
            Point { x: 2, y: 1 },
        ],
        PieceKind::O => [
            Point { x: 1, y: 1 },
            Point { x: 2, y: 1 },
            Point { x: 1, y: 2 },
            Point { x: 2, y: 2 },
        ],
        PieceKind::S => [
            Point { x: 1, y: 0 },
            Point { x: 2, y: 0 },
            Point { x: 0, y: 1 },
            Point { x: 1, y: 1 },
        ],
        PieceKind::T => [
            Point { x: 1, y: 0 },
            Point { x: 0, y: 1 },
            Point { x: 1, y: 1 },
            Point { x: 2, y: 1 },
        ],
        PieceKind::Z => [
            Point { x: 0, y: 0 },
            Point { x: 1, y: 0 },
            Point { x: 1, y: 1 },
            Point { x: 2, y: 1 },
        ],
    }
}

fn get_bounding_box_size(kind: &PieceKind, _config: &Config) -> usize {
    match kind {
        PieceKind::I => 4,
        PieceKind::J => 3,
        PieceKind::L => 3,
        PieceKind::O => 4,
        PieceKind::S => 3,
        PieceKind::T => 3,
        PieceKind::Z => 3,
    }
}

fn orient_piece_fill(piece_fill: &mut PieceFill, orientation: &Orientation) {
    let size_minus_one = (piece_fill.bounding_box_size - 1) as isize;
    match orientation {
        Orientation::North => {}
        Orientation::South => {
            for offset in piece_fill.position_offsets.iter_mut() {
                offset.x = size_minus_one - offset.x;
                offset.y = size_minus_one - offset.y;
            }
        }
        Orientation::East => {
            for offset in piece_fill.position_offsets.iter_mut() {
                (offset.x, offset.y) = (size_minus_one - offset.y, offset.x);
            }
        }
        Orientation::West => {
            for offset in piece_fill.position_offsets.iter_mut() {
                (offset.x, offset.y) = (offset.y, size_minus_one - offset.x);
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

    mod orient_piece_fill {
        use super::*;

        mod north {
            use super::*;

            #[test]
            fn i_piece() {
                let mut piece_fill = get_piece_fill(&PieceKind::I, &CONFIG);
                orient_piece_fill(&mut piece_fill, &Orientation::North);
                assert!(piece_fill.position_offsets.contains(&Point { x: 0, y: 1 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 1, y: 1 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 2, y: 1 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 3, y: 1 }));
            }

            #[test]
            fn j_piece() {
                let mut piece_fill = get_piece_fill(&PieceKind::J, &CONFIG);
                orient_piece_fill(&mut piece_fill, &Orientation::North);
                assert!(piece_fill.position_offsets.contains(&Point { x: 0, y: 0 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 0, y: 1 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 1, y: 1 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 2, y: 1 }));
            }

            #[test]
            fn l_piece() {
                let mut piece_fill = get_piece_fill(&PieceKind::L, &CONFIG);
                orient_piece_fill(&mut piece_fill, &Orientation::North);
                assert!(piece_fill.position_offsets.contains(&Point { x: 2, y: 0 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 0, y: 1 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 1, y: 1 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 2, y: 1 }));
            }

            #[test]
            fn o_piece() {
                let mut piece_fill = get_piece_fill(&PieceKind::O, &CONFIG);
                orient_piece_fill(&mut piece_fill, &Orientation::North);
                assert!(piece_fill.position_offsets.contains(&Point { x: 1, y: 1 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 2, y: 1 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 1, y: 2 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 2, y: 2 }));
            }

            #[test]
            fn s_piece() {
                let mut piece_fill = get_piece_fill(&PieceKind::S, &CONFIG);
                orient_piece_fill(&mut piece_fill, &Orientation::North);
                assert!(piece_fill.position_offsets.contains(&Point { x: 1, y: 0 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 2, y: 0 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 0, y: 1 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 1, y: 1 }));
            }

            #[test]
            fn t_piece() {
                let mut piece_fill = get_piece_fill(&PieceKind::T, &CONFIG);
                orient_piece_fill(&mut piece_fill, &Orientation::North);
                assert!(piece_fill.position_offsets.contains(&Point { x: 1, y: 0 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 0, y: 1 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 1, y: 1 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 2, y: 1 }));
            }

            #[test]
            fn z_piece() {
                let mut piece_fill = get_piece_fill(&PieceKind::Z, &CONFIG);
                orient_piece_fill(&mut piece_fill, &Orientation::North);
                assert!(piece_fill.position_offsets.contains(&Point { x: 0, y: 0 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 1, y: 0 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 1, y: 1 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 2, y: 1 }));
            }
        }

        mod south {
            use super::*;

            #[test]
            fn i_piece() {
                let mut piece_fill = get_piece_fill(&PieceKind::I, &CONFIG);
                orient_piece_fill(&mut piece_fill, &Orientation::South);
                assert!(piece_fill.position_offsets.contains(&Point { x: 0, y: 2 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 1, y: 2 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 2, y: 2 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 3, y: 2 }));
            }

            #[test]
            fn j_piece() {
                let mut piece_fill = get_piece_fill(&PieceKind::J, &CONFIG);
                orient_piece_fill(&mut piece_fill, &Orientation::South);
                assert!(piece_fill.position_offsets.contains(&Point { x: 0, y: 1 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 1, y: 1 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 2, y: 1 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 2, y: 2 }));
            }

            #[test]
            fn l_piece() {
                let mut piece_fill = get_piece_fill(&PieceKind::L, &CONFIG);
                orient_piece_fill(&mut piece_fill, &Orientation::South);
                assert!(piece_fill.position_offsets.contains(&Point { x: 0, y: 1 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 1, y: 1 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 2, y: 1 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 0, y: 2 }));
            }

            #[test]
            fn o_piece() {
                let mut piece_fill = get_piece_fill(&PieceKind::O, &CONFIG);
                orient_piece_fill(&mut piece_fill, &Orientation::South);
                assert!(piece_fill.position_offsets.contains(&Point { x: 1, y: 1 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 2, y: 1 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 1, y: 2 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 2, y: 2 }));
            }

            #[test]
            fn s_piece() {
                let mut piece_fill = get_piece_fill(&PieceKind::S, &CONFIG);
                orient_piece_fill(&mut piece_fill, &Orientation::South);
                assert!(piece_fill.position_offsets.contains(&Point { x: 1, y: 1 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 2, y: 1 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 0, y: 2 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 1, y: 2 }));
            }

            #[test]
            fn t_piece() {
                let mut piece_fill = get_piece_fill(&PieceKind::T, &CONFIG);
                orient_piece_fill(&mut piece_fill, &Orientation::South);
                assert!(piece_fill.position_offsets.contains(&Point { x: 0, y: 1 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 1, y: 1 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 2, y: 1 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 1, y: 2 }));
            }

            #[test]
            fn z_piece() {
                let mut piece_fill = get_piece_fill(&PieceKind::Z, &CONFIG);
                orient_piece_fill(&mut piece_fill, &Orientation::South);
                assert!(piece_fill.position_offsets.contains(&Point { x: 0, y: 1 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 1, y: 1 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 1, y: 2 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 2, y: 2 }));
            }
        }

        mod east {
            use super::*;

            #[test]
            fn i_piece() {
                let mut piece_fill = get_piece_fill(&PieceKind::I, &CONFIG);
                orient_piece_fill(&mut piece_fill, &Orientation::East);
                assert!(piece_fill.position_offsets.contains(&Point { x: 2, y: 0 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 2, y: 1 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 2, y: 2 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 2, y: 3 }));
            }

            #[test]
            fn j_piece() {
                let mut piece_fill = get_piece_fill(&PieceKind::J, &CONFIG);
                orient_piece_fill(&mut piece_fill, &Orientation::East);
                assert!(piece_fill.position_offsets.contains(&Point { x: 1, y: 0 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 2, y: 0 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 1, y: 1 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 1, y: 2 }));
            }

            #[test]
            fn l_piece() {
                let mut piece_fill = get_piece_fill(&PieceKind::L, &CONFIG);
                orient_piece_fill(&mut piece_fill, &Orientation::East);
                assert!(piece_fill.position_offsets.contains(&Point { x: 1, y: 0 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 1, y: 1 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 1, y: 2 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 2, y: 2 }));
            }

            #[test]
            fn o_piece() {
                let mut piece_fill = get_piece_fill(&PieceKind::O, &CONFIG);
                orient_piece_fill(&mut piece_fill, &Orientation::East);
                assert!(piece_fill.position_offsets.contains(&Point { x: 1, y: 1 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 2, y: 1 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 1, y: 2 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 2, y: 2 }));
            }

            #[test]
            fn s_piece() {
                let mut piece_fill = get_piece_fill(&PieceKind::S, &CONFIG);
                orient_piece_fill(&mut piece_fill, &Orientation::East);
                assert!(piece_fill.position_offsets.contains(&Point { x: 1, y: 0 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 1, y: 1 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 2, y: 1 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 2, y: 2 }));
            }

            #[test]
            fn t_piece() {
                let mut piece_fill = get_piece_fill(&PieceKind::T, &CONFIG);
                orient_piece_fill(&mut piece_fill, &Orientation::East);
                assert!(piece_fill.position_offsets.contains(&Point { x: 1, y: 0 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 1, y: 1 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 2, y: 1 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 1, y: 2 }));
            }

            #[test]
            fn z_piece() {
                let mut piece_fill = get_piece_fill(&PieceKind::Z, &CONFIG);
                orient_piece_fill(&mut piece_fill, &Orientation::East);
                assert!(piece_fill.position_offsets.contains(&Point { x: 2, y: 0 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 1, y: 1 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 2, y: 1 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 1, y: 2 }));
            }
        }

        mod west {
            use super::*;

            #[test]
            fn i_piece() {
                let mut piece_fill = get_piece_fill(&PieceKind::I, &CONFIG);
                orient_piece_fill(&mut piece_fill, &Orientation::West);
                assert!(piece_fill.position_offsets.contains(&Point { x: 1, y: 0 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 1, y: 1 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 1, y: 2 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 1, y: 3 }));
            }

            #[test]
            fn j_piece() {
                let mut piece_fill = get_piece_fill(&PieceKind::J, &CONFIG);
                orient_piece_fill(&mut piece_fill, &Orientation::West);
                assert!(piece_fill.position_offsets.contains(&Point { x: 1, y: 0 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 1, y: 1 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 0, y: 2 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 1, y: 2 }));
            }

            #[test]
            fn l_piece() {
                let mut piece_fill = get_piece_fill(&PieceKind::L, &CONFIG);
                orient_piece_fill(&mut piece_fill, &Orientation::West);
                assert!(piece_fill.position_offsets.contains(&Point { x: 0, y: 0 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 1, y: 0 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 1, y: 1 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 1, y: 2 }));
            }

            #[test]
            fn o_piece() {
                let mut piece_fill = get_piece_fill(&PieceKind::O, &CONFIG);
                orient_piece_fill(&mut piece_fill, &Orientation::West);
                assert!(piece_fill.position_offsets.contains(&Point { x: 1, y: 1 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 2, y: 1 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 1, y: 2 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 2, y: 2 }));
            }

            #[test]
            fn s_piece() {
                let mut piece_fill = get_piece_fill(&PieceKind::S, &CONFIG);
                orient_piece_fill(&mut piece_fill, &Orientation::West);
                assert!(piece_fill.position_offsets.contains(&Point { x: 0, y: 0 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 0, y: 1 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 1, y: 1 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 1, y: 2 }));
            }

            #[test]
            fn t_piece() {
                let mut piece_fill = get_piece_fill(&PieceKind::T, &CONFIG);
                orient_piece_fill(&mut piece_fill, &Orientation::West);
                assert!(piece_fill.position_offsets.contains(&Point { x: 1, y: 0 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 0, y: 1 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 1, y: 1 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 1, y: 2 }));
            }

            #[test]
            fn z_piece() {
                let mut piece_fill = get_piece_fill(&PieceKind::Z, &CONFIG);
                orient_piece_fill(&mut piece_fill, &Orientation::West);
                assert!(piece_fill.position_offsets.contains(&Point { x: 1, y: 0 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 0, y: 1 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 1, y: 1 }));
                assert!(piece_fill.position_offsets.contains(&Point { x: 0, y: 2 }));
            }
        }
    }
}
