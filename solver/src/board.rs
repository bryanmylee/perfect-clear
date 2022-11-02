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
    /**
    `{ x: 0, y: 0 }` starts on the bottom-left.

    For convenience, we treat `x: -1` and `x: 10` as filled for the kick-table.
    */
    fn is_filled(&self, at: Point<isize>) -> bool {
        if at.x < 0 || at.x > 9 {
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

    fn has_overlap(&self, against: &BoardFill) -> bool {
        true
    }

    fn with_overlap(&self, against: &BoardFill) -> Board {
        self.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

            for x in 0..10 {
                for y in 0..24 {
                    let is_filled = match (x, y) {
                        (0, 0) => true,
                        (1, 0) => true,
                        (3, 0) => true,
                        (4, 0) => true,
                        (5, 0) => true,
                        (6, 0) => true,
                        (8, 0) => true,
                        (9, 0) => true,
                        (0, 1) => true,
                        (8, 1) => true,
                        (9, 1) => true,
                        (0, 2) => true,
                        (0, 6) => true,
                        (1, 6) => true,
                        (3, 6) => true,
                        (4, 6) => true,
                        (5, 6) => true,
                        (6, 6) => true,
                        (8, 6) => true,
                        (9, 6) => true,
                        (0, 7) => true,
                        (8, 7) => true,
                        (9, 7) => true,
                        (0, 8) => true,
                        (_, _) => false,
                    };
                    assert_eq!(
                        board.is_filled(Point { x, y }),
                        is_filled,
                        "Expected board to be {} at ({}, {})",
                        if is_filled { "filled" } else { "empty" },
                        x,
                        y
                    );
                }
            }
        }

        #[test]
        fn walls_are_filled() {
            let board = Board {
                fill: [
                    0b0000000000_0000000000_0000000000_0000000000_0000000000_0000000000,
                    0b0000000000_0000000000_0000000000_0000000000_0000000000_0000000000,
                    0b0000000000_0000000000_0000000000_0000000000_0000000000_0000000000,
                    0b0000000000_0000000000_0000000000_0000000000_0000000000_0000000000,
                ],
            };

            for y in 0..24 {
                assert!(
                    board.is_filled(Point { x: -1, y }),
                    "Expected left wall to be filled on line {}",
                    y
                );
                assert!(
                    board.is_filled(Point { x: 10, y }),
                    "Expected right wall to be filled on line {}",
                    y
                );
            }
        }
    }
}
