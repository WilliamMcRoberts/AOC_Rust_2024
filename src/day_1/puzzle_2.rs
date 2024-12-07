pub fn process_day_1_puzzle_2(input: &str) -> usize {
    let (mut left, mut right) = (vec![], vec![]);
    let mut lines = input.lines();

    while let Some(line) = lines.next() {
        let mut nums = line.split_whitespace();
        left.push(nums.next().unwrap().parse::<usize>().unwrap());
        right.push(nums.next().unwrap().parse::<usize>().unwrap());
    }

    left.iter()
        .map(|nl| nl * right.iter().filter(|nr| &nl == nr).count())
        .sum()
}

#[test]
fn test_day_1_puzzle_2() {
    let test_input = r#" 3   4
        4   3
        2   5
        1   3
        3   9
        3   3 "#;

    assert_eq!(31, process_day_1_puzzle_2(test_input));
}
