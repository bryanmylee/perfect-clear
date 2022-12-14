use crate::board::Board;
use crate::config::Config;
use crate::piece::{Piece, PieceKind};
use crate::utils::direction::Direction;
use crate::utils::rotation::Rotation;
use std::convert::TryInto;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq)]
pub struct Game {
    pub board: Board,

    pub piece: Option<Piece>,

    pub hold_kind: Option<PieceKind>,

    pub is_hold_used: bool,

    /// Fixed queue size to reduce heap allocations.
    #[wasm_bindgen(skip)]
    pub queue: [Option<PieceKind>; 7],
}

impl Game {
    pub fn initial() -> Game {
        Game {
            board: Board::empty_board(),
            piece: None,
            hold_kind: None,
            is_hold_used: false,
            queue: [None; 7],
        }
    }

    pub fn reduce(&self, config: &Config, action: &Action) -> Result<Game, ReduceError> {
        match action {
            Action::Move(mov) => self
                .with_moved_piece(config, mov)
                .map_err(|e| ReduceError::Move(e)),
            Action::Hold { switch } => self
                .with_hold_used(config, *switch)
                .map_err(|e| ReduceError::Hold(e)),
            Action::Place => self
                .with_placed_piece(config)
                .map_err(|e| ReduceError::Place(e)),
        }
    }

    fn with_moved_piece(&self, config: &Config, mov: &Move) -> Result<Game, MoveError> {
        match mov {
            Move::Rotate(rotation) => self.with_rotated_piece(config, &rotation),
            Move::Translate(direction) => self.with_translated_piece(config, &direction),
            Move::Drop => self.with_dropped_piece(config),
        }
    }

    fn with_rotated_piece(&self, config: &Config, rotation: &Rotation) -> Result<Game, MoveError> {
        let Some(piece) = self.piece.as_ref() else {
            return Err(MoveError::NoPiece);
        };

        let from_orientation = piece.orientation;
        let to_orientation = from_orientation.rotated(rotation);

        let mut rotated_piece = Piece {
            orientation: to_orientation,
            ..piece.clone()
        };
        let piece_points = rotated_piece.get_points(config);

        if self.board.can_fit(&piece_points) {
            return Ok(Game {
                piece: Some(rotated_piece),
                ..self.clone()
            });
        }

        let Some(kicks) = config.kick_table(&piece.kind, &from_orientation, &to_orientation) else {
            return Err(MoveError::InvalidMove);
        };

        for kick in kicks {
            let kicked_points = piece_points.map(|point| point + kick);
            if self.board.can_fit(&kicked_points) {
                rotated_piece.position += kick;
                return Ok(Game {
                    piece: Some(rotated_piece),
                    ..self.clone()
                });
            }
        }

        Err(MoveError::InvalidMove)
    }

    fn with_translated_piece(
        &self,
        config: &Config,
        direction: &Direction,
    ) -> Result<Game, MoveError> {
        let Some(piece) = self.piece.as_ref() else {
            return Err(MoveError::NoPiece);
        };

        let direction_offset = direction.get_offset();

        let next_piece = Piece {
            position: piece.position + direction_offset,
            ..piece.clone()
        };

        let next_piece_points = next_piece.get_points(config);

        if !self.board.can_fit(&next_piece_points) {
            return Err(MoveError::InvalidMove);
        }

        Ok(Game {
            piece: Some(next_piece),
            ..self.clone()
        })
    }

    fn with_dropped_piece(&self, config: &Config) -> Result<Game, MoveError> {
        let Some(piece) = self.piece.as_ref() else {
            return Err(MoveError::NoPiece);
        };

        let mut dropped_piece = piece.clone();

        while self.board.can_fit(&dropped_piece.get_points(config)) {
            dropped_piece.position.y -= 1;
        }
        dropped_piece.position.y += 1;

        if dropped_piece.position.y == piece.position.y {
            return Err(MoveError::InvalidMove);
        }

        Ok(Game {
            piece: Some(dropped_piece),
            ..self.clone()
        })
    }

    fn with_hold_used(&self, config: &Config, switch: bool) -> Result<Game, HoldError> {
        if self.is_hold_used {
            return Err(HoldError::NotAvailable);
        }

        if !switch {
            return Ok(Game {
                is_hold_used: true,
                ..self.clone()
            });
        }

        let Some(hold_kind) = self.hold_kind.as_ref() else {
            return Err(HoldError::NoHoldPiece);
        };

        let next_piece = Piece::spawn(config, &hold_kind);

        if !self.board.can_fit(&next_piece.get_points(config)) {
            return Err(HoldError::PieceCollision);
        }

        let Some(piece) = self.piece.as_ref() else {
            return Err(HoldError::NoPiece);
        };

        Ok(Game {
            is_hold_used: true,
            piece: Some(next_piece),
            hold_kind: Some(piece.kind),
            ..self.clone()
        })
    }

