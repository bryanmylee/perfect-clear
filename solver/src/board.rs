use std::fmt;

use crate::{piece::PiecePoints, point::Point};

pub type BoardFill = [u64; 4];

#[derive(Clone, PartialEq, Eq)]
pub struct Board {
    /**
    A tetris board has 24 rows of 10 columns. We split the board into 4 segments of 6 rows to get
    60 cells in each segment. This lets us store the fill state of each segment as a bitfield.

    The segments are ordered from bottom to top and the cells in each segment are ordered from bottom-left to top-right.
    */
    fill: BoardFill,
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const BOARD_CAPACITY: usize = 24 * 14;
        let mut board_result = String::with_capacity(BOARD_CAPACITY);
        for y in (0..24).map(|y| 23 - y) {
            board_result.push_str(&format!("\n{:0>2} ", y));
            for x in 0..10 {
                board_result.push(if self.is_filled(&Point::new(x, y)) {
                    'x'
                } else {
                    '-'
                });
            }
        }
        f.write_str(&board_result)
    }
}

impl Board {
    pub fn empty_board() -> Board {
        Board {
            fill: [
                0b0000000000_0000000000_0000000000_0000000000_0000000000_0000000000,
                0b0000000000_0000000000_0000000000_0000000000_0000000000_0000000000,
                0b0000000000_0000000000_0000000000_0000000000_0000000000_0000000000,
                0b0000000000_0000000000_0000000000_0000000000_0000000000_0000000000,
            ],
        }
    }

    pub fn filled_board() -> Board {
        Board {
            fill: [
                0b1111111111_1111111111_1111111111_1111111111_1111111111_1111111111,
                0b1111111111_1111111111_1111111111_1111111111_1111111111_1111111111,
                0b1111111111_1111111111_1111111111_1111111111_1111111111_1111111111,
                0b1111111111_1111111111_1111111111_1111111111_1111111111_1111111111,
            ],
        }
    }

    /**
    `{ x: 0, y: 0 }` starts on the bottom-left.

    For convenience, we treat `x: -1` and `x: 10` as filled for the kick-table.
    */
    pub fn is_filled(&self, at: &Point<isize>) -> bool {
        if at.x < 0 || at.x >= 10 || at.y < 0 {
            return true;
        }
        let y_segment_idx = at.y / 6;
        let y_segment = self.fill.get(y_segment_idx as usize);
        let Some(y_segment) = y_segment else {
            return false;
        };
        let y_idx = at.y % 6;
        (*y_segment >> (at.x + y_idx * 10)) & 0b1 == 1
    }

    pub fn fill(&mut self, point: &Point<isize>) {
        if point.x < 0 || point.x >= 10 || point.y < 0 || point.y >= 24 {
            return;
        }
        let y_segment_idx = point.y / 6;
        let y_segment = self.fill.get_mut(y_segment_idx as usize);
        let Some(y_segment) = y_segment else {
            return;
        };
        let y_idx = point.y % 6;
        *y_segment |= 0b1 << (point.x + y_idx * 10);
    }

    pub fn empty(&mut self, point: &Point<isize>) {
        if point.x < 0 || point.x >= 10 || point.y < 0 || point.y >= 24 {
            return;
        }
        let y_segment_idx = point.y / 6;
        let y_segment = self.fill.get_mut(y_segment_idx as usize);
        let Some(y_segment) = y_segment else {
            return;
        };
        let y_idx = point.y % 6;
        *y_segment &= !(0b1 << (point.x + y_idx * 10));
    }

    pub fn has_intersect(&self, other: &Board) -> bool {
        self.fill
            .iter()
            .zip(other.fill.iter())
            .any(|(a, b)| a & b > 0)
    }

    pub fn union(&mut self, other: &Board) {
        for (a, b) in self.fill.iter_mut().zip(other.fill.iter()) {
            *a |= b;
        }
    }

    pub fn can_fit(&self, piece_points: &PiecePoints) -> bool {
        piece_points.iter().all(|point| !self.is_filled(point))
    }

