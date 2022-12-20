use crate::config::{srs, Config, RotationSystem};
use crate::game::{Action as GameAction, Game};
use crate::piece::{Piece, PIECE_KINDS};
use crate::state::{Action, QueueError, ReduceError, State};
use crate::utils::point::Point;
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

#[derive(Debug, Clone)]
pub struct PerfectClearResult {
    game_path: Vec<State>,
}

pub fn branch_state_to_perfect_clears(config: &Config, state: &State) -> Vec<PerfectClearResult> {
    let in_progress = vec![state.clone()];
    let mut perfect_clear_results = vec![];
    generate_next_states(config, state, in_progress, &mut perfect_clear_results);
    perfect_clear_results
}

fn generate_next_states(
    config: &Config,
    state: &State,
    in_progress: Vec<State>,
    perfect_clear_results: &mut Vec<PerfectClearResult>,
) {
    if state.game.board.can_perfect_clear() {
        println!("found a perfect clear solution");
        perfect_clear_results.push(PerfectClearResult {
            game_path: in_progress,
        });
        return;
    }

    if state.moves_remaining == 0 {
        println!("no solution after 10 moves");
        return;
    }

    branch_state_for_piece(config, state)
        .iter()
        .flat_map(|state_with_piece| {
            branch_game_on_hold(config, &state_with_piece.game)
                .into_iter()
                .map(move |game_after_hold| State {
                    game: game_after_hold,
                    ..state_with_piece.clone()
                })
        })
        .flat_map(|state_after_hold| {
            branch_game_to_placable_pieces(config, &state_after_hold.game)
                .into_iter()
                .map(move |game_after_move| State {
                    game: game_after_move,
                    ..state_after_hold.clone()
                })
        })
        .map(|state_after_move| {
            state_after_move
                .reduce(config, &Action::Play(GameAction::Place))
                .unwrap()
        })
        .for_each(|state_after_place| {
            // Any perfect clear must only fill lines 0 to 3.
            // Early return here before cloning work so far and state.
            if !state_after_place.game.board.is_line_empty(4) {
                return;
            }

            let mut next_in_progress = in_progress.to_vec();
            next_in_progress.push(state_after_place.clone());
            generate_next_states(
                config,
                &state_after_place,
                next_in_progress,
                perfect_clear_results,
            )
        });
}

const NEXT_PROB: f32 = 1.0 / 7.0;

pub fn branch_state_for_piece(config: &Config, state: &State) -> Vec<State> {
    if state.game.piece.is_some() {
        return vec![state.clone()];
    }
    match state.reduce(config, &Action::ConsumeQueue) {
        Ok(s) => vec![s],
        Err(ReduceError::ConsumeQueue(QueueError::QueueEmpty)) => PIECE_KINDS
            .iter()
            .filter_map(|&kind| {
                state
                    .reduce(
                        config,
                        &Action::GuessNext {
                            kind,
                            // Assume all next pieces are equally likely for now.
                            // TODO Calculate next probabilities.
                            prob: NEXT_PROB,
                        },
                    )
                    .ok()
            })
            .collect(),
        _ => vec![],
    }
}

pub fn branch_game_on_hold(config: &Config, game: &Game) -> Vec<Game> {
    [true, false]
        .iter()
        .filter_map(|&switch| game.reduce(config, &GameAction::Hold { switch }).ok())
        .collect()
}

type PlaceablePiecesKey = (Point, Orientation);

struct PlaceablePiecesValue {
    is_placable: bool,
    previous_key: Option<PlaceablePiecesKey>,
}

