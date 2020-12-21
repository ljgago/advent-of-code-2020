// Advent of Code 2020 - Day 11

use aoc2020::input::read_by_line;

mod part_one;
mod part_two;

fn main() {
    let data: Vec<String> = read_by_line("./resources/input.txt");

    let count = part_one::count_stabilized_seats(&data).unwrap();
    println!("--- Part One ---");
    println!("Seats occupied: {}", count);

    let count = part_two::count_stabilized_seats(&data).unwrap();
    println!("--- Part Two ---");
    println!("Seats occupied: {}", count);
}
