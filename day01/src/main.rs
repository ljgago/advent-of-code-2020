// Advent of Code 2020 - Day 1

use std::collections::HashSet;

use aoc2020::input::read_by_line;

mod part_one;
mod part_two;

fn main() {
    let data: Vec<i64> = read_by_line("./resources/input.txt");

    let (sum, multiply) = part_one::check_expense_report(&data);
    println!("--- Part One ---");
    println!("Sum: {}\nMultiply: {}", sum, multiply);

    let (sum, multiply) = part_two::check_expense_report(&data);
    println!("--- Part Two ---");
    println!("Sum: {}\nMultiply: {}", sum, multiply);

    let data_hash: HashSet<i64> = data.clone().into_iter().collect();

    let multiply = part_one::check_expense_report_op(data_hash.clone(), 2020).unwrap();
    println!("--- Part One Optimized ---");
    println!("Multiply: {}", multiply);

    let multiply = part_two::check_expense_report_op(data_hash.clone(), 2020).unwrap();
    println!("--- Part Two Optimized ---");
    println!("Multiply: {}", multiply);
}