    pub fn can_place(&self, piece_points: &PiecePoints) -> bool {
        let offset = Point::new(0, -1);
        piece_points
            .iter()
            .any(|point| self.is_filled(&(*point + offset)))
    }

    pub fn fill_piece_points(&mut self, piece_points: &PiecePoints) {
        for point in piece_points {
            self.fill(point);
        }
    }

    pub fn is_line_filled(&self, y: isize) -> bool {
        (0..10).all(|x| self.is_filled(&Point::new(x, y)))
    }

    pub fn clear_filled_lines(&mut self) {
        let mut next_board = Board::empty_board();
        let mut next_y = 0;
        for y in 0..20 {
            if self.is_line_filled(y) {
                continue;
            }
            for x in 0..10 {
                if self.is_filled(&Point::new(x, y)) {
                    next_board.fill(&Point::new(x, next_y));
                }
            }
            next_y += 1;
        }
        self.fill = next_board.fill;
    }
}

#[cfg(test)]
mod tests {
    use crate::config::{Config, RotationSystem};

    use super::*;

    const CONFIG: Config = Config {
        rotation_system: RotationSystem::SRS,
    };

    fn assert_only_filled(board: &Board, fills: Vec<Point<isize>>) {
        for x in 0..10 {
            for y in 0..24 {
                let is_filled = fills.contains(&Point::new(x, y));
                assert_eq!(
                    board.is_filled(&Point::new(x, y)),
                    is_filled,
                    "Expected board to be {} at ({}, {})",
                    if is_filled { "filled" } else { "empty" },
                    x,
                    y
                );
            }
        }
    }

    fn assert_only_emptied(board: &Board, empties: Vec<Point<isize>>) {
        for x in 0..10 {
            for y in 0..24 {
                let is_empty = empties.contains(&Point::new(x, y));
                assert_eq!(
                    !board.is_filled(&Point::new(x, y)),
                    is_empty,
                    "Expected board to be {} at ({}, {})",
                    if is_empty { "empty" } else { "filled" },
                    x,
                    y
                );
            }
        }
    }

    mod is_filled {
        use super::*;

        #[test]
        fn detects_filled_and_empty_cells() {
            let board = Board {
                fill: [
                    0b0000000000_0000000000_0000000000_0000000001_1100000001_1101111011,
                    0b0000000000_0000000000_0000000000_0000000001_1100000001_1101111011,
                    0b0000000000_0000000000_0000000000_0000000000_0000000000_0000000000,
                    0b0000000000_0000000000_0000000000_0000000000_0000000000_0000000000,
                ],
            };

            assert_only_filled(
                &board,
                vec![
                    Point::new(0, 0),
                    Point::new(1, 0),
                    Point::new(3, 0),
                    Point::new(4, 0),
                    Point::new(5, 0),
                    Point::new(6, 0),
                    Point::new(8, 0),
                    Point::new(9, 0),
                    Point::new(0, 1),
                    Point::new(8, 1),
                    Point::new(9, 1),
                    Point::new(0, 2),
                    Point::new(0, 6),
                    Point::new(1, 6),
                    Point::new(3, 6),
                    Point::new(4, 6),
                    Point::new(5, 6),
                    Point::new(6, 6),
                    Point::new(8, 6),
                    Point::new(9, 6),
                    Point::new(0, 7),
                    Point::new(8, 7),
                    Point::new(9, 7),
                    Point::new(0, 8),
                ],
            );
        }

        #[test]
        fn walls_are_filled() {
            let board = Board::empty_board();

            for y in 0..24 {
                assert!(
                    board.is_filled(&Point::new(-1, y)),
                    "Expected left wall to be filled on line {}",
                    y
                );
                assert!(
                    board.is_filled(&Point::new(10, y)),
                    "Expected right wall to be filled on line {}",
                    y
                );
            }
        }

        #[test]
        fn floor_is_filled() {
            let board = Board::empty_board();

            for x in 0..10 {
                assert!(
                    board.is_filled(&Point::new(x, -1)),
                    "Expected floor to be filled on column {}",
                    x
                );
            }
        }
    }

    mod fill {
        use super::*;

