use nom::bytes::complete::{is_a, tag};
use nom::character::complete::{digit1, space1};
use nom::combinator::{eof, map};
use nom::multi::separated_list1;
use nom::IResult;
use std::collections::VecDeque;
use time::Instant;

const INPUT_FILE: &'static str = "input/12.txt";

pub fn a() -> String {
    let input = std::fs::read_to_string(INPUT_FILE).expect("Input should exist");
    a_with_input(&input).to_string()
}

fn a_with_input(input: &str) -> usize {
    input.lines().map(a_line).sum()
}

fn a_line(input: &str) -> usize {
    num_arrangements(parse(input))
}

pub fn b() -> String {
    let input = std::fs::read_to_string(INPUT_FILE).expect("Input should exist");
    b_with_input(&input).to_string()
}

fn b_with_input(input: &str) -> usize {
    let start = Instant::now();
    let total_lines = input.lines().count();
    let mut total = 0;
    for (i, line) in input.lines().enumerate() {
        let line_time = Instant::now();
        total += b_line(line);
        println!(
            "Finished line {} of {total_lines} -- took {}; total elapsed {}",
            i + 1,
            line_time.elapsed(),
            start.elapsed()
        )
    }
    total
}

fn b_line(input: &str) -> usize {
    let parsed = parse(input);
    let expanded = parsed.expand();
    num_arrangements(expanded)
}

fn num_arrangements(input: ParseResult) -> usize {
    let ParseResult {
        cells,
        damaged_counts,
    } = input;

    // Start with the dumbest possible solution -- recursively try every possible assignment
    // of Broken and NotBroken
    fn count_solutions_rec(
        cells: &mut [ParsedCell],
        damaged_counts: &mut VecDeque<usize>,
        next_ind: usize,
        remaining_budget: usize,
        current_run: usize,
    ) -> usize {
        if next_ind == cells.len() {
            return if current_run > 0 {
                if damaged_counts.len() == 1 && damaged_counts.get(0) == Some(&current_run) {
                    1
                } else {
                    0
                }
            } else {
                if damaged_counts.is_empty() {
                    1
                } else {
                    0
                }
            };
        }

        if remaining_budget > cells.len() - next_ind {
            return 0;
        }

        match cells[next_ind] {
            ParsedCell::Unknown => {
                let mut total = 0;

                // try operational?
                cells[next_ind] = ParsedCell::Operational;
                total += count_solutions_rec(
                    cells,
                    damaged_counts,
                    next_ind,
                    remaining_budget,
                    current_run,
                );

                // try damaged?
                if remaining_budget > 0 && !damaged_counts.is_empty() {
                    let next_run = damaged_counts.get(0).copied().unwrap();
                    if next_run > current_run {
                        cells[next_ind] = ParsedCell::Damaged;
                        total += count_solutions_rec(
                            cells,
                            damaged_counts,
                            next_ind,
                            remaining_budget - 1,
                            current_run,
                        );
                    }
                }

                // put the array back how we found it
                cells[next_ind] = ParsedCell::Unknown;

                total
            }
            ParsedCell::Operational => {
                if current_run > 0 {
                    if damaged_counts.get(0) != Some(&current_run) {
                        return 0;
                    }

                    damaged_counts.pop_front();
                    let total = count_solutions_rec(
                        cells,
                        damaged_counts,
                        next_ind + 1,
                        remaining_budget,
                        0,
                    );
                    damaged_counts.push_front(current_run);
                    total
                } else {
                    count_solutions_rec(cells, damaged_counts, next_ind + 1, remaining_budget, 0)
                }
            }
            ParsedCell::Damaged => {
                // increment the current run and move on to the next thing
                if damaged_counts.is_empty()
                    || damaged_counts.get(0).copied().unwrap() <= current_run
                {
                    return 0;
                }

                count_solutions_rec(
                    cells,
                    damaged_counts,
                    next_ind + 1,
                    remaining_budget,
                    current_run + 1,
                )
            }
        }
    }

    let required_damage: usize = damaged_counts.iter().copied().sum();
    let known_damage = cells.iter().filter(|&&c| c == ParsedCell::Damaged).count();
    let budget = required_damage - known_damage;

    let mut dc_vd: VecDeque<usize> = damaged_counts.iter().copied().collect();

    count_solutions_rec(&mut cells.clone(), &mut dc_vd, 0, budget, 0)
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum ParsedCell {
    Unknown,
    Operational,
    Damaged,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct ParseResult {
    cells: Vec<ParsedCell>,
    damaged_counts: Vec<usize>,
}

impl ParseResult {
    fn expand(&self) -> Self {
        let mut new_cells = self.cells.clone();
        let mut new_counts = self.damaged_counts.clone();

        for _ in 0..4 {
            new_cells.push(ParsedCell::Unknown);
            for c in self.cells.iter().copied() {
                new_cells.push(c);
            }

            for i in self.damaged_counts.iter().copied() {
                new_counts.push(i);
            }
        }

        ParseResult {
            cells: new_cells,
            damaged_counts: new_counts,
        }
    }
}

fn parse(line: &str) -> ParseResult {
    fn parse_cells(input: &str) -> IResult<&str, Vec<ParsedCell>> {
        let (input, cell_chars) = is_a(".#?")(input)?;
        let cells = cell_chars
            .chars()
            .map(|c| match c {
                '.' => ParsedCell::Operational,
                '#' => ParsedCell::Damaged,
                '?' => ParsedCell::Unknown,
                _ => panic!("Bad input {c}"),
            })
            .collect();

        Ok((input, cells))
    }

    fn parse_nums(line: &str) -> IResult<&str, Vec<usize>> {
        separated_list1(tag(","), map(digit1, |s: &str| s.parse::<usize>().unwrap()))(line)
    }

    fn parse_helper(line: &str) -> IResult<&str, ParseResult> {
        let (line, cells) = parse_cells(line)?;
        let (line, _) = space1(line)?;
        let (line, damaged_counts) = parse_nums(line)?;
        let (_, _) = eof(line)?;
        Ok((
            "",
            ParseResult {
                cells,
                damaged_counts,
            },
        ))
    }

    parse_helper(line).unwrap().1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_a() {
        for (s, exp) in [
            ("???.### 1,1,3", 1),
            (".??..??...?##. 1,1,3", 4),
            ("?#?#?#?#?#?#?#? 1,3,1,6", 1),
            ("????.#...#... 4,1,1", 1),
            ("????.######..#####. 1,6,5", 4),
            ("?###???????? 3,2,1", 10),
        ] {
            assert_eq!(a_line(s), exp);
        }
    }

    #[test]
    fn b_easy() {
        assert_eq!(b_line("???.### 1,1,3"), 1);
    }

    #[test]
    fn sample_b() {
        for (s, exp) in [
            (".??..??...?##. 1,1,3", 16384),
            ("?#?#?#?#?#?#?#? 1,3,1,6", 1),
            ("????.#...#... 4,1,1", 16),
            ("????.######..#####. 1,6,5", 2500),
            ("?###???????? 3,2,1", 506250),
        ] {
            assert_eq!(b_line(s), exp);
        }
    }
}
