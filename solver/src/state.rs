use crate::board::Board;
use crate::config::{srs, Config};
use crate::piece::{Piece, PieceKind};
use crate::point::Point;
use crate::rotation::Rotation;

#[derive(Debug, Clone, PartialEq)]
pub struct State {
    board: Board,
    piece: Option<Piece>,
    hold_kind: Option<PieceKind>,
    is_hold_used: bool,
    queue: [Option<PieceKind>; 7], // fixed queue size to reduce heap allocations
    seen: [Option<PieceKind>; 14], // only 2-bags needed at most to determine next piece probability
    moves_remaining: isize,
    current_prob: f32,
}

impl State {
    pub fn initial() -> State {
        State {
            board: Board::empty_board(),
            piece: None,
            hold_kind: None,
            is_hold_used: false,
            queue: [None; 7],
            seen: [None; 14],
            moves_remaining: 10,
            current_prob: 1.0,
        }
    }
}

impl State {
    pub fn reduce(&self, action: &Action, config: &Config) -> Result<State, ReduceError> {
        match action {
            Action::ConsumeQueue => self.with_consumed_queue(config),
            Action::GuessNext { kind, prob } => self.with_guessed_next(config, kind, *prob),
            Action::Hold { switch } => self.with_hold_used(config, *switch),
            Action::Move(mov) => self.with_move(config, *mov),
            Action::Place => self.with_placed_piece(config),
        }
    }

    fn with_consumed_queue(&self, config: &Config) -> Result<State, ReduceError> {
        let Some((Some(next_piece_kind), rest_piece_kinds)) = self.queue.split_first() else {
            return Err(ReduceError::ConsumeQueue(ConsumeQueueError::QueueEmpty));
        };

        let next_piece = Piece::spawn(next_piece_kind, config);

        if !self.board.can_fit(&next_piece.get_points(config)) {
            return Err(ReduceError::GameOver);
        }

        let mut new_queue = [None; 7];
        new_queue[..rest_piece_kinds.len()].clone_from_slice(rest_piece_kinds);

        Ok(State {
            queue: new_queue,
            piece: Some(next_piece),
            is_hold_used: false,
            ..self.clone()
        })
    }

    fn with_guessed_next(
        &self,
        config: &Config,
        piece_kind: &PieceKind,
        with_prob: f32,
    ) -> Result<State, ReduceError> {
        let next_piece = Piece::spawn(piece_kind, config);

        if !self.board.can_fit(&next_piece.get_points(config)) {
            return Err(ReduceError::GameOver);
        }

        Ok(State {
            piece: Some(next_piece),
            current_prob: self.current_prob * with_prob,
            ..self.clone()
        })
    }

    fn with_hold_used(&self, config: &Config, switch: bool) -> Result<State, ReduceError> {
        if self.is_hold_used {
            return Err(ReduceError::Hold(HoldError::NotAvailable));
        }

        if !switch {
            return Ok(State {
                is_hold_used: true,
                ..self.clone()
            });
        }

        let Some(hold_kind) = self.hold_kind.as_ref() else {
            return Err(ReduceError::Hold(HoldError::NoHoldPiece));
        };

        let next_piece = Piece::spawn(&hold_kind, config);

        if !self.board.can_fit(&next_piece.get_points(config)) {
            return Err(ReduceError::GameOver);
        }

        let Some(piece) = self.piece.as_ref() else {
            return Err(ReduceError::Hold(HoldError::NoPiece))
        };

        Ok(State {
            is_hold_used: true,
            piece: Some(next_piece),
            hold_kind: Some(piece.kind),
            ..self.clone()
        })
    }

    fn with_move(&self, config: &Config, mov: Move) -> Result<State, ReduceError> {
        match mov {
            Move::Rotate(rotation) => self.with_rotation(config, &rotation),
            Move::Translate(direction) => self.with_translation(config, &direction),
        }
    }

    fn with_rotation(&self, config: &Config, rotation: &Rotation) -> Result<State, ReduceError> {
        let Some(piece) = self.piece.as_ref() else {
            return Err(ReduceError::Move(MoveError::NoPiece));
        };

        let from_orientation = piece.orientation;
        let to_orientation = from_orientation.rotated(rotation);

        let mut rotated_piece = Piece {
            orientation: to_orientation,
            ..piece.clone()
        };
        let piece_points = rotated_piece.get_points(config);

        if self.board.can_fit(&piece_points) {
            return Ok(State {
                piece: Some(rotated_piece),
                ..self.clone()
            });
        }

        let Some(kicks) = srs::kick_table(&piece.kind, &from_orientation, &to_orientation) else {
            return Err(ReduceError::Move(MoveError::InvalidMove));
        };

        for kick in kicks {
            let kicked_points = piece_points.map(|point| point + kick);
            if self.board.can_fit(&kicked_points) {
                rotated_piece.position += kick;
                return Ok(State {
                    piece: Some(rotated_piece),
                    ..self.clone()
                });
            }
        }

        Err(ReduceError::Move(MoveError::InvalidMove))
    }

