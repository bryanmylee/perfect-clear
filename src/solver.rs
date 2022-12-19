use crate::config::{srs, Config, RotationSystem};
use crate::game::{Action, Game};
use crate::piece::Piece;
use crate::state::State;
use crate::utils::point::ISizePoint;
use crate::utils::rotation::Orientation;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Solver {
    current_state: State,
}

#[wasm_bindgen]
impl Solver {
    pub fn new() -> Solver {
        Solver {
            current_state: State::initial(),
        }
    }

    pub fn update_game(&mut self, game: Game) {
        self.current_state.game = game;
    }
}

pub fn branch_game_on_hold(game: &Game, config: &Config) -> Vec<Game> {
    [true, false]
        .iter()
        .filter_map(|&switch| game.reduce(config, &Action::Hold { switch }).ok())
        .collect()
}

type PlaceablePiecesKey = (ISizePoint, Orientation);

struct PlaceablePiecesValue {
    is_placable: bool,
    previous_key: Option<PlaceablePiecesKey>,
}

pub fn branch_game_to_placable_pieces(game: &Game, config: &Config) -> Vec<Game> {
    let Some(piece) = game.piece else {
            return vec![];
        };

    let mut memo = HashMap::new();

    memoize_game_to_placable_pieces(game, config, &mut memo);

    memo.into_iter()
        .filter_map(|(k, v)| if v.is_placable { Some(k) } else { None })
        .map(|(position, orientation)| Game {
            piece: Some(Piece {
                orientation,
                position,
                ..piece
            }),
            ..game.clone()
        })
        .collect()
}

/// For a given board and piece kind, each piece position and rotation should be memoized.
///
/// `self.piece` must be `Some` variant.
fn memoize_game_to_placable_pieces(
    game: &Game,
    config: &Config,
    memo: &mut HashMap<PlaceablePiecesKey, PlaceablePiecesValue>,
) {
    let piece = game.piece.unwrap();

    match config.rotation_system {
        RotationSystem::SRS => srs::POSSIBLE_MOVES
            .iter()
            .filter_map(|&mov| game.reduce(config, &Action::Move(mov)).ok())
            .for_each(|next_game| {
                let next_piece = next_game.piece.unwrap();
                let key = (next_piece.position, next_piece.orientation);
                if memo.contains_key(&key) {
                    // TODO relax path
                } else {
                    // set memo and continue branching
                    memo.entry((next_piece.position, next_piece.orientation))
                        .or_insert(PlaceablePiecesValue {
                            is_placable: next_game.board.can_place(&next_piece.get_points(config)),
                            previous_key: Some((piece.position, piece.orientation)),
                        });
                    memoize_game_to_placable_pieces(&next_game, config, memo);
                }
            }),
    }
}

#[cfg(test)]
mod tests {
    use crate::piece::PieceKind;

    use super::*;

    const CONFIG: Config = Config {
        rotation_system: RotationSystem::SRS,
    };

    mod branch_game_to_placable_pieces {
        use crate::board::Board;

        use super::*;

        #[test]
        fn i_piece_in_empty_board() {
            let game = Game {
                piece: Some(Piece::spawn(&CONFIG, &PieceKind::I)),
                ..Game::initial()
            };

            let next_games = branch_game_to_placable_pieces(&game, &CONFIG);
            let next_pieces = next_games
                .into_iter()
                .filter_map(|game| game.piece)
                .collect::<Vec<_>>();

            // 10 each for east and west
            // 7 each for north and south
            assert_eq!(next_pieces.len(), 10 + 10 + 7 + 7);

            for horizontal_x in 0..7 {
                // North
                let expected_piece = Piece {
                    kind: PieceKind::I,
                    orientation: Orientation::North,
                    position: ISizePoint::new(horizontal_x, -2),
                };
                assert!(next_pieces.contains(&expected_piece));

                // South
                let expected_piece = Piece {
                    kind: PieceKind::I,
                    orientation: Orientation::South,
                    position: ISizePoint::new(horizontal_x, -1),
                };
                assert!(next_pieces.contains(&expected_piece));
            }

            for vertical_x in 0..10 {
                // East
                let expected_piece = Piece {
                    kind: PieceKind::I,
                    orientation: Orientation::East,
                    position: ISizePoint::new(vertical_x - 2, 0),
                };
                assert!(next_pieces.contains(&expected_piece));

                // West
                let expected_piece = Piece {
                    kind: PieceKind::I,
                    orientation: Orientation::West,
                    position: ISizePoint::new(vertical_x - 1, 0),
                };
                assert!(next_pieces.contains(&expected_piece));
            }
        }