pub fn branch_game_to_placable_pieces(config: &Config, game: &Game) -> Vec<Game> {
    let Some(piece) = game.piece else {
            return vec![];
        };

    let mut memo = HashMap::new();

    generate_placable_pieces(config, game, &mut memo);

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
fn generate_placable_pieces(
    config: &Config,
    game: &Game,
    memo: &mut HashMap<PlaceablePiecesKey, PlaceablePiecesValue>,
) {
    let piece = game.piece.unwrap();

    match config.rotation_system {
        RotationSystem::SRS => srs::POSSIBLE_MOVES
            .iter()
            .filter_map(|&mov| game.reduce(config, &GameAction::Move(mov)).ok())
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
                    generate_placable_pieces(config, &next_game, memo);
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

            let next_games = branch_game_to_placable_pieces(&CONFIG, &game);
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
                    position: Point::new(horizontal_x, -2),
                };
                assert!(next_pieces.contains(&expected_piece));

                // South
                let expected_piece = Piece {
                    kind: PieceKind::I,
                    orientation: Orientation::South,
                    position: Point::new(horizontal_x, -1),
                };
                assert!(next_pieces.contains(&expected_piece));
            }

            for vertical_x in 0..10 {
                // East
                let expected_piece = Piece {
                    kind: PieceKind::I,
                    orientation: Orientation::East,
                    position: Point::new(vertical_x - 2, 0),
                };
                assert!(next_pieces.contains(&expected_piece));

                // West
                let expected_piece = Piece {
                    kind: PieceKind::I,
                    orientation: Orientation::West,
                    position: Point::new(vertical_x - 1, 0),
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
                        b.empty(&Point::new(x, y));
                    }
                }
                b.empty(&Point::new(3, 0));
                b.empty(&Point::new(3, 1));
                b.empty(&Point::new(4, 1));
                b.empty(&Point::new(3, 2));
                b.empty(&Point::new(3, 3));
                b.empty(&Point::new(4, 3));
                b.empty(&Point::new(5, 3));
                b.empty(&Point::new(4, 4));
                b.empty(&Point::new(5, 4));
                b
            };

            let game = Game {
                piece: Some(Piece::spawn(&CONFIG, &PieceKind::T)),
                board,
                ..Game::initial()
            };

            let next_games = branch_game_to_placable_pieces(&CONFIG, &game);
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
                position: Point::new(3, 4),
            };
            assert!(next_pieces.contains(&expected_piece));

            // South on overhang
            let expected_piece = Piece {
                kind: PieceKind::T,
                orientation: Orientation::South,
                position: Point::new(3, 4),
            };
            assert!(next_pieces.contains(&expected_piece));

            // East on overhang
            let expected_piece = Piece {
                kind: PieceKind::T,
                orientation: Orientation::East,
                position: Point::new(2, 5),
            };
            assert!(next_pieces.contains(&expected_piece));

            // West on overhang
            let expected_piece = Piece {
                kind: PieceKind::T,
                orientation: Orientation::West,
                position: Point::new(3, 4),
            };
            assert!(next_pieces.contains(&expected_piece));

            // East besides overhang
            let expected_piece = Piece {
                kind: PieceKind::T,
                orientation: Orientation::East,
                position: Point::new(3, 3),
            };
            assert!(next_pieces.contains(&expected_piece));

            // West besides overhang
            let expected_piece = Piece {
                kind: PieceKind::T,
                orientation: Orientation::West,
                position: Point::new(4, 3),
            };
            assert!(next_pieces.contains(&expected_piece));

            // North tucked under overhang
            let expected_piece = Piece {
                kind: PieceKind::T,
                orientation: Orientation::North,
                position: Point::new(3, 2),
            };
            assert!(next_pieces.contains(&expected_piece));

            // T-spin triple
            let expected_piece = Piece {
                kind: PieceKind::T,
                orientation: Orientation::East,
                position: Point::new(2, 0),
            };
            assert!(next_pieces.contains(&expected_piece));
        }
    }

    // mod tests {
    //     use super::*;

    //     #[test]
    //     pub fn test() {
    //         let state = State {
    //             game: Game {
    //                 piece: Some(Piece::spawn(&CONFIG, &PieceKind::I)),
    //                 queue: PIECE_KINDS.map(|kind| Some(kind)),
    //                 ..Game::initial()
    //             },
    //             ..State::initial()
    //         };
    //         let results = branch_state_to_perfect_clears(&CONFIG, &state);
    //         for result in results {
    //             let Some(last) = result.game_path.last() else {
    //                 continue;
    //             };
    //             println!("{:?}", last);
    //         }
    //     }
    // }
}
