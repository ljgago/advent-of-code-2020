// Advent of Code 2020 - Day 6

use aoc2020::input::read_by_custom_split;

mod common;
use common::Form;

mod part_one;
mod part_two;

fn main() {
    let data: Vec<Form> = read_by_custom_split("./resources/input.txt", "\n\n");

    let total_counts = part_one::sum_of_answers(&data).unwrap();
    println!("--- Part One ---");
    println!("Total counts: {}", total_counts);

    let total_counts = part_two::sum_of_answers(&data).unwrap();
    println!("--- Part Two ---");
    println!("Total counts: {}", total_counts);
}
