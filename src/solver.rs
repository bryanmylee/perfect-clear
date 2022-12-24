use crate::board::Board;
use crate::config::Config;
use crate::game::{Action as GameAction, Game};
use crate::piece::{Piece, PieceKind, PIECE_KINDS};
use crate::state::{Action, State};
use crate::utils::point::Point;
use crate::utils::rotation::Orientation;
use crate::utils::weight_indexed_graph::WeightIndexedGraph;
use petgraph::algo::all_simple_paths;
use petgraph::graph::NodeIndex;
use petgraph::visit::EdgeRef;
use std::collections::HashMap;
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct GraphNode {
    board: Board,
    moves_remaining: u8,
    is_valid: bool,
}

impl Hash for GraphNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.board.hash(state);
        self.moves_remaining.hash(state);
    }
}

#[derive(Debug)]
struct GraphEdge {
    piece_kind: PieceKind,
}

pub fn get_perfect_clear_paths(
    config: &Config,
    state: &State,
) -> Vec<Vec<(Board, PieceKind, f32)>> {
    let mut node_graph = WeightIndexedGraph::new();
    let board_too_high = !state.game.board.is_line_empty(4);
    let can_perfect_clear = state.game.board.can_perfect_clear();
    let out_of_moves = state.moves_remaining == 0 && !can_perfect_clear;
    let is_valid = !board_too_high && !out_of_moves;
    let node_idx = node_graph.update_node(GraphNode {
        board: state.game.board,
        moves_remaining: state.moves_remaining,
        is_valid,
    });
    generate_next_states(config, state, node_idx, &mut node_graph);
    get_perfect_clear_paths_from_graph(&node_graph)
}

fn get_perfect_clear_paths_from_graph(
    node_graph: &WeightIndexedGraph<GraphNode, GraphEdge>,
) -> Vec<Vec<(Board, PieceKind, f32)>> {
    // TODO Re-implement this.
    vec![]
    // let Some(empty_idx) = node_graph.get_node_index(Board::empty_board()) else {
    //     return vec![];
    // };
    // let graph = &node_graph.graph;

    // let paths = Board::PC_BOARDS
    //     .iter()
    //     .filter_map(|&board| node_graph.get_node_index(board))
    //     .map(|pc_idx| {
    //         all_simple_paths::<Vec<_>, _>(graph, empty_idx, pc_idx, 4, None).collect::<Vec<_>>()
    //     })
    //     .reduce(|total, prev| [total, prev].concat())
    //     .unwrap_or(vec![]);

    // paths
    //     .iter()
    //     .map(|indices| {
    //         indices
    //             .windows(2)
    //             .map(|window| {
    //                 let from = window[0];
    //                 let to = window[1];
    //                 let from_board = graph[from];
    //                 let edge = graph.edges_connecting(from, to).next().unwrap().weight();
    //                 (from_board, edge.piece_kind, edge.probability)
    //             })
    //             .collect()
    //     })
    //     .collect()
}

fn generate_next_states(
    config: &Config,
    previous_state: &State,
    previous_node_idx: NodeIndex,
    node_graph: &mut WeightIndexedGraph<GraphNode, GraphEdge>,
) {
    branch_state_for_piece(config, previous_state)
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
            let board_too_high = !state_after_place.game.board.is_line_empty(4);
            let can_perfect_clear = state_after_place.game.board.can_perfect_clear();
            let out_of_moves = state_after_place.moves_remaining == 0 && !can_perfect_clear;

            let is_valid = !board_too_high && !out_of_moves;

            let graph_node = GraphNode {
                board: state_after_place.game.board,
                is_valid,
                moves_remaining: state_after_place.moves_remaining,
            };

            if let Some(node_idx) = node_graph.get_node_index(graph_node) {
                node_graph
                    .graph
                    .add_edge(previous_node_idx, node_idx, GraphEdge { piece_kind });
                return;
            }

            let Ok(node_idx) = node_graph.add_node(graph_node) else {
                return;
            };

            if can_perfect_clear {
                println!("found a perfect clear solution");
            }

            if can_perfect_clear || board_too_high || out_of_moves {
                return;
            }

            generate_next_states(config, &state_after_place, node_idx, node_graph);
        });
}

const NEXT_PROBABILITY: f32 = 1.0 / 7.0;

fn branch_state_for_piece(config: &Config, state: &State) -> Vec<State> {
    if state.game.piece.is_some() {
        return vec![state.clone()];
    }
    if let Ok(state_after_consume_queue) = state.reduce(config, &Action::ConsumeQueue) {
        return vec![state_after_consume_queue];
    }
    PIECE_KINDS
        .iter()
        .filter_map(|&kind| {
            // Assume all next pieces are equally likely for now.
            // TODO Calculate next probabilities.
            let guess_probability = NEXT_PROBABILITY;
            match state.reduce(config, &Action::WithNextPiece { kind }) {
                Ok(state) => Some(state),
                Err(_) => None,
            }
        })
        .collect()
}

fn branch_game_on_hold(config: &Config, game: &Game) -> Vec<Game> {
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

fn branch_game_to_placable_pieces(config: &Config, game: &Game) -> Vec<Game> {
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

    config
        .possible_moves()
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
        });
}

#[cfg(test)]
mod tests {
    use crate::piece::PieceKind;

    use super::*;

    const CONFIG: Config = Config::default();

    mod branch_game_to_placable_pieces {
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
        fn test() {
            let state = State {
                game: Game {
                    piece: Some(Piece::spawn(&CONFIG, &PieceKind::I)),
                    queue: PIECE_KINDS.map(|kind| Some(kind)),
                    ..Game::initial()
                },
                ..State::initial()
            };
            let results = get_perfect_clear_paths(&CONFIG, &state);
            // for result in results {
            //     println!("{:?}", result);
            // }
        }
    }
}
