use crate::board::Board;
use crate::config::Config;
use crate::piece::{Piece, PieceKind};

#[derive(Debug, Clone, PartialEq)]
pub struct State {
    board: Board,
    piece: Option<Piece>,
    hold: Option<PieceKind>,
    is_hold_used: bool,
    queue: [Option<PieceKind>; 7], // fixed queue size to reduce heap allocations
    seen: [Option<PieceKind>; 14], // only 2-bags needed at most to determine next piece probability
    moves_remaining: isize,
    current_probability: f32,
}

impl State {
    pub fn initial() -> State {
        State {
            board: Board::empty_board(),
            piece: None,
            hold: None,
            is_hold_used: false,
            queue: [None; 7],
            seen: [None; 14],
            moves_remaining: 10,
            current_probability: 1.0,
        }
    }
}

impl State {
    pub fn reduce(&self, action: &Action, config: &Config) -> Result<State, ReduceError> {
        match action {
            Action::ConsumeQueue => self.with_consumed_queue(config),
            Action::GuessNext(piece_kind, with_probability) => {
                self.with_guessed_next(config, piece_kind, *with_probability)
            }
            Action::Place => self.with_placed_piece(config),
        }
    }

    fn with_consumed_queue(&self, config: &Config) -> Result<State, ReduceError> {
        let mut new_state = self.clone();

        let Some((Some(next_piece_kind), rest_piece_kinds)) = self.queue.split_first() else {
            return Err(ReduceError::ConsumeQueue(ConsumeQueueError::QueueEmpty));
        };

        new_state.queue = [None; 7];

        new_state.queue[..rest_piece_kinds.len()].clone_from_slice(rest_piece_kinds);

        let next_piece = Piece::spawn(next_piece_kind, config);

        if !new_state.board.can_fit(&next_piece.get_points(config)) {
            return Err(ReduceError::GameOver);
        }

        new_state.piece = Some(next_piece);

        new_state.is_hold_used = false;

        Ok(new_state)
    }

    fn with_guessed_next(
        &self,
        config: &Config,
        piece_kind: &PieceKind,
        with_probability: f32,
    ) -> Result<State, ReduceError> {
        let mut new_state = self.clone();

        new_state.piece = Some(Piece::spawn(piece_kind, config));

        new_state.current_probability *= with_probability;

        Ok(new_state)
    }

    fn with_placed_piece(&self, config: &Config) -> Result<State, ReduceError> {
        let mut new_state = self.clone();

        let Some(piece) = &self.piece else {
            return Err(ReduceError::Place(PlaceError::NoPiece));
        };

        let piece_points = piece.get_points(config);

        if !self.board.can_place(&piece_points) {
            return Err(ReduceError::Place(PlaceError::PieceInAir));
        }

        new_state.board.fill_piece_points(&piece_points);

        new_state.piece = None;

        Ok(new_state)
    }
}

#[derive(Debug, Clone)]
pub enum Action {
    ConsumeQueue,
    GuessNext(PieceKind, f32),
    Place,
}

#[derive(Debug, PartialEq)]
pub enum ReduceError {
    Place(PlaceError),
    ConsumeQueue(ConsumeQueueError),
    GameOver,
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

#[derive(Debug, Clone)]
pub enum Rotation {
    Clockwise,
    AntiClockwise,
    Half,
}

#[derive(Debug, Clone)]
pub enum Direction {
    Left,
    Right,
    Down,
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
                board.fill(&Point { x, y: 20 });
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
        fn updates_probability_and_sets_piece() {
            let state = State::initial();

            let next_state = state.reduce(&Action::GuessNext(PieceKind::J, 0.5), &CONFIG);

            assert!(next_state.is_ok());
            let next_state = next_state.unwrap();

            assert!(next_state.piece.is_some());
            assert_eq!(next_state.piece.as_ref().unwrap().kind, PieceKind::J);

            assert_eq!(next_state.current_probability, 0.5);
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
                    position: Point { x: 3, y: -1 },
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
                    position: Point { x: 3, y: -2 },
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
            expected_board.fill(&Point { x: 3, y: 0 });
            expected_board.fill(&Point { x: 4, y: 0 });
            expected_board.fill(&Point { x: 5, y: 0 });
            expected_board.fill(&Point { x: 6, y: 0 });
            assert_eq!(
                next_state.board, expected_board,
                "Previous active piece should fill the board after placement"
            );
        }
    }
}
