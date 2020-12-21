// Advent of Code 2020 - Day 5

use aoc2020::input::read_by_line;

mod common;
use crate::common::*;

mod part_one;
mod part_two;

fn main() {
    let data: Vec<Seat> = read_by_line("./resources/input.txt");

    let highest_id = part_one::highest_seat_id(&data).unwrap();
    println!("--- Part One ---");
    println!("Highest seat ID: {}", highest_id);

    let my_id = part_two::get_seat_id(&data).unwrap();
    println!("--- Part Two ---");
    println!("My seat ID: {}", my_id);
}
