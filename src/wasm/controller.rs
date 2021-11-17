extern crate console_error_panic_hook;
use crate::algorithm::MiniMax;
use crate::strategy::Strategy;
use crate::triqui::Triqui;
use std::panic;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Controller {
    triqui_tree: Triqui,
    depth: i64,
}

#[wasm_bindgen]
pub struct Board {
    movement_found: usize,
    board_state: i16,
}

#[wasm_bindgen]
impl Board {
    pub fn get_movement_found(&self) -> usize {
        self.movement_found
    }

    pub fn get_board_state(&self) -> i16 {
        self.board_state
    }
}

#[wasm_bindgen]
impl Controller {
    pub fn new(depth: i64, size: usize, maximizer: char, minimizer: char) -> Self {
        Self {
            triqui_tree: Triqui::create_game(Some(size), Some(maximizer), Some(minimizer), None),
            depth: depth,
        }
    }

    pub fn get_invalid_mv(&self) -> usize {
        usize::MAX
    }

    pub fn get_min_value(&self) -> i16 {
        self.triqui_tree.size as i16 * self.triqui_tree.symbol_value(self.triqui_tree.minimizer)
    }

    pub fn get_max_value(&self) -> i16 {
        self.triqui_tree.size as i16 * self.triqui_tree.symbol_value(self.triqui_tree.maximizer)
    }

    pub fn play_turn(&mut self, id: usize) -> Board {
        panic::set_hook(Box::new(console_error_panic_hook::hook));
        self.triqui_tree.play(&id, false);
        let movement_found = self.triqui_tree.best_movement(&self.depth);
        if movement_found == self.triqui_tree.invalid_movement() {
            return Board {
                movement_found: usize::MAX,
                board_state: 0,
            };
        }
        Board {
            movement_found: movement_found,
            board_state: self.triqui_tree.play(&movement_found, true),
        }
    }
}
