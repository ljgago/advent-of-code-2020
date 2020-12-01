// Advent of Code 2020 - Day 1

use aoc2020;

mod part_one;
mod part_two;

fn main() -> std::io::Result<()> {
    let data: Vec<usize> = aoc2020::input::read("./resources/input.txt")?.collect();

    let (sum, multiply) = part_one::check_expense_report(&data);
    println!("--- Part One ---");
    println!("Sum: {}\nMultiply: {}\n", sum, multiply);

    let (sum, multiply) = part_two::check_expense_report(&data);
    println!("--- Part Two ---");
    println!("Sum: {}\nMultiply: {}\n", sum, multiply);

    Ok(())
}
