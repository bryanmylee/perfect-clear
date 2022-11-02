use crate::point::Point;

pub type BoardFill = [u64; 4];

#[derive(Debug, Clone)]
pub struct Board {
    /**
    A tetris board has 24 rows of 10 columns. We split the board into 4 segments of 6 rows to get
    60 cells in each segment. This lets us store the fill state of each segment as a bitfield.

    The segments are ordered from bottom to top and the cells in each segment are ordered from bottom-left to top-right.
    */
    fill: BoardFill,
}

impl Board {
    fn empty_board() -> Board {
        Board {
            fill: [
                0b0000000000_0000000000_0000000000_0000000000_0000000000_0000000000,
                0b0000000000_0000000000_0000000000_0000000000_0000000000_0000000000,
                0b0000000000_0000000000_0000000000_0000000000_0000000000_0000000000,
                0b0000000000_0000000000_0000000000_0000000000_0000000000_0000000000,
            ],
        }
    }

    fn filled_board() -> Board {
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
    fn is_filled(&self, at: &Point<isize>) -> bool {
        if at.x < 0 || at.x >= 10 {
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

    fn fill(&mut self, point: &Point<isize>) {
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

    fn empty(&mut self, point: &Point<isize>) {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_only_filled(board: &Board, fills: Vec<Point<isize>>) {
        for x in 0..10 {
            for y in 0..24 {
                let is_filled = fills.contains(&Point { x, y });
                assert_eq!(
                    board.is_filled(&Point { x, y }),
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
                let is_empty = empties.contains(&Point { x, y });
                assert_eq!(
                    !board.is_filled(&Point { x, y }),
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
                    Point { x: 0, y: 0 },
                    Point { x: 1, y: 0 },
                    Point { x: 3, y: 0 },
                    Point { x: 4, y: 0 },
                    Point { x: 5, y: 0 },
                    Point { x: 6, y: 0 },
                    Point { x: 8, y: 0 },
                    Point { x: 9, y: 0 },
                    Point { x: 0, y: 1 },
                    Point { x: 8, y: 1 },
                    Point { x: 9, y: 1 },
                    Point { x: 0, y: 2 },
                    Point { x: 0, y: 6 },
                    Point { x: 1, y: 6 },
                    Point { x: 3, y: 6 },
                    Point { x: 4, y: 6 },
                    Point { x: 5, y: 6 },
                    Point { x: 6, y: 6 },
                    Point { x: 8, y: 6 },
                    Point { x: 9, y: 6 },
                    Point { x: 0, y: 7 },
                    Point { x: 8, y: 7 },
                    Point { x: 9, y: 7 },
                    Point { x: 0, y: 8 },
                ],
            );
        }

        #[test]
        fn walls_are_filled() {
            let board = Board::empty_board();

            for y in 0..24 {
                assert!(
                    board.is_filled(&Point { x: -1, y }),
                    "Expected left wall to be filled on line {}",
                    y
                );
                assert!(
                    board.is_filled(&Point { x: 10, y }),
                    "Expected right wall to be filled on line {}",
                    y
                );
            }
        }
    }

    mod fill {
        use super::*;

        #[test]
        fn fills_cells() {
            let mut board = Board::empty_board();

            board.fill(&Point { x: 0, y: 0 });
            assert_only_filled(&board, vec![Point { x: 0, y: 0 }]);

            board.fill(&Point { x: 9, y: 0 });
            assert_only_filled(&board, vec![Point { x: 0, y: 0 }, Point { x: 9, y: 0 }]);

            board.fill(&Point { x: 0, y: 10 });
            assert_only_filled(
                &board,
                vec![
                    Point { x: 0, y: 0 },
                    Point { x: 9, y: 0 },
                    Point { x: 0, y: 10 },
                ],
            );

            board.fill(&Point { x: 9, y: 10 });
            assert_only_filled(
                &board,
                vec![
                    Point { x: 0, y: 0 },
                    Point { x: 9, y: 0 },
                    Point { x: 0, y: 10 },
                    Point { x: 9, y: 10 },
                ],
            );

            board.fill(&Point { x: 0, y: 20 });
            assert_only_filled(
                &board,
                vec![
                    Point { x: 0, y: 0 },
                    Point { x: 9, y: 0 },
                    Point { x: 0, y: 10 },
                    Point { x: 9, y: 10 },
                    Point { x: 0, y: 20 },
                ],
            );

            board.fill(&Point { x: 9, y: 20 });
            assert_only_filled(
                &board,
                vec![
                    Point { x: 0, y: 0 },
                    Point { x: 9, y: 0 },
                    Point { x: 0, y: 10 },
                    Point { x: 9, y: 10 },
                    Point { x: 0, y: 20 },
                    Point { x: 9, y: 20 },
                ],
            );
        }
    }

    mod empty {
        use super::*;

        #[test]
        fn empties_cells() {
            let mut board = Board::filled_board();

            board.empty(&Point { x: 0, y: 0 });
            assert_only_emptied(&board, vec![Point { x: 0, y: 0 }]);

            board.empty(&Point { x: 9, y: 0 });
            assert_only_emptied(&board, vec![Point { x: 0, y: 0 }, Point { x: 9, y: 0 }]);

            board.empty(&Point { x: 0, y: 10 });
            assert_only_emptied(
                &board,
                vec![
                    Point { x: 0, y: 0 },
                    Point { x: 9, y: 0 },
                    Point { x: 0, y: 10 },
                ],
            );

            board.empty(&Point { x: 9, y: 10 });
            assert_only_emptied(
                &board,
                vec![
                    Point { x: 0, y: 0 },
                    Point { x: 9, y: 0 },
                    Point { x: 0, y: 10 },
                    Point { x: 9, y: 10 },
                ],
            );

            board.empty(&Point { x: 0, y: 20 });
            assert_only_emptied(
                &board,
                vec![
                    Point { x: 0, y: 0 },
                    Point { x: 9, y: 0 },
                    Point { x: 0, y: 10 },
                    Point { x: 9, y: 10 },
                    Point { x: 0, y: 20 },
                ],
            );

            board.empty(&Point { x: 9, y: 20 });
            assert_only_emptied(
                &board,
                vec![
                    Point { x: 0, y: 0 },
                    Point { x: 9, y: 0 },
                    Point { x: 0, y: 10 },
                    Point { x: 9, y: 10 },
                    Point { x: 0, y: 20 },
                    Point { x: 9, y: 20 },
                ],
            );
        }
    }
}
