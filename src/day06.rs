use nom::bytes::complete::tag;
use nom::character::complete::{digit1, space1};
use nom::combinator::{eof, map};
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::tuple;

const INPUT_FILE: &'static str = "input/06.txt";

pub fn a() -> String {
    let input = std::fs::read_to_string(INPUT_FILE).expect("Input should exist");
    a_with_input(&input).to_string()
}

fn a_with_input(input: &str) -> usize {
    let races = parse_input_a(input);

    races.iter().map(|r| r.num_solutions()).product()
}

fn distance(race_time: u64, charge_time: u64) -> u64 {
    assert!(charge_time <= race_time);

    let speed = charge_time;
    let duration = race_time - charge_time;

    // polynomial formula: D = (T - C) * C = -C^2 + TC
    speed * duration
}

struct Race {
    time: u64,
    record: u64,
}

impl Race {
    fn num_solutions(&self) -> usize {
        (0 ..= self.time)
            .map(|charge_time| distance(self.time, charge_time))
            .filter(|total_dist| *total_dist > self.record)
            .count()
    }
}

fn parse_input_a(input: &str) -> Vec<Race> {
    fn parse_line<'a>(input: &'a str, str_tag: &str) -> IResult<&'a str, Vec<u64>> {
        let (input, _) = tuple((tag(str_tag), tag(":"), space1))(input)?;
        let (input, nums) = separated_list1(space1, map(digit1, |d: &str| d.parse::<u64>().unwrap()))(input)?;
        let (_, _) = eof(input)?;
        Ok(("", nums))
    }

    let mut line_iter = input.lines();
    let (_, times) = line_iter.next().map(|line| parse_line(line, "Time")).expect("First line should exist").expect("First line should parse");
    let (_, records) = line_iter.next().map(|line| parse_line(line, "Distance")).expect("Second line should exist").expect("Second line should parse");
    assert_eq!(line_iter.next(), None);

    assert_eq!(times.len(), records.len());

    let mut out = Vec::new();

    for i in 0 .. times.len() {
        out.push(Race {
            time: times[i],
            record: records[i]
        });
    }

    out
}

fn parse_input_b(input: &str) -> Race {
    fn parse_line<'a>(input: &'a str, str_tag: &str) -> IResult<&'a str,u64> {
        let (input, _) = tuple((tag(str_tag), tag(":"), space1))(input)?;
        let joined: String = input.chars().filter(|c| !c.is_ascii_whitespace()).collect();
        let num: u64 = joined.parse().unwrap();
        Ok(("", num))
    }

    let mut line_iter = input.lines();
    let (_, time) = line_iter.next().map(|line| parse_line(line, "Time")).expect("First line should exist").expect("First line should parse");
    let (_, record) = line_iter.next().map(|line| parse_line(line, "Distance")).expect("Second line should exist").expect("Second line should parse");
    assert_eq!(line_iter.next(), None);

    Race { time, record }
}

pub fn b() -> String {
    let input = std::fs::read_to_string(INPUT_FILE).expect("Input should exist");
    b_with_input(&input).to_string()
}

fn b_with_input(input: &str) -> usize {
    let race = parse_input_b(input);
    race.num_solutions()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_A: &'static str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn sample_a() {
        assert_eq!(a_with_input(SAMPLE_A), 288);
    }

    #[test]
    fn sample_b() {
        assert_eq!(b_with_input(SAMPLE_A), 71503);
    }
}
