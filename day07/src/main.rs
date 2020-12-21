// Advent of Code 2020 - Day 7

use aoc2020::input::read_by_line;

mod common;
mod part_one;
mod part_two;

fn main() {
    let data: Vec<String> = read_by_line("./resources/input.txt");

    let bags_map = common::hash_map_of_bags(&data);
    let count = part_one::count_color_bags("shiny gold", &bags_map).unwrap();
    println!("--- Part One ---");
    println!("Count: {}", count);

    let count = part_two::all_individual_bags("shiny gold", &bags_map).unwrap();
    println!("--- Part Two ---");
    println!("Individual bags: {}", count);
}