        #[test]
        fn t_spin_triple() {
            let board = {
                let mut b = Board::filled_board();
                for x in 3..=5 {
                    for y in 5..24 {
                        b.empty(&ISizePoint::new(x, y));
                    }
                }
                b.empty(&ISizePoint::new(3, 0));
                b.empty(&ISizePoint::new(3, 1));
                b.empty(&ISizePoint::new(4, 1));
                b.empty(&ISizePoint::new(3, 2));
                b.empty(&ISizePoint::new(3, 3));
                b.empty(&ISizePoint::new(4, 3));
                b.empty(&ISizePoint::new(5, 3));
                b.empty(&ISizePoint::new(4, 4));
                b.empty(&ISizePoint::new(5, 4));
                b
            };

            let game = Game {
                piece: Some(Piece::spawn(&CONFIG, &PieceKind::T)),
                board,
                ..Game::initial()
            };

            let next_games = branch_game_to_placable_pieces(&game, &CONFIG);
            let next_pieces = next_games
                .into_iter()
                .filter_map(|game| game.piece)
                .collect::<Vec<_>>();

            // 4 on the overhang for all orientations
            // 2 beside the overhang for east and west
            // 1 for north tucked under the overhang
            // 1 for the t-spin triple
            assert_eq!(next_pieces.len(), 4 + 2 + 1 + 1);

            // North on overhang
            let expected_piece = Piece {
                kind: PieceKind::T,
                orientation: Orientation::North,
                position: ISizePoint::new(3, 4),
            };
            assert!(next_pieces.contains(&expected_piece));

            // South on overhang
            let expected_piece = Piece {
                kind: PieceKind::T,
                orientation: Orientation::South,
                position: ISizePoint::new(3, 4),
            };
            assert!(next_pieces.contains(&expected_piece));

            // East on overhang
            let expected_piece = Piece {
                kind: PieceKind::T,
                orientation: Orientation::East,
                position: ISizePoint::new(2, 5),
            };
            assert!(next_pieces.contains(&expected_piece));

            // West on overhang
            let expected_piece = Piece {
                kind: PieceKind::T,
                orientation: Orientation::West,
                position: ISizePoint::new(3, 4),
            };
            assert!(next_pieces.contains(&expected_piece));

            // East besides overhang
            let expected_piece = Piece {
                kind: PieceKind::T,
                orientation: Orientation::East,
                position: ISizePoint::new(3, 3),
            };
            assert!(next_pieces.contains(&expected_piece));

            // West besides overhang
            let expected_piece = Piece {
                kind: PieceKind::T,
                orientation: Orientation::West,
                position: ISizePoint::new(4, 3),
            };
            assert!(next_pieces.contains(&expected_piece));

            // North tucked under overhang
            let expected_piece = Piece {
                kind: PieceKind::T,
                orientation: Orientation::North,
                position: ISizePoint::new(3, 2),
            };
            assert!(next_pieces.contains(&expected_piece));

            // T-spin triple
            let expected_piece = Piece {
                kind: PieceKind::T,
                orientation: Orientation::East,
                position: ISizePoint::new(2, 0),
            };
            assert!(next_pieces.contains(&expected_piece));
        }
    }
}
