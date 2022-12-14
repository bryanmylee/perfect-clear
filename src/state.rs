use crate::config::Config;
use crate::game::{Action as GameAction, Game, ReduceError as GameError};
use crate::piece::{Piece, PieceKind};
use crate::utils::piece_kind_set::PieceKindSet;

#[derive(Debug, Clone, PartialEq)]
pub struct State {
    pub game: Game,

    pub seen_piece_kind_in_bag: PieceKindSet<bool>,

    pub moves_remaining: u8,
}

impl State {
    pub fn initial() -> State {
        State {
            game: Game::initial(),
            seen_piece_kind_in_bag: PieceKindSet::new_with_value(false),
            moves_remaining: 10,
        }
    }

    pub fn reduce(&self, config: &Config, action: &Action) -> Result<State, ReduceError> {
        match action {
            Action::ConsumeQueue => self
                .with_consumed_queue(config)
                .map_err(|e| ReduceError::ConsumeQueue(e)),
            Action::WithNextPiece { kind } => self
                .with_next_piece(config, kind)
                .map_err(|e| ReduceError::ConsumeQueue(e)),
            Action::Play(action) => self
                .game
                .reduce(config, action)
                .map(|game| State {
                    game,
                    moves_remaining: if *action == GameAction::Place {
                        self.moves_remaining - 1
                    } else {
                        self.moves_remaining
                    },
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

    fn with_next_piece(&self, config: &Config, kind: &PieceKind) -> Result<State, QueueError> {
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
            ..next_state
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Action {
    ConsumeQueue,
    WithNextPiece { kind: PieceKind },
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
    use crate::utils::point::Point;

    use super::*;

    const CONFIG: Config = Config::default();

    mod with_consumed_queue {
        use crate::piece::PIECE_KINDS;

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
                board.fill(&Point::new(x, 2));
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
            let queue: [Option<PieceKind>; 7] = PIECE_KINDS.map(|k| Some(k));

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
                board.fill(&Point::new(x, 2));
            }

            let state = State {
                game: Game {
                    board,
                    ..State::initial().game
                },
                ..State::initial()
            };

            let next_state = state.reduce(&CONFIG, &Action::WithNextPiece { kind: PieceKind::I });

            assert_eq!(
                next_state,
                Err(ReduceError::ConsumeQueue(QueueError::PieceCollision)),
                "Expected state to be invalid if next active piece intersects the board",
            )
        }

        #[test]
        fn updates_piece() {
            let state = State::initial();

            let next_state = state.reduce(&CONFIG, &Action::WithNextPiece { kind: PieceKind::J });

            assert!(next_state.is_ok());
            let next_state = next_state.unwrap();

            assert!(next_state.game.piece.is_some());
            assert_eq!(next_state.game.piece.as_ref().unwrap().kind, PieceKind::J);
        }
    }
}
