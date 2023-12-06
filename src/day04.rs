use nom::bytes::complete::tag;
use nom::character::complete::{digit1, space1};
use nom::combinator::{eof, map};
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::IResult;
use std::collections::HashMap;

const INPUT_FILE: &'static str = "input/04.txt";

pub fn a() -> String {
    let input = std::fs::read_to_string(INPUT_FILE).expect("Input should exist");
    a_with_input(&input).to_string()
}

fn a_with_input(input: &str) -> u64 {
    let cards: Vec<GameCard> = input.lines().map(parse_line).collect();

    let mut out = 0;

    for card in cards {
        out += card.score();
    }

    out
}

pub fn b() -> String {
    let input = std::fs::read_to_string(INPUT_FILE).expect("Input should exist");
    b_with_input(&input).to_string()
}

fn b_with_input(input: &str) -> usize {
    let cards: Vec<GameCard> = input.lines().map(parse_line).collect();

    let mut out = 0;

    let mut value_cache = HashMap::new();

    for i in (0..cards.len()).rev() {
        let card = &cards[i];
        let num_wins = card.num_wins() as usize;

        // you always keep the card itself; plus anything you win (transitively)
        let mut card_value = 1;

        for j in (i + 1)..=(i + num_wins) {
            card_value += value_cache
                .get(&j)
                .expect("Iteration order should guarantee the value cache is populated");
        }

        value_cache.insert(i, card_value);
        out += card_value;
    }

    out
}

fn parse_line(input: &str) -> GameCard {
    fn parse_helper(input: &str) -> IResult<&str, GameCard> {
        let mut parse_num = map(digit1, |d: &str| d.parse::<u64>().unwrap());

        let (input, _) = tuple((tag("Card"), space1))(input)?;
        let (input, id) = parse_num(input)?;
        let (input, _) = tuple((tag(":"), space1))(input)?;
        let (input, winning_numbers) = separated_list1(space1, &mut parse_num)(input)?;
        let (input, _) = tuple((space1, tag("|"), space1))(input)?;
        let (input, actual_numbers) = separated_list1(space1, &mut parse_num)(input)?;

        let (_, _) = eof(input)?;

        Ok((
            "",
            GameCard {
                id,
                winning_numbers,
                actual_numbers,
            },
        ))
    }

    let (_, out) = parse_helper(input).unwrap();
    out
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct GameCard {
    id: u64,
    winning_numbers: Vec<u64>,
    actual_numbers: Vec<u64>,
}

impl GameCard {
    fn num_wins(&self) -> u64 {
        let mut num_wins: u64 = 0;
        for a in self.actual_numbers.iter() {
            if self.winning_numbers.contains(a) {
                num_wins += 1;
            }
        }
        num_wins
    }

    fn score(&self) -> u64 {
        get_score(self.num_wins())
    }
}

fn get_score(num_wins: u64) -> u64 {
    if num_wins == 0 {
        0
    } else {
        1 << (num_wins - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_A: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn sample_a() {
        assert_eq!(a_with_input(SAMPLE_A), 13);
    }

    #[test]
    fn sample_b() {
        assert_eq!(b_with_input(SAMPLE_A), 30);
    }
}
