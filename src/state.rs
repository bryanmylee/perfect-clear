use crate::config::Config;
use crate::game::Game;
use crate::piece::{Piece, PieceKind};

#[derive(Debug, Clone, PartialEq)]
pub struct State {
    pub game: Game,

    /// Only 2-bags needed at most to determine next piece probability.
    pub seen: [Option<PieceKind>; 14],

    pub moves_remaining: isize,

    pub current_prob: f32,
}

impl State {
    pub fn initial() -> State {
        State {
            game: Game::initial(),
            seen: [None; 14],
            moves_remaining: 10,
            current_prob: 1.0,
        }
    }

    pub fn reduce(&self, config: &Config, action: &Action) -> Result<State, ReduceError> {
        match action {
            Action::ConsumeQueue => self.with_consumed_queue(config),
            Action::GuessNext { kind, prob } => self.with_guessed_next(config, kind, *prob),
            Action::Place => self.with_placed_piece(config),
        }
    }

    fn with_consumed_queue(&self, config: &Config) -> Result<State, ReduceError> {
        let Some((Some(next_piece_kind), rest_piece_kinds)) = self.game.queue.split_first() else {
            return Err(ReduceError::ConsumeQueue(ConsumeQueueError::QueueEmpty));
        };

        let next_piece = Piece::spawn(next_piece_kind, config);

        if !self.game.board.can_fit(&next_piece.get_points(config)) {
            return Err(ReduceError::GameOver);
        }

        let mut new_queue = [None; 7];
        new_queue[..rest_piece_kinds.len()].clone_from_slice(rest_piece_kinds);

        let next_state = self.clone();
        Ok(State {
            game: Game {
                queue: new_queue,
                piece: Some(next_piece),
                is_hold_used: false,
                ..next_state.game
            },
            ..next_state
        })
    }

    fn with_guessed_next(
        &self,
        config: &Config,
        kind: &PieceKind,
        prob: f32,
    ) -> Result<State, ReduceError> {
        let next_piece = Piece::spawn(kind, config);

        if !self.game.board.can_fit(&next_piece.get_points(config)) {
            return Err(ReduceError::GameOver);
        }

        let next_state = self.clone();
        Ok(State {
            game: Game {
                piece: Some(next_piece),
                ..next_state.game
            },
            current_prob: self.current_prob * prob,
            ..next_state
        })
    }

