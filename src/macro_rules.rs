macro_rules! get_coord {
    ($num: expr, $max_num: expr) => {
        ($num as usize / $max_num, $num as usize % $max_num)
    }
}

pub(crate) use {get_coord};