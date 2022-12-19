use crate::config::Config;
use crate::game::{Action as GameAction, Game, ReduceError as GameError};
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
            Action::ConsumeQueue => self
                .with_consumed_queue(config)
                .map_err(|e| ReduceError::ConsumeQueue(e)),
            Action::GuessNext { kind, prob } => self
                .with_guessed_next(config, kind, *prob)
                .map_err(|e| ReduceError::ConsumeQueue(e)),
            Action::Play(action) => self
                .game
                .reduce(config, action)
                .map(|game| State {
                    game,
                    ..self.clone()
                })
                .map_err(|e| ReduceError::Play(e)),
        }
    }

    fn with_consumed_queue(&self, config: &Config) -> Result<State, QueueError> {
        let Some((Some(next_piece_kind), rest_piece_kinds)) = self.game.queue.split_first() else {
            return Err(QueueError::QueueEmpty);
        };

        let next_piece = Piece::spawn(config, next_piece_kind);

        if !self.game.board.can_fit(&next_piece.get_points(config)) {
            return Err(QueueError::PieceCollision);
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
    ) -> Result<State, QueueError> {
        let next_piece = Piece::spawn(config, kind);

        if !self.game.board.can_fit(&next_piece.get_points(config)) {
            return Err(QueueError::PieceCollision);
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
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Action {
    ConsumeQueue,
    GuessNext { kind: PieceKind, prob: f32 },
    Play(GameAction),
}

#[derive(Debug, PartialEq)]
pub enum ReduceError {
    ConsumeQueue(QueueError),
    Play(GameError),
}

#[derive(Debug, PartialEq)]
pub enum QueueError {
    QueueEmpty,
    PieceCollision,
}

#[cfg(test)]
mod tests {
    use crate::board::Board;
    use crate::config::RotationSystem;
    use crate::utils::point::ISizePoint;

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
                Err(ReduceError::ConsumeQueue(QueueError::QueueEmpty)),
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
                Err(ReduceError::ConsumeQueue(QueueError::PieceCollision)),
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
                Err(ReduceError::ConsumeQueue(QueueError::PieceCollision)),
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
}
