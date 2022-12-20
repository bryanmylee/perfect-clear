use crate::board::Board;
use crate::config::{srs, Config, RotationSystem};
use crate::game::{Action as GameAction, Game};
use crate::piece::{Piece, PieceKind, PIECE_KINDS};
use crate::state::{Action, QueueError, ReduceError, State};
use crate::utils::point::Point;
use crate::utils::rotation::Orientation;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
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

pub struct SolverNode {
    previous_node: Option<*const SolverNode>,
    board: Board,
    piece_kind: PieceKind,
}

impl Hash for SolverNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.previous_node.hash(state);
        self.board.hash(state);
        self.piece_kind.hash(state);
    }
}

impl PartialEq for SolverNode {
    fn eq(&self, other: &Self) -> bool {
        self.board == other.board && self.piece_kind == other.piece_kind
    }
}

impl Eq for SolverNode {}

pub fn branch_state_to_perfect_clears(
    config: &Config,
    state: &State,
) -> Vec<Vec<(Board, PieceKind)>> {
    let mut nodes = HashSet::new();
    generate_next_states(config, state, None, &mut nodes);
    nodes
        .iter()
        .filter(|node| node.board.can_perfect_clear())
        .map(get_state_path)
        .collect()
}

fn get_state_path(node: &SolverNode) -> Vec<(Board, PieceKind)> {
    let mut node = node;
    let mut path = vec![(node.board, node.piece_kind)];
    while let Some(previous_node) = node.previous_node {
        unsafe {
            let previous_node = &*previous_node;
            path.push((previous_node.board, previous_node.piece_kind));
            node = previous_node
        }
    }
    path
}

fn generate_next_states(
    config: &Config,
    state: &State,
    previous_node: Option<*const SolverNode>,
    nodes: &mut HashSet<SolverNode>,
) {
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
            (
                state_after_move
                    .reduce(config, &Action::Play(GameAction::Place))
                    .unwrap(),
                state_after_move.game.piece.unwrap().kind,
            )
        })
        .for_each(|(state_after_place, piece_kind)| {
            // Any perfect clear must only fill lines 0 to 3.
            if !state_after_place.game.board.is_line_empty(4) {
                return;
            }

            if state_after_place.game.board.can_perfect_clear() {
                println!("found a perfect clear solution");
                let node = nodes.get_or_insert(SolverNode {
                    previous_node,
                    board: state_after_place.game.board,
                    piece_kind,
                });
                println!("{:?}", get_state_path(node));
                return;
            }

            if state_after_place.moves_remaining == 0 {
                println!("no solution after 10 moves");
                return;
            }

            let node = nodes.get_or_insert(SolverNode {
                previous_node,
                board: state_after_place.game.board,
                piece_kind,
            });

            generate_next_states(config, &state_after_place, Some(node), nodes)
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
    }

    mod tests {
        use super::*;

        #[test]
        pub fn test() {
            let state = State {
                game: Game {
                    piece: Some(Piece::spawn(&CONFIG, &PieceKind::I)),
                    queue: PIECE_KINDS.map(|kind| Some(kind)),
                    ..Game::initial()
                },
                ..State::initial()
            };
            let results = branch_state_to_perfect_clears(&CONFIG, &state);
            for result in results {
                println!("{:?}", result);
            }
        }
    }
}
