use crate::board::Board;
use crate::config::{srs, Config};
use crate::piece::{Piece, PieceKind};
use crate::point::ISizePoint;
use crate::rotation::Rotation;
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

    pub fn with_move(&self, config: &Config, mov: &Move) -> Result<Game, MoveError> {
        match mov {
            Move::Rotate(rotation) => self.with_rotation(config, &rotation),
            Move::Translate(direction) => self.with_translation(config, &direction),
        }
    }

    fn with_rotation(&self, config: &Config, rotation: &Rotation) -> Result<Game, MoveError> {
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

        let Some(kicks) = srs::kick_table(&piece.kind, &from_orientation, &to_orientation) else {
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

    fn with_translation(&self, config: &Config, direction: &Direction) -> Result<Game, MoveError> {
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
pub enum Move {
    Rotate(Rotation),
    Translate(Direction),
}

#[derive(Debug, PartialEq)]
pub enum MoveError {
    NoPiece,
    InvalidMove,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Down,
}

impl Direction {
    pub fn get_offset(&self) -> ISizePoint {
        match self {
            Direction::Down => ISizePoint::new(0, -1),
            Direction::Left => ISizePoint::new(-1, 0),
            Direction::Right => ISizePoint::new(1, 0),
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

    mod with_rotation {
        use crate::rotation::Orientation;

        use super::*;

        mod i_piece {
            use super::*;

            #[test]
            fn no_kick() {
                let game = Game {
                    piece: Some(Piece::spawn(&PieceKind::I, &CONFIG)),
                    ..Game::initial()
                };

                let original_position = game.piece.as_ref().unwrap().position;

                let next_game = game.with_move(&CONFIG, &Move::Rotate(Rotation::Clockwise));

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

                    board.empty(&ISizePoint::new(3, 2));
                    board.empty(&ISizePoint::new(4, 2));
                    board.empty(&ISizePoint::new(5, 2));
                    board.empty(&ISizePoint::new(6, 2));

                    board.empty(&ISizePoint::new(3, 0));
                    board.empty(&ISizePoint::new(3, 1));
                    board.empty(&ISizePoint::new(3, 2));
                    board.empty(&ISizePoint::new(3, 3));

                    let game = Game {
                        board,
                        piece: Some(Piece {
                            position: ISizePoint::new(3, 0),
                            ..Piece::spawn(&PieceKind::I, &CONFIG)
                        }),
                        ..Game::initial()
                    };

                    let next_game = game.with_move(&CONFIG, &Move::Rotate(Rotation::Clockwise));

                    assert!(next_game.is_ok());
                    let next_game = next_game.unwrap();

                    assert!(next_game.piece.is_some());
                    assert_eq!(
                        next_game.piece.as_ref().unwrap().orientation,
                        Orientation::East
                    );
                    assert_eq!(
                        next_game.piece.as_ref().unwrap().position,
                        ISizePoint::new(1, 0),
                    );

                    let next_game =
                        next_game.with_move(&CONFIG, &Move::Rotate(Rotation::AntiClockwise));

                    assert!(next_game.is_ok());
                    let next_state = next_game.unwrap();

                    assert!(next_state.piece.is_some());
                    assert_eq!(
                        next_state.piece.as_ref().unwrap().orientation,
                        Orientation::North
                    );
                    assert_eq!(
                        next_state.piece.as_ref().unwrap().position,
                        ISizePoint::new(3, 0)
                    );
                }

                #[test]
                fn kick_two() {
                    let mut board = Board::filled_board();

                    board.empty(&ISizePoint::new(3, 2));
                    board.empty(&ISizePoint::new(4, 2));
                    board.empty(&ISizePoint::new(5, 2));
                    board.empty(&ISizePoint::new(6, 2));

                    board.empty(&ISizePoint::new(6, 0));
                    board.empty(&ISizePoint::new(6, 1));
                    board.empty(&ISizePoint::new(6, 2));
                    board.empty(&ISizePoint::new(6, 3));

                    let game = Game {
                        board,
                        piece: Some(Piece {
                            position: ISizePoint::new(3, 0),
                            ..Piece::spawn(&PieceKind::I, &CONFIG)
                        }),
                        ..Game::initial()
                    };

                    let next_game = game.with_move(&CONFIG, &Move::Rotate(Rotation::Clockwise));

                    assert!(next_game.is_ok());
                    let next_game = next_game.unwrap();

                    assert!(next_game.piece.is_some());
                    assert_eq!(
                        next_game.piece.as_ref().unwrap().orientation,
                        Orientation::East
                    );
                    assert_eq!(
                        next_game.piece.as_ref().unwrap().position,
                        ISizePoint::new(4, 0),
                    );

                    let next_game =
                        next_game.with_move(&CONFIG, &Move::Rotate(Rotation::AntiClockwise));

                    assert!(next_game.is_ok());
                    let next_game = next_game.unwrap();

                    assert!(next_game.piece.is_some());
                    assert_eq!(
                        next_game.piece.as_ref().unwrap().orientation,
                        Orientation::North
                    );
                    assert_eq!(
                        next_game.piece.as_ref().unwrap().position,
                        ISizePoint::new(3, 0)
                    );
                }

                #[test]
                fn kick_three() {
                    let mut board = Board::filled_board();

                    board.empty(&ISizePoint::new(3, 3));
                    board.empty(&ISizePoint::new(4, 3));
                    board.empty(&ISizePoint::new(5, 3));
                    board.empty(&ISizePoint::new(6, 3));

                    board.empty(&ISizePoint::new(3, 0));
                    board.empty(&ISizePoint::new(3, 1));
                    board.empty(&ISizePoint::new(3, 2));
                    board.empty(&ISizePoint::new(3, 3));

                    let game = Game {
                        board,
                        piece: Some(Piece {
                            position: ISizePoint::new(3, 1),
                            ..Piece::spawn(&PieceKind::I, &CONFIG)
                        }),
                        ..Game::initial()
                    };

                    let next_game = game.with_move(&CONFIG, &Move::Rotate(Rotation::Clockwise));

                    assert!(next_game.is_ok());
                    let next_game = next_game.unwrap();

                    assert!(next_game.piece.is_some());
                    assert_eq!(
                        next_game.piece.as_ref().unwrap().orientation,
                        Orientation::East
                    );
                    assert_eq!(
                        next_game.piece.as_ref().unwrap().position,
                        ISizePoint::new(1, 0),
                    );

                    let next_game =
                        next_game.with_move(&CONFIG, &Move::Rotate(Rotation::AntiClockwise));

                    assert!(next_game.is_ok());
                    let next_game = next_game.unwrap();

                    assert!(next_game.piece.is_some());
                    assert_eq!(
                        next_game.piece.as_ref().unwrap().orientation,
                        Orientation::North
                    );
                    assert_eq!(
                        next_game.piece.as_ref().unwrap().position,
                        ISizePoint::new(3, 1)
                    );
                }

                #[test]
                fn kick_four() {
                    let mut board = Board::filled_board();

                    board.empty(&ISizePoint::new(3, 2));
                    board.empty(&ISizePoint::new(4, 2));
                    board.empty(&ISizePoint::new(5, 2));
                    board.empty(&ISizePoint::new(6, 2));

                    board.empty(&ISizePoint::new(6, 2));
                    board.empty(&ISizePoint::new(6, 3));
                    board.empty(&ISizePoint::new(6, 4));
                    board.empty(&ISizePoint::new(6, 5));

                    let game = Game {
                        board,
                        piece: Some(Piece {
                            position: ISizePoint::new(3, 0),
                            ..Piece::spawn(&PieceKind::I, &CONFIG)
                        }),
                        ..Game::initial()
                    };

                    let next_game = game.with_move(&CONFIG, &Move::Rotate(Rotation::Clockwise));

                    assert!(next_game.is_ok());
                    let next_game = next_game.unwrap();

                    assert!(next_game.piece.is_some());
                    assert_eq!(
                        next_game.piece.as_ref().unwrap().orientation,
                        Orientation::East
                    );
                    assert_eq!(
                        next_game.piece.as_ref().unwrap().position,
                        ISizePoint::new(4, 2),
                    );

                    let next_game =
                        next_game.with_move(&CONFIG, &Move::Rotate(Rotation::AntiClockwise));

                    assert!(next_game.is_ok());
                    let next_game = next_game.unwrap();

                    assert!(next_game.piece.is_some());
                    assert_eq!(
                        next_game.piece.as_ref().unwrap().orientation,
                        Orientation::North
                    );
                    assert_eq!(
                        next_game.piece.as_ref().unwrap().position,
                        ISizePoint::new(3, 0)
                    );
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
                    position: ISizePoint::new(3, -1),
                    ..Piece::spawn(&PieceKind::I, &CONFIG)
                }),
                ..Game::initial()
            };

            let next_game = game.with_move(&CONFIG, &Move::Translate(Direction::Down));

            assert!(next_game.is_ok());
            let next_game = next_game.unwrap();

            let piece = next_game.piece.as_ref().unwrap();
            assert_eq!(piece.position, ISizePoint::new(3, -2));

            let next_game = next_game.with_move(&CONFIG, &Move::Translate(Direction::Left));

            assert!(next_game.is_ok());
            let next_game = next_game.unwrap();

            let piece = next_game.piece.as_ref().unwrap();
            assert_eq!(piece.position, ISizePoint::new(2, -2));

            let next_game = next_game.with_move(&CONFIG, &Move::Translate(Direction::Right));

            assert!(next_game.is_ok());
            let next_game = next_game.unwrap();

            let piece = next_game.piece.as_ref().unwrap();
            assert_eq!(piece.position, ISizePoint::new(3, -2));

            let next_game = next_game.with_move(&CONFIG, &Move::Translate(Direction::Down));
            assert_eq!(next_game, Err(MoveError::InvalidMove));
        }
    }
}
