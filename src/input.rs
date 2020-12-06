// input.rs

use std::str::FromStr;

pub fn read_by_line<T>(filename: &str) -> Vec<T>
where
    T: FromStr,
{
    std::fs::read_to_string(filename)
        .expect("file not found!")
        .lines()
        .flat_map(|x| x.parse())
        .collect()
}
