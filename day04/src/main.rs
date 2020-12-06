// Advent of Code 2020 - Day 4

mod common;
use common::*;

mod part_one;
mod part_two;

fn main() -> Result<(), std::io::Error> {
    let data: Vec<Passport> = read("./resources/input.txt");

    let count = part_one::count_valid_passport(&data).unwrap();
    println!("--- Part One ---");
    println!("Valid Passport: {}", count);

    let count = part_two::count_valid_passport(&data).unwrap();
    println!("--- Part Two ---");
    println!("Valid Passport: {}", count);

    Ok(())
}
