use crate::macro_rules::*;
use crate::triqui::Triqui;

pub trait Strategy {
    type Player;
    type Movement;
    type Board;
    fn available_movements(&self) -> Vec<Self::Movement>;
    fn play(&mut self, mv: &Self::Movement, maximizer: bool) -> i16;
    fn set_play(&mut self, mv: &Self::Movement, symbol: char) -> i16;
    fn redo(&mut self, mv: &Self::Movement);
    fn invalid_movement(&self) -> Self::Movement;
}

impl Strategy for Triqui {
    type Player = char;
    type Movement = usize;
    type Board = Vec<Vec<char>>;

    fn available_movements(&self) -> Vec<Self::Movement> {
        let mut movements: Vec<usize> = vec![];
        self.board.iter().enumerate().for_each(|(i, p)| {
            p.iter().enumerate().for_each(|(j, &q)| {
                if q == self.default_symbol {
                    movements.push(i * self.size + j);
                }
            })
        });
        movements
    }
    fn play(&mut self, &mv: &Self::Movement, maximizer: bool) -> i16 {
        match maximizer {
            true => self.set_play(&mv, self.maximizer),
            _ => self.set_play(&mv, self.minimizer),
        }
    }

    fn set_play(&mut self, &mv: &Self::Movement, symbol: char) -> i16 {
        let (row, col) = get_coord!(mv, self.size);
        let val = self.symbol_value(symbol);
        self.board[row][col] = symbol;
        self.rows_container[row] += val;
        self.cols_container[col] += val;
        if row == col {
            self.diagonal_container += val;
            if (self.size / 2) as usize == row {
                self.opposite_diagonal_container += val;
            }
        } else if self.size - row - 1 == col {
            self.opposite_diagonal_container += val;
        }
        let val = self.size as i16 * val;
        if (val < 0
            && (self.rows_container[row] <= val
                || self.cols_container[col] <= val
                || self.diagonal_container <= val
                || self.opposite_diagonal_container <= val))
            || (val > 0
                && (self.rows_container[row] >= val
                    || self.cols_container[col] >= val
                    || self.diagonal_container >= val
                    || self.opposite_diagonal_container >= val))
        {
            return val;
        }
        0
    }

    fn redo(&mut self, &mv: &Self::Movement) {
        let (row, col) = get_coord!(mv, self.size);
        let val = self.symbol_value(self.board[row][col]);
        self.board[row][col] = self.default_symbol;
        self.rows_container[row] -= val;
        self.cols_container[col] -= val;
        if row == col {
            self.diagonal_container -= val;
            if (self.size / 2) as usize == row {
                self.opposite_diagonal_container -= val;
            }
        } else if self.size - row - 1 == col {
            self.opposite_diagonal_container -= val;
        }
    }

    fn invalid_movement(&self) -> Self::Movement {
        self.size * self.size + 1
    }
}
