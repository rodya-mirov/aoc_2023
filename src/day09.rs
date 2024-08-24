const INPUT_FILE: &str = "input/09.txt";

pub fn a() -> String {
    let input = std::fs::read_to_string(INPUT_FILE).expect("Input should exist");
    a_with_input(&input).to_string()
}

fn a_with_input(input: &str) -> i64 {
    input
        .lines()
        .map(parse_line)
        .map(|v| extrapolate_next(&v))
        .sum()
}

fn parse_line(line: &str) -> Vec<i64> {
    line.split_ascii_whitespace()
        .map(|n| n.parse::<i64>().unwrap())
        .collect()
}

fn extrapolate_next(nums: &[i64]) -> i64 {
    if nums.iter().copied().all(|z| z == 0) {
        return 0;
    }

    let diffs: Vec<i64> = nums.windows(2).map(|arr| arr[1] - arr[0]).collect();
    let next_diff = extrapolate_next(&diffs);

    nums[nums.len() - 1] + next_diff
}

pub fn b() -> String {
    let input = std::fs::read_to_string(INPUT_FILE).expect("Input should exist");
    b_with_input(&input).to_string()
}

fn b_with_input(input: &str) -> i64 {
    input
        .lines()
        .map(parse_line)
        .map(|v| extrapolate_prev(&v))
        .sum()
}

fn extrapolate_prev(nums: &[i64]) -> i64 {
    if nums.iter().copied().all(|z| z == 0) {
        return 0;
    }

    let diffs: Vec<i64> = nums.windows(2).map(|arr| arr[1] - arr[0]).collect();
    let next_diff = extrapolate_prev(&diffs);

    nums[0] - next_diff
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn sample_a() {
        assert_eq!(a_with_input(SAMPLE), 114);
    }

    #[test]
    fn sample_b() {
        assert_eq!(b_with_input(SAMPLE), 2);
    }
}
