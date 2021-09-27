use wasm_bindgen::prelude::*;
use crate::triqui::Triqui;
use crate::algorithm::MiniMax;
use crate::strategy::Strategy;
use crate::board::Board;

#[wasm_bindgen]
pub struct Controller {
    triqui_tree: Triqui,
    depth: i64,
    maximizer: String,
    minimizer: String,
}

#[wasm_bindgen]
impl Controller {
    pub fn new(depth: i64) -> Self {
        Self {
            triqui_tree: Triqui::create_game(None, None, None),
            depth: depth,
            maximizer: "fa-circle".to_string(),
            minimizer: "fa-times".to_string(),
        }
    }

    pub fn get_maximizer(&self) -> String{
        format!("{}", self.maximizer).into()
    }

    pub fn get_minimizer(&self) -> String{
        format!("{}", self.minimizer).into()
    }

    pub fn get_invalid_mv(&self) -> usize{
        usize::MAX
    }

    pub fn completed_game(&self) -> bool {
        self.triqui_tree.completed_game()
    }

    pub fn get_winner(&self) -> String {
        if self.triqui_tree.is_tie() {
            return "Es empate".to_string();
        } else if self.triqui_tree.maximizer == self.triqui_tree.get_winner() {
                return "La ganadora es la máquina".to_string();
        } else {
            return "El ganador es el jugador".to_string();
        }
    }

    pub fn play_turn(&mut self, id: &str) -> usize {
        let square: usize = id.parse::<usize>().unwrap_or(usize::MAX);
        self.triqui_tree.play(&square, false);
        let movement_found = self.triqui_tree.best_movement(self.depth);
        if movement_found == self.triqui_tree.invalid_movement() {
            return usize::MAX;
        }
        self.triqui_tree.play(&movement_found, true);
        movement_found
    }
}