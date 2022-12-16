use wasm_bindgen::prelude::wasm_bindgen;

use crate::state::State;

#[wasm_bindgen]
struct Solver {
    current_state: State,
}

#[wasm_bindgen]
impl Solver {
    pub fn new() -> Solver {
        Solver {
            current_state: State::initial(),
        }
    }
}
