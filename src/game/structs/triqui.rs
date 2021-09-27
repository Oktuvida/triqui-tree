pub struct Triqui {
    pub size: usize,
    pub board: Vec<Vec<char>>,
    pub maximizer: char,
    pub minimizer: char,
    pub default_symbol: char,
}

impl Triqui {
    pub fn create_game(
        maximizer: Option<char>,
        minimizer: Option<char>,
        default_symbol: Option<char>,
    ) -> Self {
        let size: usize = 3;
        let board: Vec<Vec<char>> = vec![vec![default_symbol.unwrap_or(' '); size]; size];
        Self {
            size,
            board,
            maximizer: maximizer.unwrap_or('O'),
            minimizer: minimizer.unwrap_or('X'),
            default_symbol: default_symbol.unwrap_or(' '),
        }
    }
}
