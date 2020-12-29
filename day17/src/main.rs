// Advent of Code 2020 - Day 17

use aoc2020::input::read_by_line;

mod part_one;
mod part_two;

fn main() {
    let plane: Vec<String> = read_by_line("./resources/input.txt");

    let result = part_one::active_cubes_after_sixth_cycles(&plane).unwrap();
    println!("--- Part One ---");
    println!("Actve cube after the sixth cycle: {}", result);

    let result = part_two::active_cubes_after_sixth_cycles(&plane).unwrap();
    println!("--- Part Two ---");
    println!("Actve cube after the sixth cycle: {}", result);
}