    fn with_placed_piece(&self, config: &Config) -> Result<State, ReduceError> {
        let Some(piece) = &self.game.piece else {
            return Err(ReduceError::Place(PlaceError::NoPiece));
        };

        let piece_points = piece.get_points(config);

        if !self.game.board.can_place(&piece_points) {
            return Err(ReduceError::Place(PlaceError::PieceInAir));
        }

        let next_state = self.clone();
        let mut next_board = next_state.game.board;
        next_board.fill_piece_points(&piece_points);
        Ok(State {
            game: Game {
                board: next_board,
                piece: None,
                ..next_state.game
            },
            ..next_state
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Action {
    ConsumeQueue,
    GuessNext { kind: PieceKind, prob: f32 },
    Place,
}

#[derive(Debug, PartialEq)]
pub enum ReduceError {
    Place(PlaceError),
    ConsumeQueue(ConsumeQueueError),
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

#[cfg(test)]
mod tests {
    use crate::board::Board;
    use crate::config::RotationSystem;
    use crate::point::ISizePoint;

    use super::*;

    const CONFIG: Config = Config {
        rotation_system: RotationSystem::SRS,
    };

    mod with_consumed_queue {
        use super::*;

        #[test]
        fn invalid_if_queue_empty() {
            let state = State::initial();

            let next_state = state.reduce(&CONFIG, &Action::ConsumeQueue);

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
                board.fill(&ISizePoint::new(x, 20));
            }

            let mut queue: [Option<PieceKind>; 7] = [None; 7];
            queue[0] = Some(PieceKind::I);

            let state = State {
                game: Game {
                    board,
                    queue,
                    ..State::initial().game
                },
                ..State::initial()
            };

            let next_state = state.reduce(&CONFIG, &Action::ConsumeQueue);

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
                game: Game {
                    queue,
                    ..State::initial().game
                },
                ..State::initial()
            };

            let next_state = state.reduce(&CONFIG, &Action::ConsumeQueue);

            assert!(next_state.is_ok());

            let next_state = next_state.unwrap();

            assert!(!next_state.game.is_hold_used);
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
                game: Game {
                    queue,
                    ..State::initial().game
                },
                ..State::initial()
            };

            let next_state = state.reduce(&CONFIG, &Action::ConsumeQueue);

            assert!(next_state.is_ok());
            let next_state = next_state.unwrap();

            assert!(next_state.game.piece.is_some());
            assert_eq!(next_state.game.piece.as_ref().unwrap().kind, PieceKind::I);
            assert_eq!(
                next_state.game.queue,
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

            let next_state = next_state.reduce(&CONFIG, &Action::ConsumeQueue);

            assert!(next_state.is_ok());
            let next_state = next_state.unwrap();

            assert!(next_state.game.piece.is_some());
            assert_eq!(next_state.game.piece.as_ref().unwrap().kind, PieceKind::J);
            assert_eq!(
                next_state.game.queue,
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
                board.fill(&ISizePoint::new(x, 20));
            }

            let state = State {
                game: Game {
                    board,
                    ..State::initial().game
                },
                ..State::initial()
            };

            let next_state = state.reduce(
                &CONFIG,
                &Action::GuessNext {
                    kind: PieceKind::I,
                    prob: 0.5,
                },
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
                &CONFIG,
                &Action::GuessNext {
                    kind: PieceKind::J,
                    prob: 0.5,
                },
            );

            assert!(next_state.is_ok());
            let next_state = next_state.unwrap();

            assert!(next_state.game.piece.is_some());
            assert_eq!(next_state.game.piece.as_ref().unwrap().kind, PieceKind::J);

            assert_eq!(next_state.current_prob, 0.5);
        }
    }

    mod with_placed_piece {
        use super::*;

        #[test]
        fn invalid_if_no_active_piece() {
            let state = State::initial();

            let next_state = state.reduce(&CONFIG, &Action::Place);

            assert_eq!(
                next_state,
                Err(ReduceError::Place(PlaceError::NoPiece)),
                "Expected state to be invalid if placing without active piece"
            );
        }

        #[test]
        fn invalid_if_piece_in_air() {
            let state = State {
                game: Game {
                    piece: Some(Piece {
                        position: ISizePoint::new(3, -1),
                        ..Piece::spawn(&PieceKind::I, &CONFIG)
                    }),
                    ..State::initial().game
                },
                ..State::initial()
            };

            let next_state = state.reduce(&CONFIG, &Action::Place);

            assert_eq!(
                next_state,
                Err(ReduceError::Place(PlaceError::PieceInAir)),
                "Expected state to be invalid if placing without filled cell below piece"
            );
        }

        #[test]
        fn piece_placed() {
            let state = State {
                game: Game {
                    piece: Some(Piece {
                        position: ISizePoint::new(3, -2),
                        ..Piece::spawn(&PieceKind::I, &CONFIG)
                    }),
                    ..State::initial().game
                },
                ..State::initial()
            };

            let next_state = state.reduce(&CONFIG, &Action::Place);

            assert!(next_state.is_ok());
            let next_state = next_state.unwrap();
            assert!(
                next_state.game.piece.is_none(),
                "Active piece should be none after placement"
            );

            let mut expected_board = Board::empty_board();
            expected_board.fill(&ISizePoint::new(3, 0));
            expected_board.fill(&ISizePoint::new(4, 0));
            expected_board.fill(&ISizePoint::new(5, 0));
            expected_board.fill(&ISizePoint::new(6, 0));
            assert_eq!(
                next_state.game.board, expected_board,
                "Previous active piece should fill the board after placement"
            );
        }
    }
}
