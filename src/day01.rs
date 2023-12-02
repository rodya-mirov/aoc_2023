const INPUT_FILE: &'static str = "input/01.txt";

pub fn a() -> String {
    let input = std::fs::read_to_string(INPUT_FILE).expect("Input should exist");
    a_with_multi_input(&input).to_string()
}

fn a_with_multi_input(input: &str) -> u64 {
    input.lines().map(|line| a_with_input(line)).sum()
}

fn a_with_input(input: &str) -> u64 {
    let mut first_char = '0';
    let mut last_char = '0';
    let mut set = false;

    for c in input.chars().filter(|c| c.is_digit(10)) {
        if !set {
            set = true;
            first_char = c;
        }

        last_char = c;
    }

    assert!(set, "Should have found a character");

    char_to_int(first_char) * 10 + char_to_int(last_char)
}

fn char_to_int(c: char) -> u64 {
    (c as u64) - ('0' as u64)
}

pub fn b() -> String {
    let input = std::fs::read_to_string(INPUT_FILE).expect("Input should exist");
    b_with_multi_input(&input).to_string()
}

fn b_with_multi_input(input: &str) -> u64 {
    input.lines().map(|line| b_with_input(line)).sum()
}

fn b_with_input(input: &str) -> u64 {
    let chars: Vec<char> = input.chars().collect();
    let mut first_char = 0;
    let mut last_char = 0;
    let mut set = false;

    fn get_digit(ind: usize, chars: &[char]) -> Option<u64> {
        if ind > chars.len() {
            return None;
        }

        let c = chars[ind];

        if c.is_digit(10) {
            return Some(char_to_int(c));
        }

        for (val, token) in [
            (1, "one"),
            (2, "two"),
            (3, "three"),
            (4, "four"),
            (5, "five"),
            (6, "six"),
            (7, "seven"),
            (8, "eight"),
            (9, "nine"),
        ] {
            let token_end = ind + token.len();
            if token_end > chars.len() {
                continue;
            }

            let token_chars: Vec<char> = token.chars().collect();

            let other_substr = &chars[ind..token_end];

            if &token_chars == other_substr {
                return Some(val);
            }
        }

        None
    }

    for ind in 0..chars.len() {
        if let Some(digit) = get_digit(ind, &chars) {
            if !set {
                set = true;
                first_char = digit;
            }

            last_char = digit;
        }
    }

    assert!(set, "Should have found a character");

    first_char * 10 + last_char
}

#[cfg(test)]
mod tests {
    use super::*;

    fn a_test(input: &str, exp: u64) {
        let act = a_with_input(input);
        assert_eq!(act, exp);
    }

    #[test]
    fn a_samples() {
        a_test("1abc2", 12);
        a_test("pqr3stu8vwx", 38);
        a_test("a1b2c3d4e5f", 15);
        a_test("treb7uchet", 77);
    }

    fn b_test(input: &str, exp: u64) {
        let act = b_with_input(input);
        assert_eq!(act, exp);
    }

    #[test]
    fn b_samples() {
        b_test("two1nine", 29);
        b_test("eightwothree", 83);
        b_test("abcone2threexyz", 13);
        b_test("xtwone3four", 24);
        b_test("4nineeightseven2", 42);
        b_test("zoneight234", 14);
        b_test("7pqrstsixteen", 76);
    }
}
