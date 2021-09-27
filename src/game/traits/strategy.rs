use crate::triqui::Triqui;
use crate::board::Board;
use crate::macro_rules::*;

pub trait Strategy {
    type Player;
    type Movement;
    type Board;
    fn evaluate(&self) -> f64;
    fn is_tie(&self) -> bool;
    fn completed_game(&self) -> bool;
    fn available_movements(&self) -> Vec<Self::Movement>;
    fn play(&mut self, mv: &Self::Movement, maximizer: bool);
    fn redo(&mut self, mv: &Self::Movement);
    fn invalid_movement(&self) -> Self::Movement;
}

impl Strategy for Triqui {
    type Player = char;
    type Movement = usize;
    type Board = Vec<Vec<char>>;

    fn evaluate(&self) -> f64 {
        if self.is_tie() {
            return 0.;
        }
        if self.get_winner() == self.maximizer {
            return 100.;
        }
        return -100.;
    }

    fn is_tie(&self) -> bool {
        let _winner = self.get_winner();
        _winner == self.default_symbol && self.available_movements().is_empty()
    }

    fn completed_game(&self) -> bool {
        let _winner = self.get_winner();
        _winner != self.default_symbol || self.available_movements().is_empty()
    }

    fn available_movements(&self) -> Vec<Self::Movement> {
        let mut movements: Vec<usize> = vec![];
        for square in 0..(self.size * self.size) as usize {
            let row = square2row!(square, self.size);
            let col = square2col!(square, self.size);
            if self.board[row][col] == self.default_symbol {
                movements.push(square);
            }
        }
        movements
    }
    fn play(&mut self, &mv: &Self::Movement, maximizer: bool) {
        let row = square2row!(mv, self.size);
        let col = square2col!(mv, self.size);
        if maximizer {
            self.board[row][col] = self.maximizer;
        } else {
            self.board[row][col] = self.minimizer;
        }
    }

    fn redo(&mut self, &mv: &Self::Movement) {
        let row = square2row!(mv, self.size);
        let col = square2col!(mv, self.size);
        self.board[row][col] = self.default_symbol;
    }

    fn invalid_movement(&self) -> Self::Movement {
        self.size * self.size + 1
    }

}