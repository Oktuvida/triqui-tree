macro_rules! square2row {
    ($square: expr, $number_of_squares:expr) => {
        $square  as usize / $number_of_squares
    }
}
macro_rules! square2col {
    ($square: expr, $number_of_squares:expr) => {
        $square as usize % $number_of_squares 
    }
}

pub(crate) use {square2row, square2col};