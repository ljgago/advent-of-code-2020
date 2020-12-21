// Advent of Code 2020 - Day 10

use aoc2020::input::read_by_line;

mod part_one;
mod part_two;

fn main() {
    let adapters: Vec<usize> = read_by_line("./resources/input.txt");

    let result = part_one::jolt_differences_mul(&adapters).unwrap();
    println!("--- Part One ---");
    println!("1-jolt differences * 3-jolt differences = {}", result);

    let result = part_two::distinct_ways_arrange(&adapters).unwrap();
    println!("--- Part Two ---");
    println!("Total number of distinct ways: {}", result);
}