        #[test]
        fn fills_cells() {
            let mut board = Board::empty_board();

            board.fill(&Point::new(0, 0));
            assert_only_filled(&board, vec![Point::new(0, 0)]);

            board.fill(&Point::new(9, 0));
            assert_only_filled(&board, vec![Point::new(0, 0), Point::new(9, 0)]);

            board.fill(&Point::new(0, 10));
            assert_only_filled(
                &board,
                vec![Point::new(0, 0), Point::new(9, 0), Point::new(0, 10)],
            );

            board.fill(&Point::new(9, 10));
            assert_only_filled(
                &board,
                vec![
                    Point::new(0, 0),
                    Point::new(9, 0),
                    Point::new(0, 10),
                    Point::new(9, 10),
                ],
            );

            board.fill(&Point::new(0, 20));
            assert_only_filled(
                &board,
                vec![
                    Point::new(0, 0),
                    Point::new(9, 0),
                    Point::new(0, 10),
                    Point::new(9, 10),
                    Point::new(0, 20),
                ],
            );

            board.fill(&Point::new(9, 20));
            assert_only_filled(
                &board,
                vec![
                    Point::new(0, 0),
                    Point::new(9, 0),
                    Point::new(0, 10),
                    Point::new(9, 10),
                    Point::new(0, 20),
                    Point::new(9, 20),
                ],
            );
        }
    }

    mod empty {
        use super::*;

        #[test]
        fn empties_cells() {
            let mut board = Board::filled_board();

            board.empty(&Point::new(0, 0));
            assert_only_emptied(&board, vec![Point::new(0, 0)]);

            board.empty(&Point::new(9, 0));
            assert_only_emptied(&board, vec![Point::new(0, 0), Point::new(9, 0)]);

            board.empty(&Point::new(0, 10));
            assert_only_emptied(
                &board,
                vec![Point::new(0, 0), Point::new(9, 0), Point::new(0, 10)],
            );

            board.empty(&Point::new(9, 10));
            assert_only_emptied(
                &board,
                vec![
                    Point::new(0, 0),
                    Point::new(9, 0),
                    Point::new(0, 10),
                    Point::new(9, 10),
                ],
            );

            board.empty(&Point::new(0, 20));
            assert_only_emptied(
                &board,
                vec![
                    Point::new(0, 0),
                    Point::new(9, 0),
                    Point::new(0, 10),
                    Point::new(9, 10),
                    Point::new(0, 20),
                ],
            );

            board.empty(&Point::new(9, 20));
            assert_only_emptied(
                &board,
                vec![
                    Point::new(0, 0),
                    Point::new(9, 0),
                    Point::new(0, 10),
                    Point::new(9, 10),
                    Point::new(0, 20),
                    Point::new(9, 20),
                ],
            );
        }
    }

    mod has_intersect {
        use super::*;

        #[test]
        fn interlaced_boards() {
            let a = Board {
                fill: [
                    0b0101010101_0101010101_0101010101_0101010101_0101010101_0101010101,
                    0b1010101010_1010101010_1010101010_1010101010_1010101010_1010101010,
                    0b0101010101_0101010101_0101010101_0101010101_0101010101_0101010101,
                    0b1010101010_1010101010_1010101010_1010101010_1010101010_1010101010,
                ],
            };
            let b = Board {
                fill: [
                    0b1010101010_1010101010_1010101010_1010101010_1010101010_1010101010,
                    0b0101010101_0101010101_0101010101_0101010101_0101010101_0101010101,
                    0b1010101010_1010101010_1010101010_1010101010_1010101010_1010101010,
                    0b0101010101_0101010101_0101010101_0101010101_0101010101_0101010101,
                ],
            };
            assert!(
                !a.has_intersect(&b),
                "Expected interlaced boards to have no overlap"
            );
        }

