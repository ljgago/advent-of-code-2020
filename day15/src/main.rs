// Advent of Code 2020 - Day 15

use aoc2020::input::read_by_custom_split;

mod part_one;
mod part_two;

fn main() {
    let data: Vec<usize> = read_by_custom_split("./resources/input.txt", ",");

    let result = part_one::number_spoken(&data, 2020).unwrap();
    println!("--- Part One ---");
    println!("2020th spoken number: {}", result);

    let result = part_two::number_spoken(&data, 30000000).unwrap();
    println!("--- Part Two ---");
    println!("30000000th spoken number: {}", result);
}
