// Advent of Code 2020 - Day 18

use aoc2020::input::read_by_line;

mod part_one;
mod part_two;

fn main() {
    let expressions: Vec<String> = read_by_line("./resources/input.txt");

    let result = part_one::sum_operations(&expressions).unwrap();
    println!("--- Part One ---");
    println!("Sum of expressions: {}", result);

    let result = part_two::sum_operations(&expressions).unwrap();
    println!("--- Part Two ---");
    println!("Sum of expressions: {}", result);
}
