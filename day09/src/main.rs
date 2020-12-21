// Advent of Code 2020 - Day 9

use aoc2020::input::read_by_line;

mod part_one;
mod part_two;

fn main() {
    let data: Vec<usize> = read_by_line("./resources/input.txt");

    let num1 = part_one::num_break_xmas(&data, 25).unwrap();
    // Optimized version
    let num2 = part_one::num_break_xmas_opt(&data, 25).unwrap();
    println!("--- Part One ---");
    println!("It is not a valid number: {}", num1);
    println!("It is not a valid number: {} (optimized)", num2);

    let num = part_two::encryption_weakness(&data, num2).unwrap();
    println!("--- Part Two ---");
    println!("Encryption weakness: {}", num);
}
