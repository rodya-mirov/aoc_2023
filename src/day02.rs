use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::IResult;

const INPUT_FILE: &'static str = "input/02.txt";

pub fn a() -> String {
    let input = std::fs::read_to_string(INPUT_FILE).expect("Input should exist");
    a_with_input(&input).to_string()
}

fn a_with_input(input: &str) -> usize {
    input
        .lines()
        .map(a_one_game)
        .filter(|(_id, is_valid)| *is_valid)
        .map(|(id, _)| id)
        .sum()
}

/// Returns (id, is_valid) for the parsed game
fn a_one_game(input: &str) -> (usize, bool) {
    let game_record = parse_game(input);
    let is_valid = game_record
        .pulls
        .iter()
        .all(|p| p.num_red <= 12 && p.num_green <= 13 && p.num_blue <= 14);
    (game_record.id, is_valid)
}

pub fn b() -> String {
    let input = std::fs::read_to_string(INPUT_FILE).expect("Input should exist");
    b_with_input(&input).to_string()
}

fn b_with_input(input: &str) -> usize {
    input.lines().map(game_power).sum()
}

fn game_power(input: &str) -> usize {
    let mut min_red = 0;
    let mut min_blue = 0;
    let mut min_green = 0;

    for pull in parse_game(input).pulls {
        min_red = min_red.max(pull.num_red);
        min_blue = min_blue.max(pull.num_blue);
        min_green = min_green.max(pull.num_green);
    }

    min_red * min_blue * min_green
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct GameRecord {
    id: usize,
    pulls: Vec<Pull>,
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Pull {
    num_red: usize,
    num_blue: usize,
    num_green: usize,
}

fn parse_game(input: &str) -> GameRecord {
    enum Color {
        Blue,
        Red,
        Green,
    }

    /// Parses things like "green"
    fn parse_color(input: &str) -> IResult<&str, Color> {
        alt((
            map(tag("blue"), |_| Color::Blue),
            map(tag("green"), |_| Color::Green),
            map(tag("red"), |_| Color::Red),
        ))(input)
    }

    /// Parses things like "3 red"
    fn parse_color_num(input: &str) -> IResult<&str, (usize, Color)> {
        map(
            tuple((digit1, tag(" "), parse_color)),
            |(digits, _, color)| (digits.parse::<usize>().expect("digits should parse"), color),
        )(input)
    }

    /// Parses things like "3 blue, 4 red"
    fn parse_pull(input: &str) -> IResult<&str, Pull> {
        let (input, colors) = separated_list1(tag(", "), parse_color_num)(input)?;

        let mut num_red = 0;
        let mut num_blue = 0;
        let mut num_green = 0;

        for (amt, color) in colors {
            match color {
                Color::Blue => {
                    num_blue = amt;
                }
                Color::Green => {
                    num_green = amt;
                }
                Color::Red => {
                    num_red = amt;
                }
            }
        }

        let pull = Pull {
            num_green,
            num_red,
            num_blue,
        };

        Ok((input, pull))
    }

    /// Parses the whole line
    fn parse_helper(input: &str) -> IResult<&str, GameRecord> {
        let (input, _) = tag("Game ")(input)?;
        let (input, id_digits) = digit1(input)?;
        let id: usize = id_digits.parse().expect("Digits should be a usize");
        let (input, _) = tag(": ")(input)?;
        let (input, pulls) = separated_list1(tag("; "), parse_pull)(input)?;
        Ok((input, GameRecord { id, pulls }))
    }

    let parsed = parse_helper(input).expect("Input should be parseable");
    assert_eq!(parsed.0, "");
    parsed.1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let exp = GameRecord {
            id: 1,
            pulls: vec![
                Pull {
                    num_blue: 3,
                    num_red: 4,
                    num_green: 0,
                },
                Pull {
                    num_red: 1,
                    num_green: 2,
                    num_blue: 6,
                },
                Pull {
                    num_green: 2,
                    num_red: 0,
                    num_blue: 0,
                },
            ],
        };

        assert_eq!(parse_game(input), exp);
    }

    fn game_test(input: &str, exp: (usize, bool)) {
        let act = a_one_game(input);
        assert_eq!(act.0, exp.0, "ID should parse");
        assert_eq!(act.1, exp.1, "Validity should check correctly");
    }

    #[test]
    fn a_samples() {
        let fixtures = [
            (
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
                1,
                true,
            ),
            (
                "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
                2,
                true,
            ),
            (
                "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
                3,
                false,
            ),
            (
                "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
                4,
                false,
            ),
            (
                "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
                5,
                true,
            ),
        ];

        for (input, id, valid) in fixtures {
            game_test(input, (id, valid))
        }
    }

    fn game_power_test(input: &str, exp: usize) {
        let act = game_power(input);
        assert_eq!(act, exp);
    }

    #[test]
    fn b_samples() {
        let fixtures = [
            ("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", 48),
            (
                "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
                12,
            ),
            (
                "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
                1560,
            ),
            (
                "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
                630,
            ),
            ("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", 36),
        ];

        for (input, exp) in fixtures {
            game_power_test(input, exp);
        }
    }
}
