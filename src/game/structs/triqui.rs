pub struct Triqui {
    pub size: usize,
    pub board: Vec<Vec<char>>,
    pub rows_container: Vec<i16>,
    pub cols_container: Vec<i16>,
    pub diagonal_container: i16,
    pub opposite_diagonal_container: i16,
    pub maximizer: char,
    pub minimizer: char,
    pub default_symbol: char,
}

impl Triqui {
    pub fn create_game(
        size: Option<usize>,
        maximizer: Option<char>,
        minimizer: Option<char>,
        default_symbol: Option<char>,
    ) -> Self {
        let size: usize = size.unwrap_or(3);
        let maximizer: char = maximizer.unwrap_or('O');
        let minimizer: char = minimizer.unwrap_or('X');
        let default_symbol: char = default_symbol.unwrap_or(' ');
        let board: Vec<Vec<char>> = vec![vec![default_symbol; size]; size];
        let rows_container: Vec<i16> = vec![0; size];
        let cols_container: Vec<i16> = vec![0; size];
        let diagonal_container: i16 = 0;
        let opposite_diagonal_container: i16 = 0;
        Self {
            size,
            board,
            rows_container,
            cols_container,
            diagonal_container,
            opposite_diagonal_container,
            maximizer,
            minimizer,
            default_symbol,
        }
    }

    pub fn symbol_value(&self, symbol: char) -> i16 {
        if symbol == self.maximizer {
            return 1;
        }
        if symbol == self.minimizer {
            return -1;
        }
        0
    }
}