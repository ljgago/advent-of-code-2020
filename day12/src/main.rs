// Advent of Code 2020 - Day 12

use aoc2020::input::read_by_line;

mod common;
use crate::common::Instruction;

mod part_one;
mod part_two;

fn main() {
    let instructions: Vec<Instruction> = read_by_line("./resources/input.txt");

    let distance = part_one::get_distance(&instructions).unwrap();
    println!("--- Part One ---");
    println!("Distance: {}", distance);

    let distance = part_two::get_distance(&instructions).unwrap();
    println!("--- Part Two ---");
    println!("Distance: {}", distance);
}
