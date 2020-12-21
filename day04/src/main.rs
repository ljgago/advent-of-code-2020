// Advent of Code 2020 - Day 4

use aoc2020::input::read_by_custom_split;

mod common;
use common::Passport;

mod part_one;
mod part_two;

fn main() {
    let data: Vec<Passport> = read_by_custom_split("./resources/input.txt", "\n\n");

    let count = part_one::count_valid_passport(&data).unwrap();
    println!("--- Part One ---");
    println!("Valid Passport: {}", count);

    let count = part_two::count_valid_passport(&data).unwrap();
    println!("--- Part Two ---");
    println!("Valid Passport: {}", count);
}
