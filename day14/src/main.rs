// Advent of Code 2020 - Day 14

use aoc2020::input::read_by_custom_split;

mod common;
use crate::common::Program;

mod part_one;
mod part_two;

fn main() {
    let full_program: Vec<Program> = read_by_custom_split("./resources/input.txt", "mask = ");

    let result = part_one::run_and_sum(&full_program).unwrap();
    println!("--- Part One ---");
    println!("Sum of values: {}", result);

    let result = part_two::run_and_sum(&full_program).unwrap();
    println!("--- Part Two ---");
    println!("Sum of values: {}", result);
}
