use std::collections::HashMap;

use crate::config::{srs, Config, RotationSystem};
use crate::game::{Action, Game};
use crate::piece::Piece;
use crate::rotation::Orientation;
use crate::state::State;
use crate::utils::point::ISizePoint;
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
