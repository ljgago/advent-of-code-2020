// Advent of Code 2020 - Day 8

use aoc2020::input::read_by_line;

mod common;
use crate::common::Instruction;

mod part_one;
mod part_two;

fn main() {
    let instructions: Vec<Instruction> = read_by_line("./resources/input.txt");

    let acc = part_one::get_acc_infinite_loop(&instructions).unwrap();
    println!("--- Part One ---");
    println!("Accumulator: {}", acc);

    let (line, acc) = part_two::search_fix_operation(&instructions).unwrap();
    println!("--- Part Two ---");
    println!("line: {}, acc: {}", line, acc);
}
