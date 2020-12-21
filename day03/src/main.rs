// Advent of Code 2020 - Day 3

use aoc2020::input::read_by_line;

mod part_one;
mod part_two;

fn main() {
    let data: Vec<String> = read_by_line("./resources/input.txt");

    let count = part_one::count_trajectory_trees(&data, &(3, 1)).unwrap();
    println!("--- Part One ---");
    println!("Trees: {}", count);

    let slopes: Vec<(usize, usize)> = vec![(1,1), (3,1), (5, 1), (7, 1), (1, 2)];
    let multiply = part_two::multiply_trajectory_trees(&data, &slopes).unwrap();
    println!("--- Part Two ---");
    println!("Multiply slopes: {}", multiply);
}
