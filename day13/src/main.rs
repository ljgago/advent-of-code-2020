// Advent of Code 2020 - Day 13

mod part_one;
mod part_two;

fn main() {
    let notes: String = std::fs::read_to_string("./resources/input.txt").unwrap();

    let result = part_one::earliest_bus_times_minutes(&notes).unwrap();
    println!("--- Part One ---");
    println!("ID * min_to_wait = {}", result);

    let result = part_two::earliest_timestamp_chinese(&notes).unwrap();
    println!("--- Part Two ---");
    println!("Timestamp = {}", result);
}