    fn with_translation(
        &self,
        config: &Config,
        direction: &Direction,
    ) -> Result<State, ReduceError> {
        let Some(piece) = self.piece.as_ref() else {
            return Err(ReduceError::Move(MoveError::NoPiece));
        };

        let direction_offset = direction.get_offset();

        let next_piece = Piece {
            position: piece.position + direction_offset,
            ..piece.clone()
        };

        let next_piece_points = next_piece.get_points(config);

        if !self.board.can_fit(&next_piece_points) {
            return Err(ReduceError::Move(MoveError::InvalidMove));
        }

        Ok(State {
            piece: Some(next_piece),
            ..self.clone()
        })
    }

    fn with_placed_piece(&self, config: &Config) -> Result<State, ReduceError> {
        let mut next_state = self.clone();

        let Some(piece) = &self.piece else {
            return Err(ReduceError::Place(PlaceError::NoPiece));
        };

        let piece_points = piece.get_points(config);

        if !self.board.can_place(&piece_points) {
            return Err(ReduceError::Place(PlaceError::PieceInAir));
        }

        next_state.board.fill_piece_points(&piece_points);

        next_state.piece = None;

        Ok(next_state)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Action {
    ConsumeQueue,
    GuessNext { kind: PieceKind, prob: f32 },
    Hold { switch: bool },
    Move(Move),
    Place,
}

#[derive(Debug, PartialEq)]
pub enum ReduceError {
    Place(PlaceError),
    ConsumeQueue(ConsumeQueueError),
    Hold(HoldError),
    Move(MoveError),
    GameOver,
    NoPerfectClear,
}

#[derive(Debug, PartialEq)]
pub enum PlaceError {
    NoPiece,
    PieceInAir,
}

#[derive(Debug, PartialEq)]
pub enum ConsumeQueueError {
    QueueEmpty,
}

#[derive(Debug, PartialEq)]
pub enum HoldError {
    NotAvailable,
    NoHoldPiece,
    NoPiece,
}

#[derive(Debug, PartialEq)]
pub enum MoveError {
    NoPiece,
    InvalidMove,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Move {
    Rotate(Rotation),
    Translate(Direction),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Down,
}

impl Direction {
    pub fn get_offset(&self) -> Point<isize> {
        match self {
            Direction::Down => Point::new(0, -1),
            Direction::Left => Point::new(-1, 0),
            Direction::Right => Point::new(1, 0),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::config::RotationSystem;

    use super::*;

    const CONFIG: Config = Config {
        rotation_system: RotationSystem::SRS,
    };

    mod with_consumed_queue {
        use crate::point::Point;

        use super::*;

        #[test]
        fn invalid_if_queue_empty() {
            let state = State::initial();

            let next_state = state.reduce(&Action::ConsumeQueue, &CONFIG);

            assert_eq!(
                next_state,
                Err(ReduceError::ConsumeQueue(ConsumeQueueError::QueueEmpty)),
                "Expected state to be invalid if consuming an empty queue"
            );
        }

        #[test]
        fn invalid_if_new_piece_intersects_board() {
            let mut board = Board::empty_board();
            for x in 3..7 {
                board.fill(&Point::new(x, 20));
            }

            let mut queue: [Option<PieceKind>; 7] = [None; 7];
            queue[0] = Some(PieceKind::I);

            let state = State {
                board,
                queue,
                ..State::initial()
            };

            let next_state = state.reduce(&Action::ConsumeQueue, &CONFIG);

            assert_eq!(
                next_state,
                Err(ReduceError::GameOver),
                "Expected state to be invalid if next active piece intersects the board",
            )
        }

        #[test]
        fn resets_is_hold_used() {
            let mut queue: [Option<PieceKind>; 7] = [None; 7];
            queue[0] = Some(PieceKind::I);

            let state = State {
                queue,
                ..State::initial()
            };

            let next_state = state.reduce(&Action::ConsumeQueue, &CONFIG);

            assert!(next_state.is_ok());

            let next_state = next_state.unwrap();

            assert!(!next_state.is_hold_used);
        }

        #[test]
        fn consumes_queue_and_sets_piece() {
            let queue: [Option<PieceKind>; 7] = [
                Some(PieceKind::I),
                Some(PieceKind::J),
                Some(PieceKind::L),
                Some(PieceKind::O),
                Some(PieceKind::S),
                Some(PieceKind::T),
                Some(PieceKind::Z),
            ];

            let state = State {
                queue,
                ..State::initial()
            };

            let next_state = state.reduce(&Action::ConsumeQueue, &CONFIG);

            assert!(next_state.is_ok());
            let next_state = next_state.unwrap();

            assert!(next_state.piece.is_some());
            assert_eq!(next_state.piece.as_ref().unwrap().kind, PieceKind::I);
            assert_eq!(
                next_state.queue,
                [
                    Some(PieceKind::J),
                    Some(PieceKind::L),
                    Some(PieceKind::O),
                    Some(PieceKind::S),
                    Some(PieceKind::T),
                    Some(PieceKind::Z),
                    None,
                ]
            );

            let next_state = next_state.reduce(&Action::ConsumeQueue, &CONFIG);

            assert!(next_state.is_ok());
            let next_state = next_state.unwrap();

            assert!(next_state.piece.is_some());
            assert_eq!(next_state.piece.as_ref().unwrap().kind, PieceKind::J);
            assert_eq!(
                next_state.queue,
                [
                    Some(PieceKind::L),
                    Some(PieceKind::O),
                    Some(PieceKind::S),
                    Some(PieceKind::T),
                    Some(PieceKind::Z),
                    None,
                    None,
                ]
            );
        }
    }

    mod with_guessed_next {
        use super::*;

        #[test]
        fn invalid_if_new_piece_intersects_board() {
            let mut board = Board::empty_board();
            for x in 3..7 {
                board.fill(&Point::new(x, 20));
            }

            let state = State {
                board,
                ..State::initial()
            };

            let next_state = state.reduce(
                &Action::GuessNext {
                    kind: PieceKind::I,
                    prob: 0.5,
                },
                &CONFIG,
            );

            assert_eq!(
                next_state,
                Err(ReduceError::GameOver),
                "Expected state to be invalid if next active piece intersects the board",
            )
        }

        #[test]
        fn updates_prob_and_sets_piece() {
            let state = State::initial();

            let next_state = state.reduce(
                &Action::GuessNext {
                    kind: PieceKind::J,
                    prob: 0.5,
                },
                &CONFIG,
            );

            assert!(next_state.is_ok());
            let next_state = next_state.unwrap();

            assert!(next_state.piece.is_some());
            assert_eq!(next_state.piece.as_ref().unwrap().kind, PieceKind::J);

            assert_eq!(next_state.current_prob, 0.5);
        }
    }

    mod with_hold_used {
        use super::*;

        #[test]
        fn invalid_if_no_active_piece() {
            let state = State {
                hold_kind: Some(PieceKind::J),
                ..State::initial()
            };

            let next_state = state.reduce(&Action::Hold { switch: true }, &CONFIG);

            assert_eq!(next_state, Err(ReduceError::Hold(HoldError::NoPiece)));
        }

        #[test]
        fn invalid_if_no_hold_piece() {
            let state = State {
                piece: Some(Piece::spawn(&PieceKind::I, &CONFIG)),
                ..State::initial()
            };

            let next_state = state.reduce(&Action::Hold { switch: true }, &CONFIG);

            assert_eq!(next_state, Err(ReduceError::Hold(HoldError::NoHoldPiece)));
        }

        #[test]
        fn invalid_if_new_piece_intersects_board() {
            let mut board = Board::empty_board();
            for x in 3..7 {
                board.fill(&Point::new(x, 20));
            }

            let state = State {
                board,
                hold_kind: Some(PieceKind::I),
                piece: Some(Piece::spawn(&PieceKind::J, &CONFIG)),
                ..State::initial()
            };

            let next_state = state.reduce(&Action::Hold { switch: true }, &CONFIG);

            assert_eq!(
                next_state,
                Err(ReduceError::GameOver),
                "Expected state to be invalid if next active piece intersects the board",
            )
        }

        #[test]
        fn consumes_hold_and_swaps_hold() {
            let state = State {
                hold_kind: Some(PieceKind::J),
                piece: Some(Piece::spawn(&PieceKind::I, &CONFIG)),
                ..State::initial()
            };

            let next_state = state.reduce(&Action::Hold { switch: true }, &CONFIG);

            assert!(next_state.is_ok());
            let next_state = next_state.unwrap();

            assert!(next_state.is_hold_used);
            assert_eq!(next_state.hold_kind.unwrap(), PieceKind::I);
            assert_eq!(next_state.piece.as_ref().unwrap().kind, PieceKind::J);
        }

        #[test]
        fn consumes_hold_without_swapping_hold() {
            let state = State {
                hold_kind: Some(PieceKind::J),
                piece: Some(Piece::spawn(&PieceKind::I, &CONFIG)),
                ..State::initial()
            };

            let next_state = state.reduce(&Action::Hold { switch: false }, &CONFIG);

            assert!(next_state.is_ok());
            let next_state = next_state.unwrap();

            assert!(next_state.is_hold_used);
            assert_eq!(next_state.hold_kind.unwrap(), PieceKind::J);
            assert_eq!(next_state.piece.as_ref().unwrap().kind, PieceKind::I);
        }
    }

    mod with_rotation {
        use crate::rotation::Orientation;

        use super::*;

        mod i_piece {
            use super::*;

            #[test]
            fn no_kick() {
                let state = State {
                    piece: Some(Piece::spawn(&PieceKind::I, &CONFIG)),
                    ..State::initial()
                };

                let original_position = state.piece.as_ref().unwrap().position;

                let next_state =
                    state.reduce(&Action::Move(Move::Rotate(Rotation::Clockwise)), &CONFIG);

                assert!(next_state.is_ok());
                let next_state = next_state.unwrap();

                assert!(next_state.piece.is_some());
                assert_eq!(
                    next_state.piece.as_ref().unwrap().orientation,
                    Orientation::East
                );
                assert_eq!(
                    next_state.piece.as_ref().unwrap().position,
                    original_position,
                );
            }

            mod north_and_east {
                use crate::point::Point;

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

                    let state = State {
                        board,
                        piece: Some(Piece {
                            position: Point::new(3, 0),
                            ..Piece::spawn(&PieceKind::I, &CONFIG)
                        }),
                        ..State::initial()
                    };

                    let next_state =
                        state.reduce(&Action::Move(Move::Rotate(Rotation::Clockwise)), &CONFIG);

                    assert!(next_state.is_ok());
                    let next_state = next_state.unwrap();

                    assert!(next_state.piece.is_some());
                    assert_eq!(
                        next_state.piece.as_ref().unwrap().orientation,
                        Orientation::East
                    );
                    assert_eq!(
                        next_state.piece.as_ref().unwrap().position,
                        Point::new(1, 0),
                    );

                    let next_state = next_state.reduce(
                        &Action::Move(Move::Rotate(Rotation::AntiClockwise)),
                        &CONFIG,
                    );

                    assert!(next_state.is_ok());
                    let next_state = next_state.unwrap();

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

                    let state = State {
                        board,
                        piece: Some(Piece {
                            position: Point::new(3, 0),
                            ..Piece::spawn(&PieceKind::I, &CONFIG)
                        }),
                        ..State::initial()
                    };

                    let next_state =
                        state.reduce(&Action::Move(Move::Rotate(Rotation::Clockwise)), &CONFIG);

                    assert!(next_state.is_ok());
                    let next_state = next_state.unwrap();

                    assert!(next_state.piece.is_some());
                    assert_eq!(
                        next_state.piece.as_ref().unwrap().orientation,
                        Orientation::East
                    );
                    assert_eq!(
                        next_state.piece.as_ref().unwrap().position,
                        Point::new(4, 0),
                    );

                    let next_state = next_state.reduce(
                        &Action::Move(Move::Rotate(Rotation::AntiClockwise)),
                        &CONFIG,
                    );

                    assert!(next_state.is_ok());
                    let next_state = next_state.unwrap();

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

                    let state = State {
                        board,
                        piece: Some(Piece {
                            position: Point::new(3, 1),
                            ..Piece::spawn(&PieceKind::I, &CONFIG)
                        }),
                        ..State::initial()
                    };

                    let next_state =
                        state.reduce(&Action::Move(Move::Rotate(Rotation::Clockwise)), &CONFIG);

                    assert!(next_state.is_ok());
                    let next_state = next_state.unwrap();

                    assert!(next_state.piece.is_some());
                    assert_eq!(
                        next_state.piece.as_ref().unwrap().orientation,
                        Orientation::East
                    );
                    assert_eq!(
                        next_state.piece.as_ref().unwrap().position,
                        Point::new(1, 0),
                    );

                    let next_state = next_state.reduce(
                        &Action::Move(Move::Rotate(Rotation::AntiClockwise)),
                        &CONFIG,
                    );

                    assert!(next_state.is_ok());
                    let next_state = next_state.unwrap();

                    assert!(next_state.piece.is_some());
                    assert_eq!(
                        next_state.piece.as_ref().unwrap().orientation,
                        Orientation::North
                    );
                    assert_eq!(
                        next_state.piece.as_ref().unwrap().position,
                        Point::new(3, 1)
                    );
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

                    let state = State {
                        board,
                        piece: Some(Piece {
                            position: Point::new(3, 0),
                            ..Piece::spawn(&PieceKind::I, &CONFIG)
                        }),
                        ..State::initial()
                    };

                    let next_state =
                        state.reduce(&Action::Move(Move::Rotate(Rotation::Clockwise)), &CONFIG);

                    assert!(next_state.is_ok());
                    let next_state = next_state.unwrap();

                    assert!(next_state.piece.is_some());
                    assert_eq!(
                        next_state.piece.as_ref().unwrap().orientation,
                        Orientation::East
                    );
                    assert_eq!(
                        next_state.piece.as_ref().unwrap().position,
                        Point::new(4, 2),
                    );

                    let next_state = next_state.reduce(
                        &Action::Move(Move::Rotate(Rotation::AntiClockwise)),
                        &CONFIG,
                    );

                    assert!(next_state.is_ok());
                    let next_state = next_state.unwrap();

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
            }
        }
    }

    mod with_translation {
        use super::*;

        #[test]
        fn moves_piece() {
            let state = State {
                piece: Some(Piece {
                    position: Point::new(3, -1),
                    ..Piece::spawn(&PieceKind::I, &CONFIG)
                }),
                ..State::initial()
            };

            let next_state = state.reduce(&Action::Move(Move::Translate(Direction::Down)), &CONFIG);

            assert!(next_state.is_ok());
            let next_state = next_state.unwrap();

            let piece = next_state.piece.as_ref().unwrap();
            assert_eq!(piece.position, Point::new(3, -2));

            let next_state =
                next_state.reduce(&Action::Move(Move::Translate(Direction::Left)), &CONFIG);

            assert!(next_state.is_ok());
            let next_state = next_state.unwrap();

            let piece = next_state.piece.as_ref().unwrap();
            assert_eq!(piece.position, Point::new(2, -2));

            let next_state =
                next_state.reduce(&Action::Move(Move::Translate(Direction::Right)), &CONFIG);

            assert!(next_state.is_ok());
            let next_state = next_state.unwrap();

            let piece = next_state.piece.as_ref().unwrap();
            assert_eq!(piece.position, Point::new(3, -2));

            let next_state =
                next_state.reduce(&Action::Move(Move::Translate(Direction::Down)), &CONFIG);
            assert_eq!(next_state, Err(ReduceError::Move(MoveError::InvalidMove)));
        }
    }

    mod with_placed_piece {
        use crate::point::Point;

        use super::*;

        #[test]
        fn invalid_if_no_active_piece() {
            let state = State::initial();

            let next_state = state.reduce(&Action::Place, &CONFIG);

            assert_eq!(
                next_state,
                Err(ReduceError::Place(PlaceError::NoPiece)),
                "Expected state to be invalid if placing without active piece"
            );
        }

        #[test]
        fn invalid_if_piece_in_air() {
            let state = State {
                piece: Some(Piece {
                    position: Point::new(3, -1),
                    ..Piece::spawn(&PieceKind::I, &CONFIG)
                }),
                ..State::initial()
            };

            let next_state = state.reduce(&Action::Place, &CONFIG);

            assert_eq!(
                next_state,
                Err(ReduceError::Place(PlaceError::PieceInAir)),
                "Expected state to be invalid if placing without filled cell below piece"
            );
        }

        #[test]
        fn piece_placed() {
            let state = State {
                piece: Some(Piece {
                    position: Point::new(3, -2),
                    ..Piece::spawn(&PieceKind::I, &CONFIG)
                }),
                ..State::initial()
            };

            let next_state = state.reduce(&Action::Place, &CONFIG);

            assert!(next_state.is_ok());
            let next_state = next_state.unwrap();
            assert!(
                next_state.piece.is_none(),
                "Active piece should be none after placement"
            );

            let mut expected_board = Board::empty_board();
            expected_board.fill(&Point::new(3, 0));
            expected_board.fill(&Point::new(4, 0));
            expected_board.fill(&Point::new(5, 0));
            expected_board.fill(&Point::new(6, 0));
            assert_eq!(
                next_state.board, expected_board,
                "Previous active piece should fill the board after placement"
            );
        }
    }
}
