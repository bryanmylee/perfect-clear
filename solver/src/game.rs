use crate::board::Board;
use crate::config::Config;
use crate::piece::{Piece, PieceKind};

#[derive(Debug, Clone)]
pub struct NextProbability {
    i: f64,
    j: f64,
    l: f64,
    o: f64,
    s: f64,
    t: f64,
    z: f64,
}

#[derive(Debug, Clone)]
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

#[derive(Debug)]
enum StateError {
    NoActivePiece,
    QueueEmpty,
    GameOver,
}

impl State {
    pub fn reduce(&self, action: &Action, config: &Config) -> Result<State, StateError> {
        match action {
            Action::Place => self.with_placed_piece(config),
            _ => Ok(self.clone()),
        }
    }

    fn with_placed_piece(&self, config: &Config) -> Result<State, StateError> {
        let mut new_state = self.clone();

        let Some(piece) = &self.piece else {
            return Err(StateError::NoActivePiece);
        };
        new_state.board.fill_piece_points(&piece.get_points(config));
        new_state.piece = None;

        Ok(new_state)
    }

    fn with_next_piece(&self, config: &Config) -> Result<State, StateError> {
        let mut new_state = self.clone();

        let Some((Some(next_piece_kind), rest_piece_kinds)) = self.queue.split_first() else {
            return Err(StateError::QueueEmpty);
        };
        new_state.queue = [None; 7];
        let new_queue_size = rest_piece_kinds.len();
        new_state.queue[..new_queue_size].clone_from_slice(rest_piece_kinds);

        new_state.is_hold_used = false;

        let next_piece = Piece::spawn(next_piece_kind, config);

        if !new_state.board.can_fit(&next_piece.get_points(config)) {
            return Err(StateError::GameOver);
        }

        new_state.piece = Some(next_piece);

        Ok(new_state)
    }
}

#[derive(Debug, Clone)]
pub enum Action {
    Rotate(Rotation),
    Move(Direction),
    Drop,
    Hold,
    Place,
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

    mod with_placed_piece {
        use crate::{piece::Orientation, point::Point};

        use super::*;

        #[test]
        fn invalid_if_no_active_piece() {
            let state = State::initial();

            let next_state = state.reduce(&Action::Place, &CONFIG);

            assert!(
                next_state.is_err(),
                "Expected state to be invalid if placing without active piece"
            );
        }

        #[test]
        fn piece_placed() {
            let mut state = State::initial();
            let piece = Piece {
                kind: PieceKind::I,
                orientation: Orientation::North,
                position: Point { x: 3, y: -2 },
            };
            state.piece = Some(piece);

            let next_state = state.reduce(&Action::Place, &CONFIG);

            assert!(next_state.is_ok());
            let next_state = next_state.unwrap();
            assert!(next_state.piece.is_none());

            let mut expected_board = Board::empty_board();
            expected_board.fill(&Point { x: 3, y: 0 });
            expected_board.fill(&Point { x: 4, y: 0 });
            expected_board.fill(&Point { x: 5, y: 0 });
            expected_board.fill(&Point { x: 6, y: 0 });
            assert_eq!(next_state.board, expected_board);
        }
    }
}
