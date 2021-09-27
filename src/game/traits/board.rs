use crate::triqui::Triqui;

pub trait Board {
    fn get_winner(&self) -> char;
    fn find_winner(&self, symbol: char) -> bool;
}

impl Board for Triqui {
    fn get_winner(&self) -> char {
        let played_symbols: Vec<char> = vec![self.maximizer, self.minimizer];
        for symbol in played_symbols {
            if self.find_winner(symbol) {
                return symbol;
            }
        }
        self.default_symbol
    }

    fn find_winner(&self, symbol: char) -> bool {
        for i in 0..self.size {
            if (self.board[i][0] == symbol && self.board[i][1] == symbol && self.board[i][2] == symbol)
                || (self.board[0][i] == symbol && self.board[1][i] == symbol && self.board[2][i] == symbol) {
                    return true;
                }
        }
        (self.board[0][0] == symbol && self.board[1][1] == symbol && self.board[2][2] == symbol)
        || (self.board[0][2] == symbol && self.board[1][1] == symbol && self.board[2][0] == symbol)
    }
}
