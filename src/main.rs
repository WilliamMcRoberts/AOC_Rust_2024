use std::fs::read_to_string;
mod day_1;
use day_1::puzzle_1::process_day_1_puzzle_1;

fn main() {
    let lines = read_to_string("src/day_1/day_1_puzzle_1_input").expect("No File Found");
    let day_1_puzzle_1_result = process_day_1_puzzle_1(&lines);
    println!("Day 1 Puzzle 1 Result: {}", day_1_puzzle_1_result);
}
