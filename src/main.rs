use std::fs::read_to_string;
mod day_1;
use day_1::puzzle_1::process_day_1_puzzle_1;
use day_1::puzzle_2::process_day_1_puzzle_2;

fn main() {
    let lines = read_to_string("src/day_1/day_1_input").expect("No File Found");

    /* *** Day 1 *** */
    let day_1_puzzle_1_result = process_day_1_puzzle_1(&lines);
    println!("Day 1 Puzzle 1 Result: {}", day_1_puzzle_1_result);

    let day_1_puzzle_2_result = process_day_1_puzzle_2(&lines);
    println!("Day 1 Puzzle 2 Result: {}", day_1_puzzle_2_result);
}
