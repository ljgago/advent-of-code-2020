// input.rs

use std::str::FromStr;

pub fn read_by_line<T>(filename: &str) -> Vec<T>
where
    T: FromStr,
{
    std::fs::read_to_string(filename)
        .expect("file not found!")
        .lines()
        .flat_map(|x| x.trim().parse())
        .collect()
}

pub fn read_by_custom_split<T>(filename: &str, pat: &str) -> Vec<T>
where
    T: FromStr,
{
    std::fs::read_to_string(filename)
        .expect("file not found!")
        .split(pat)
        .flat_map(|x| x.trim().parse())
        .collect()
}
