// Advent of Code 2020 - Day 16

mod common;
use crate::common::{read_custom, Document};

mod part_one;
mod part_two;

fn main() {
    let document: Document = read_custom("./resources/input.txt").unwrap();

    let result: usize = part_one::sum_error_rate(&document).unwrap();
    println!("--- Part One ---");
    println!("Your ticket scanning error rate: {}", result);

    let result: usize = part_two::mul_found_fields(&document).unwrap();
    println!("--- Part Two ---");
    println!("Multiply departure ticket items: {}", result);
}
