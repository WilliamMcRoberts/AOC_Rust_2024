use itertools::Itertools;
pub fn process_day_1_puzzle_1(input: &str) -> i32 {
    let mut left = vec![];
    let mut right = vec![];

    for line in input.lines() {
        let mut nums = line.split_whitespace();
        left.push(nums.next().unwrap().parse::<i32>().unwrap());
        right.push(nums.next().unwrap().parse::<i32>().unwrap());
    }

    std::iter::zip(left.into_iter().sorted(), right.into_iter().sorted())
        .map(|(l, r)| (l - r).abs())
        .sum()
}

#[test]
fn test_day_1puzzle_1() {
    let test_input = r#" 3   4
        4   3
        2   5
        1   3
        3   9
        3   3 "#;

    assert_eq!(11, process_day_1_puzzle_1(test_input));
}