        #[test]
        fn overlap_on_bottom_left_cell() {
            let a = Board {
                fill: [
                    0b0101010101_0101010101_0101010101_0101010101_0101010101_0101010101,
                    0b1010101010_1010101010_1010101010_1010101010_1010101010_1010101010,
                    0b0101010101_0101010101_0101010101_0101010101_0101010101_0101010101,
                    0b1010101010_1010101010_1010101010_1010101010_1010101010_1010101010,
                ],
            };
            let b = Board {
                fill: [
                    0b1010101010_1010101010_1010101010_1010101010_1010101010_1010101011,
                    0b0101010101_0101010101_0101010101_0101010101_0101010101_0101010101,
                    0b1010101010_1010101010_1010101010_1010101010_1010101010_1010101010,
                    0b0101010101_0101010101_0101010101_0101010101_0101010101_0101010101,
                ],
            };
            assert!(a.has_intersect(&b), "Expected boards to overlap");
        }
    }

    mod union {
        use super::*;

        #[test]
        fn unions_another_board() {
            let mut a = Board {
                fill: [
                    0b0000000000_0000000000_0000000000_0000000000_0000000000_0000000000,
                    0b0101010101_0101010101_0101010101_0101010101_0101010101_0101010101,
                    0b1010101010_1010101010_1010101010_1010101010_1010101010_1010101010,
                    0b1111111111_1111111111_1111111111_1111111111_1111111111_1111111111,
                ],
            };

            let b = Board {
                fill: [
                    0b0101010101_0101010101_0101010101_0101010101_0101010101_0101010101,
                    0b1010101010_1010101010_1010101010_1010101010_1010101010_1010101010,
                    0b0000000000_0000000000_0000000000_0000000000_0000000000_0000000000,
                    0b1111111111_1111111111_1111111111_1111111111_1111111111_1111111111,
                ],
            };

            a.union(&b);

            let expected = Board {
                fill: [
                    0b0101010101_0101010101_0101010101_0101010101_0101010101_0101010101,
                    0b1111111111_1111111111_1111111111_1111111111_1111111111_1111111111,
                    0b1010101010_1010101010_1010101010_1010101010_1010101010_1010101010,
                    0b1111111111_1111111111_1111111111_1111111111_1111111111_1111111111,
                ],
            };

            assert_eq!(expected, a);
        }
    }

    mod can_fit {
        use crate::piece::{Piece, PieceKind};
        use crate::rotation::Orientation;

        use super::*;

        #[test]
        fn fits_in_a_minimal_gap() {
            let board = Board {
                fill: [
                    0b1111111111_1111111111_1111111111_1111111111_1111111111_1111111111,
                    0b1111111111_1111111111_1111111111_1111111111_1111111111_1111111111,
                    0b1111111111_1111111111_1111111111_1111111111_1111111111_1111111111,
                    0b1110000111_1111111111_1111111111_1111111111_1111111111_1111111111,
                ],
            };

            let piece = Piece {
                kind: PieceKind::I,
                orientation: Orientation::North,
                position: Point::new(3, 21),
            };

            assert!(
                board.can_fit(&piece.get_points(&CONFIG)),
                "Expected I piece to fit in board {:?}",
                board
            )
        }

        #[test]
        fn cannot_fit_when_cell_overlaps() {
            let board = Board {
                fill: [
                    0b0000000000_0000000000_0000000000_0000000000_0000000000_0000000000,
                    0b0000000000_0000000000_0000000000_0000000000_0000000000_0000000000,
                    0b0000000000_0000000000_0000000000_0000000000_0000000000_0000000000,
                    0b0001000000_0000000000_0000000000_0000000000_0000000000_0000000000,
                ],
            };

            let piece = Piece {
                kind: PieceKind::I,
                orientation: Orientation::North,
                position: Point::new(3, 21),
            };

            assert!(
                !board.can_fit(&piece.get_points(&CONFIG)),
                "Expected I piece to not fit in board {:?}",
                board
            )
        }

        #[test]
        fn cannot_fit_when_wall_collides() {
            let board = Board::empty_board();

            let piece = Piece {
                kind: PieceKind::I,
                orientation: Orientation::North,
                position: Point::new(-1, 0),
            };

            assert!(
                !board.can_fit(&piece.get_points(&CONFIG)),
                "Expected I piece to collide against the board wall",
            )
        }
    }

    mod can_place {
        use crate::piece::{Piece, PieceKind};

        use super::*;