    fn with_placed_piece(&self, config: &Config) -> Result<Game, PlaceError> {
        let Some(piece) = &self.piece else {
            return Err(PlaceError::NoPiece);
        };

        let piece_points = piece.get_points(config);

        if !self.board.can_place(&piece_points) {
            return Err(PlaceError::PieceInAir);
        }

        let next_game = self.clone();
        let mut next_board = next_game.board;

        next_board.fill_piece_points(&piece_points);
        if !next_board.can_perfect_clear() {
            next_board.clear_filled_lines();
        }

        Ok(Game {
            board: next_board,
            piece: None,
            is_hold_used: false,
            ..next_game
        })
    }
}

#[wasm_bindgen]
impl Game {
    pub fn js_new(
        board: Board,
        piece: Option<Piece>,
        hold_kind: Option<PieceKind>,
        is_hold_used: bool,
        js_queue: js_sys::Uint8Array,
    ) -> Game {
        Game {
            board,
            piece,
            hold_kind,
            is_hold_used,
            queue: {
                let mut queue = [u8::MAX; 7];
                js_queue.copy_to(&mut queue[..js_queue.length() as usize]);
                queue.map(|kind| kind.try_into().ok())
            },
        }
    }

    /// Represent the queue as a JavaScript `Uint8Array`.
    pub fn js_queue(&self) -> js_sys::Uint8Array {
        js_sys::Uint8Array::from(
            &self
                .queue
                .iter()
                .filter_map(|kind| kind.map(|kind| kind as u8))
                .collect::<Vec<u8>>()[..],
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Action {
    Move(Move),
    Hold { switch: bool },
    Place,
}

#[derive(Debug, PartialEq)]
pub enum ReduceError {
    Move(MoveError),
    Hold(HoldError),
    Place(PlaceError),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Move {
    Rotate(Rotation),
    Translate(Direction),
    Drop,
}

#[derive(Debug, PartialEq)]
pub enum MoveError {
    NoPiece,
    InvalidMove,
}

#[derive(Debug, PartialEq)]
pub enum HoldError {
    NotAvailable,
    NoHoldPiece,
    NoPiece,
    PieceCollision,
}

#[derive(Debug, PartialEq)]
pub enum PlaceError {
    NoPiece,
    PieceInAir,
}

#[cfg(test)]
mod tests {
    use crate::utils::point::Point;

    use super::*;

    const CONFIG: Config = Config::default();

    mod with_rotation {
        use crate::utils::rotation::Orientation;

        use super::*;

        mod i_piece {
            use super::*;

            #[test]
            fn no_kick() {
                let game = Game {
                    piece: Some(Piece::spawn(&CONFIG, &PieceKind::I)),
                    ..Game::initial()
                };

                let original_position = game.piece.as_ref().unwrap().position;

                let next_game = game.with_moved_piece(&CONFIG, &Move::Rotate(Rotation::Clockwise));

                assert!(next_game.is_ok());
                let next_game = next_game.unwrap();

                assert!(next_game.piece.is_some());
                assert_eq!(
                    next_game.piece.as_ref().unwrap().orientation,
                    Orientation::East
                );
                assert_eq!(
                    next_game.piece.as_ref().unwrap().position,
                    original_position,
                );
            }

            mod north_and_east {

                use super::*;

                #[test]
                fn kick_one() {
                    let mut board = Board::filled_board();

                    board.empty(&Point::new(3, 2));
                    board.empty(&Point::new(4, 2));
                    board.empty(&Point::new(5, 2));
                    board.empty(&Point::new(6, 2));

                    board.empty(&Point::new(3, 0));
                    board.empty(&Point::new(3, 1));
                    board.empty(&Point::new(3, 2));
                    board.empty(&Point::new(3, 3));

                    let game = Game {
                        board,
                        piece: Some(Piece {
                            position: Point::new(3, 0),
                            ..Piece::spawn(&CONFIG, &PieceKind::I)
                        }),
                        ..Game::initial()
                    };

                    let next_game =
                        game.with_moved_piece(&CONFIG, &Move::Rotate(Rotation::Clockwise));

                    assert!(next_game.is_ok());
                    let next_game = next_game.unwrap();

                    assert!(next_game.piece.is_some());
                    assert_eq!(
                        next_game.piece.as_ref().unwrap().orientation,
                        Orientation::East
                    );
                    assert_eq!(next_game.piece.as_ref().unwrap().position, Point::new(1, 0),);

                    let next_game =
                        next_game.with_moved_piece(&CONFIG, &Move::Rotate(Rotation::AntiClockwise));

                    assert!(next_game.is_ok());
                    let next_state = next_game.unwrap();

                    assert!(next_state.piece.is_some());
                    assert_eq!(
                        next_state.piece.as_ref().unwrap().orientation,
                        Orientation::North
                    );
                    assert_eq!(
                        next_state.piece.as_ref().unwrap().position,
                        Point::new(3, 0)
                    );
                }

                #[test]
                fn kick_two() {
                    let mut board = Board::filled_board();

                    board.empty(&Point::new(3, 2));
                    board.empty(&Point::new(4, 2));
                    board.empty(&Point::new(5, 2));
                    board.empty(&Point::new(6, 2));

                    board.empty(&Point::new(6, 0));
                    board.empty(&Point::new(6, 1));
                    board.empty(&Point::new(6, 2));
                    board.empty(&Point::new(6, 3));

                    let game = Game {
                        board,
                        piece: Some(Piece {
                            position: Point::new(3, 0),
                            ..Piece::spawn(&CONFIG, &PieceKind::I)
                        }),
                        ..Game::initial()
                    };

                    let next_game =
                        game.with_moved_piece(&CONFIG, &Move::Rotate(Rotation::Clockwise));

                    assert!(next_game.is_ok());
                    let next_game = next_game.unwrap();

                    assert!(next_game.piece.is_some());
                    assert_eq!(
                        next_game.piece.as_ref().unwrap().orientation,
                        Orientation::East
                    );
                    assert_eq!(next_game.piece.as_ref().unwrap().position, Point::new(4, 0),);

                    let next_game =
                        next_game.with_moved_piece(&CONFIG, &Move::Rotate(Rotation::AntiClockwise));

                    assert!(next_game.is_ok());
                    let next_game = next_game.unwrap();

                    assert!(next_game.piece.is_some());
                    assert_eq!(
                        next_game.piece.as_ref().unwrap().orientation,
                        Orientation::North
                    );
                    assert_eq!(next_game.piece.as_ref().unwrap().position, Point::new(3, 0));
                }

                #[test]
                fn kick_three() {
                    let mut board = Board::filled_board();

                    board.empty(&Point::new(3, 3));
                    board.empty(&Point::new(4, 3));
                    board.empty(&Point::new(5, 3));
                    board.empty(&Point::new(6, 3));

                    board.empty(&Point::new(3, 0));
                    board.empty(&Point::new(3, 1));
                    board.empty(&Point::new(3, 2));
                    board.empty(&Point::new(3, 3));

                    let game = Game {
                        board,
                        piece: Some(Piece {
                            position: Point::new(3, 1),
                            ..Piece::spawn(&CONFIG, &PieceKind::I)
                        }),
                        ..Game::initial()
                    };

                    let next_game =
                        game.with_moved_piece(&CONFIG, &Move::Rotate(Rotation::Clockwise));

                    assert!(next_game.is_ok());
                    let next_game = next_game.unwrap();

                    assert!(next_game.piece.is_some());
                    assert_eq!(
                        next_game.piece.as_ref().unwrap().orientation,
                        Orientation::East
                    );
                    assert_eq!(next_game.piece.as_ref().unwrap().position, Point::new(1, 0),);

                    let next_game =
                        next_game.with_moved_piece(&CONFIG, &Move::Rotate(Rotation::AntiClockwise));

                    assert!(next_game.is_ok());
                    let next_game = next_game.unwrap();

                    assert!(next_game.piece.is_some());
                    assert_eq!(
                        next_game.piece.as_ref().unwrap().orientation,
                        Orientation::North
                    );
                    assert_eq!(next_game.piece.as_ref().unwrap().position, Point::new(3, 1));
                }

                #[test]
                fn kick_four() {
                    let mut board = Board::filled_board();

                    board.empty(&Point::new(3, 2));
                    board.empty(&Point::new(4, 2));
                    board.empty(&Point::new(5, 2));
                    board.empty(&Point::new(6, 2));

                    board.empty(&Point::new(6, 2));
                    board.empty(&Point::new(6, 3));
                    board.empty(&Point::new(6, 4));
                    board.empty(&Point::new(6, 5));

                    let game = Game {
                        board,
                        piece: Some(Piece {
                            position: Point::new(3, 0),
                            ..Piece::spawn(&CONFIG, &PieceKind::I)
                        }),
                        ..Game::initial()
                    };

                    let next_game =
                        game.with_moved_piece(&CONFIG, &Move::Rotate(Rotation::Clockwise));

                    assert!(next_game.is_ok());
                    let next_game = next_game.unwrap();

                    assert!(next_game.piece.is_some());
                    assert_eq!(
                        next_game.piece.as_ref().unwrap().orientation,
                        Orientation::East
                    );
                    assert_eq!(next_game.piece.as_ref().unwrap().position, Point::new(4, 2),);

                    let next_game =
                        next_game.with_moved_piece(&CONFIG, &Move::Rotate(Rotation::AntiClockwise));

                    assert!(next_game.is_ok());
                    let next_game = next_game.unwrap();

                    assert!(next_game.piece.is_some());
                    assert_eq!(
                        next_game.piece.as_ref().unwrap().orientation,
                        Orientation::North
                    );
                    assert_eq!(next_game.piece.as_ref().unwrap().position, Point::new(3, 0));
                }
            }
        }
    }

    mod with_translation {
        use super::*;

        #[test]
        fn moves_piece() {
            let game = Game {
                piece: Some(Piece {
                    position: Point::new(3, -1),
                    ..Piece::spawn(&CONFIG, &PieceKind::I)
                }),
                ..Game::initial()
            };

            let next_game = game.with_moved_piece(&CONFIG, &Move::Translate(Direction::Down));

            assert!(next_game.is_ok());
            let next_game = next_game.unwrap();

            let piece = next_game.piece.as_ref().unwrap();
            assert_eq!(piece.position, Point::new(3, -2));

            let next_game = next_game.with_moved_piece(&CONFIG, &Move::Translate(Direction::Left));

            assert!(next_game.is_ok());
            let next_game = next_game.unwrap();

            let piece = next_game.piece.as_ref().unwrap();
            assert_eq!(piece.position, Point::new(2, -2));

            let next_game = next_game.with_moved_piece(&CONFIG, &Move::Translate(Direction::Right));

            assert!(next_game.is_ok());
            let next_game = next_game.unwrap();

            let piece = next_game.piece.as_ref().unwrap();
            assert_eq!(piece.position, Point::new(3, -2));

            let next_game = next_game.with_moved_piece(&CONFIG, &Move::Translate(Direction::Down));
            assert_eq!(next_game, Err(MoveError::InvalidMove));
        }
    }

    mod with_drop {
        use super::*;

        #[test]
        fn i_piece_on_floor_in_empty_board() {
            let game = Game {
                piece: Some(Piece::spawn(&CONFIG, &PieceKind::I)),
                ..Game::initial()
            };

            let next_game = game.reduce(&CONFIG, &Action::Move(Move::Drop));

            assert!(next_game.is_ok());
            let next_game = next_game.unwrap();

            assert_eq!(next_game.piece.unwrap().position.y, -2);
        }

        #[test]
        fn o_piece_on_floor_in_empty_board() {
            let game = Game {
                piece: Some(Piece::spawn(&CONFIG, &PieceKind::O)),
                ..Game::initial()
            };

            let next_game = game.reduce(&CONFIG, &Action::Move(Move::Drop));

            assert!(next_game.is_ok());
            let next_game = next_game.unwrap();

            assert_eq!(next_game.piece.unwrap().position.y, -1);
        }

        #[test]
        fn i_piece_on_filled_cell() {
            let game = Game {
                board: {
                    let mut b = Board::empty_board();
                    b.fill(&Point::new(3, 10));
                    b
                },
                piece: Some(Piece::spawn(&CONFIG, &PieceKind::I)),
                ..Game::initial()
            };

            let next_game = game.reduce(&CONFIG, &Action::Move(Move::Drop));

            assert!(next_game.is_ok());
            let next_game = next_game.unwrap();

            assert_eq!(next_game.piece.unwrap().position.y, 9);
        }
    }

    mod with_hold_used {
        use super::*;

        #[test]
        fn invalid_if_no_active_piece() {
            let game = Game {
                hold_kind: Some(PieceKind::J),
                ..Game::initial()
            };

            let next_game = game.reduce(&CONFIG, &Action::Hold { switch: true });

            assert_eq!(next_game, Err(ReduceError::Hold(HoldError::NoPiece)));
        }

        #[test]
        fn invalid_if_no_hold_piece() {
            let game = Game {
                piece: Some(Piece::spawn(&CONFIG, &PieceKind::I)),
                ..Game::initial()
            };

            let next_game = game.reduce(&CONFIG, &Action::Hold { switch: true });

            assert_eq!(next_game, Err(ReduceError::Hold(HoldError::NoHoldPiece)));
        }

        #[test]
        fn invalid_if_new_piece_intersects_board() {
            let mut board = Board::empty_board();
            for x in 3..7 {
                board.fill(&Point::new(x, 20));
            }

            let game = Game {
                board,
                hold_kind: Some(PieceKind::I),
                piece: Some(Piece::spawn(&CONFIG, &PieceKind::J)),
                ..Game::initial()
            };

            let next_game = game.reduce(&CONFIG, &Action::Hold { switch: true });

            assert_eq!(
                next_game,
                Err(ReduceError::Hold(HoldError::PieceCollision)),
                "Expected state to be invalid if next active piece intersects the board",
            )
        }

        #[test]
        fn consumes_hold_and_swaps_hold() {
            let game = Game {
                hold_kind: Some(PieceKind::J),
                piece: Some(Piece::spawn(&CONFIG, &PieceKind::I)),
                ..Game::initial()
            };

            let next_game = game.reduce(&CONFIG, &Action::Hold { switch: true });

            assert!(next_game.is_ok());
            let next_game = next_game.unwrap();

            assert!(next_game.is_hold_used);
            assert_eq!(next_game.hold_kind.unwrap(), PieceKind::I);
            assert_eq!(next_game.piece.as_ref().unwrap().kind, PieceKind::J);
        }

        #[test]
        fn consumes_hold_without_swapping_hold() {
            let game = Game {
                hold_kind: Some(PieceKind::J),
                piece: Some(Piece::spawn(&CONFIG, &PieceKind::I)),
                ..Game::initial()
            };

            let next_game = game.reduce(&CONFIG, &Action::Hold { switch: false });

            assert!(next_game.is_ok());
            let next_game = next_game.unwrap();

            assert!(next_game.is_hold_used);
            assert_eq!(next_game.hold_kind.unwrap(), PieceKind::J);
            assert_eq!(next_game.piece.as_ref().unwrap().kind, PieceKind::I);
        }
    }

    mod with_placed_piece {
        use super::*;

        #[test]
        fn invalid_if_no_active_piece() {
            let game = Game::initial();

            let next_game = game.reduce(&CONFIG, &Action::Place);

            assert_eq!(
                next_game,
                Err(ReduceError::Place(PlaceError::NoPiece)),
                "Expected state to be invalid if placing without active piece"
            );
        }

        #[test]
        fn invalid_if_piece_in_air() {
            let game = Game {
                piece: Some(Piece {
                    position: Point::new(3, -1),
                    ..Piece::spawn(&CONFIG, &PieceKind::I)
                }),
                ..Game::initial()
            };

            let next_game = game.reduce(&CONFIG, &Action::Place);

            assert_eq!(
                next_game,
                Err(ReduceError::Place(PlaceError::PieceInAir)),
                "Expected state to be invalid if placing without filled cell below piece"
            );
        }

        #[test]
        fn piece_placed() {
            let game = Game {
                piece: Some(Piece {
                    position: Point::new(3, -2),
                    ..Piece::spawn(&CONFIG, &PieceKind::I)
                }),
                ..Game::initial()
            };

            let next_game = game.reduce(&CONFIG, &Action::Place);

            assert!(next_game.is_ok());
            let next_game = next_game.unwrap();
            assert!(
                next_game.piece.is_none(),
                "Active piece should be none after placement"
            );

            let mut expected_board = Board::empty_board();
            expected_board.fill(&Point::new(3, 0));
            expected_board.fill(&Point::new(4, 0));
            expected_board.fill(&Point::new(5, 0));
            expected_board.fill(&Point::new(6, 0));
            assert_eq!(
                next_game.board, expected_board,
                "Previous active piece should fill the board after placement"
            );
        }

        #[test]
        fn resets_is_hold_used() {
            let game = Game {
                piece: Some(Piece {
                    position: Point::new(3, -2),
                    ..Piece::spawn(&CONFIG, &PieceKind::I)
                }),
                ..Game::initial()
            };

            let next_game = game.reduce(&CONFIG, &Action::Place);

            assert!(next_game.is_ok());
            let next_game = next_game.unwrap();
            assert!(
                !next_game.is_hold_used,
                "Hold should be reset after placing piece"
            );
        }
    }
}
