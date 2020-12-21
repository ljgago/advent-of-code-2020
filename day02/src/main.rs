// Advent of Code 2020 - Day 2

use aoc2020::input::read_by_line;

mod common;
use common::PasswordPolicy;

mod part_one;
mod part_two;

fn main() {
    let data = read_by_line::<PasswordPolicy>("./resources/input.txt");

    let count = part_one::count_valid_password(&data).unwrap();
    println!("--- Part One ---");
    println!("Valid passwords: {}", count);

    let count = part_two::count_valid_password(&data).unwrap();
    println!("--- Part Two ---");
    println!("Valid passwords: {}", count);
}