        #[test]
        fn can_place_i_piece_on_floor() {
            let board = Board::empty_board();

            let piece = Piece {
                position: Point::new(3, -2),
                ..Piece::spawn(&PieceKind::I, &CONFIG)
            };

            assert!(board.can_place(&piece.get_points(&CONFIG)));
        }

        #[test]
        fn cannot_place_i_piece_in_air() {
            let board = Board::empty_board();

            let piece = Piece {
                position: Point::new(3, -1),
                ..Piece::spawn(&PieceKind::I, &CONFIG)
            };

            assert!(!board.can_place(&piece.get_points(&CONFIG)));
        }

        #[test]
        fn can_place_j_piece_on_filled_cell() {
            let mut board = Board::empty_board();
            board.fill(&Point::new(0, 0));
            board.fill(&Point::new(0, 1));

            let piece = Piece {
                position: Point::new(0, 1),
                ..Piece::spawn(&PieceKind::J, &CONFIG)
            };

            assert!(board.can_place(&piece.get_points(&CONFIG)));
        }
    }

    mod fill_piece_points {
        use crate::piece::{Piece, PieceKind};
        use crate::rotation::Orientation;

        use super::*;

        #[test]
        fn fills_piece() {
            let mut board = Board::empty_board();
            let piece = Piece {
                kind: PieceKind::I,
                orientation: Orientation::North,
                position: Point::new(3, 21),
            };
            board.fill_piece_points(&piece.get_points(&CONFIG));

            let expected_board = Board {
                fill: [
                    0b0000000000_0000000000_0000000000_0000000000_0000000000_0000000000,
                    0b0000000000_0000000000_0000000000_0000000000_0000000000_0000000000,
                    0b0000000000_0000000000_0000000000_0000000000_0000000000_0000000000,
                    0b0001111000_0000000000_0000000000_0000000000_0000000000_0000000000,
                ],
            };

            assert_eq!(board, expected_board,)
        }
    }

    mod is_line_filled {
        use super::*;

        #[test]
        fn line_filled() {
            let board = Board::filled_board();
            for y in 0..24 {
                assert!(board.is_line_filled(y));
            }
        }

        #[test]
        fn line_not_filled() {
            let board = Board::empty_board();
            for y in 0..24 {
                assert!(!board.is_line_filled(y));
            }
        }

        #[test]
        fn line_not_filled_if_any_empty_cell() {
            let mut board = Board::filled_board();
            for y in 0..24 {
                board.empty(&Point::new(5, y));
            }
            for y in 0..24 {
                assert!(!board.is_line_filled(y));
            }
        }
    }

    mod clear_filled_lines {
        use super::*;

        #[test]
        fn no_difference_if_no_filled_lines() {
            let mut board = Board::filled_board();
            for y in 0..24 {
                board.empty(&Point::new(y % 10, y));
            }

            let mut next_board = board.clone();
            next_board.clear_filled_lines();

            assert_eq!(next_board, board);
        }

        #[test]
        fn moves_lines_down_when_clearing() {
            let board = {
                let mut b = Board::empty_board();

                // filled lines to clear
                for x in 0..10 {
                    b.fill(&Point::new(x, 0));
                    b.fill(&Point::new(x, 1));
                    b.fill(&Point::new(x, 4));
                    b.fill(&Point::new(x, 5));
                }
                // diagonal pattern that should be moved down
                b.fill(&Point::new(2, 2));
                b.fill(&Point::new(3, 3));
                b.fill(&Point::new(6, 6));
                b.fill(&Point::new(7, 7));
                b.fill(&Point::new(8, 8));
                b.fill(&Point::new(9, 9));

                b
            };

            let mut next_board = board.clone();
            next_board.clear_filled_lines();

            let expected_board = {
                let mut b = Board::empty_board();

                b.fill(&Point::new(2, 0));
                b.fill(&Point::new(3, 1));
                b.fill(&Point::new(6, 2));
                b.fill(&Point::new(7, 3));
                b.fill(&Point::new(8, 4));
                b.fill(&Point::new(9, 5));

                b
            };

            assert_eq!(next_board, expected_board);
        }
    }
}
