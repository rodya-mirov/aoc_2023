use ahash::{HashMap, HashSet};
use itertools::Itertools;

use nom::bytes::complete::tag;
use nom::character::complete::alphanumeric1;
use nom::combinator::eof;
use nom::IResult;

const INPUT_FILE: &str = "input/08.txt";

pub fn a() -> String {
    let input = std::fs::read_to_string(INPUT_FILE).expect("Input should exist");
    a_with_input(&input).to_string()
}

fn a_with_input(input: &str) -> usize {
    let ParseResult {
        moves,
        map,
        name_lookup,
    } = parse_input(input);

    let mut curr_node = name_lookup.get("AAA").copied().unwrap();
    let target = name_lookup.get("ZZZ").copied().unwrap();

    let mut move_idx = 0;

    let mut num_moves = 0;

    while curr_node != target {
        let next_move = moves[move_idx];

        let options = map[&curr_node];

        curr_node = match next_move {
            Turn::L => options.0,
            Turn::R => options.1,
        };

        num_moves += 1;
        move_idx += 1;
        if move_idx >= moves.len() {
            move_idx -= moves.len();
        }
    }

    num_moves
}

pub fn b() -> String {
    let input = std::fs::read_to_string(INPUT_FILE).expect("Input should exist");
    b_with_input(&input).to_string()
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
struct MoveState {
    pos: usize,
    move_idx: usize,
}

impl MoveState {
    fn next(&mut self, moves: &[Turn], map: &HashMap<usize, (usize, usize)>) {
        let next_turn = moves[self.move_idx];

        let options: (usize, usize) = map[&self.pos];

        self.pos = match next_turn {
            Turn::L => options.0,
            Turn::R => options.1,
        };

        self.move_idx += 1;
        if self.move_idx == moves.len() {
            self.move_idx = 0;
        }
    }
}

/// Returns the offset, then the cycle length
fn find_period(
    start: usize,
    map: &HashMap<usize, (usize, usize)>,
    moves: &[Turn],
) -> (usize, usize) {
    if moves.is_empty() {
        panic!("can't have empty moves list");
    }

    let mut state = MoveState {
        move_idx: 0,
        pos: start,
    };

    let mut iters = 0;

    let mut seen: HashSet<MoveState> = HashSet::default();

    while seen.insert(state) {
        state.next(moves, map);
        iters += 1;
    }

    let first_repeat_iters = iters;
    let goal_state = state;

    while iters == first_repeat_iters || state != goal_state {
        state.next(moves, map);
        iters += 1;
    }

    let second_repeat_iters = iters;

    let period = second_repeat_iters - first_repeat_iters;
    let offset = first_repeat_iters - period;

    (offset, period)
}

/// Given the initial position and offset, find the points (in the repeating period) where the
/// actor is in a target state.
fn find_target_times(
    start_pos: usize,
    starting_offset: usize,
    period: usize,
    targets: &HashSet<usize>,
    moves: &[Turn],
    map: &HashMap<usize, (usize, usize)>,
) -> Vec<usize> {
    let mut state = MoveState {
        pos: start_pos,
        move_idx: 0,
    };

    for _ in 0..starting_offset {
        state.next(moves, map);
    }

    let mut target_times = Vec::new();

    for i in 0..period {
        if targets.contains(&state.pos) {
            target_times.push(i);
        }

        state.next(moves, map);
    }

    target_times
}

fn lcm(a: i128, b: i128) -> i128 {
    let g = ring_algorithm::gcd(a, b);
    (a / g) * b
}

fn b_with_input(input: &str) -> i128 {
    let ParseResult {
        moves,
        map,
        name_lookup,
    } = parse_input(input);

    let mut is_source: HashSet<usize> = HashSet::default();
    let mut is_target: HashSet<usize> = HashSet::default();

    let mut current_positions: Vec<usize> = Vec::new();

    for (name, idx) in name_lookup.iter() {
        if name.ends_with('A') {
            is_source.insert(*idx);
            current_positions.push(*idx);
        } else if name.ends_with('Z') {
            is_target.insert(*idx);
        }
    }

    assert_eq!(is_source.len(), is_target.len());

    let num_ghosts = current_positions.len();

    let mut periods = Vec::with_capacity(num_ghosts);
    let mut max_offset = 0;

    for p in current_positions.iter().copied() {
        let (offset, period) = find_period(p, &map, &moves);

        periods.push(period);
        max_offset = max_offset.max(offset);
    }

    // Annoying correctness check: check for early stopping in the 0-to-offset region. Good in
    // some abstract sense but doesn't happen in the real data.
    {
        let mut current_positions: Vec<MoveState> = current_positions
            .iter()
            .copied()
            .map(|pos| MoveState { pos, move_idx: 0 })
            .collect();

        for step in 0..max_offset {
            if current_positions.iter().all(|m| is_target.contains(&m.pos)) {
                return step as i128;
            }
            current_positions
                .iter_mut()
                .for_each(|m| m.next(&moves, &map));
        }
    }

    let mut target_times: Vec<Vec<usize>> = Vec::with_capacity(num_ghosts);

    for i in 0..num_ghosts {
        let target_time = find_target_times(
            current_positions[i],
            max_offset,
            periods[i],
            &is_target,
            &moves,
            &map,
        );

        target_times.push(target_time);
    }

    let periods: Vec<i128> = periods.iter().copied().map(|t| t as i128).collect();

    let mut best_solution = i128::MAX;
    let mut found_solution = false;

    for targets in target_times.iter().multi_cartesian_product() {
        // at this point you need the least time where all of them are at the target; they're at the
        // target precisely if they're on step target[i] (mod period[i]) so this is a CRT thing
        let targets: Vec<i128> = targets.into_iter().copied().map(|t| t as i128).collect();

        let maybe_solution = ring_algorithm::chinese_remainder_theorem(&targets, &periods);

        if let Some(found) = maybe_solution {
            // no guarantee that's remotely in the right modulus, for some reason; need to normalize it
            let total_modulus = periods.iter().copied().fold(1, lcm);
            let soln = found.rem_euclid(total_modulus);

            best_solution = best_solution.min(soln);
            found_solution = true;
        }
    }

    if !found_solution {
        panic!("No solution found!");
    }

    // don't forget the offset we skipped at the beginning
    best_solution + (max_offset as i128)
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
enum Turn {
    L,
    R,
}

struct ParseResult {
    moves: Vec<Turn>,
    map: HashMap<usize, (usize, usize)>,
    name_lookup: HashMap<String, usize>,
}

fn parse_input(input: &str) -> ParseResult {
    fn parse(input: &str) -> IResult<&str, ParseResult> {
        let mut lines = input.lines();

        let moves: Vec<Turn> = lines
            .next()
            .unwrap()
            .chars()
            .map(|c| {
                if c == 'R' {
                    Turn::R
                } else if c == 'L' {
                    Turn::L
                } else {
                    panic!("BAD MOVE CHAR {c}")
                }
            })
            .collect();

        assert_eq!(lines.next(), Some(""));

        let mut name_to_index: HashMap<String, usize> = HashMap::default();

        let mut next_id = |name: &str| -> usize {
            let len_now = name_to_index.len();
            *name_to_index.entry(name.to_string()).or_insert(len_now)
        };

        let mut map = HashMap::default();

        for line in lines {
            let (line, source_name) = alphanumeric1(line)?;
            let (line, _) = tag(" = (")(line)?;
            let (line, l_name) = alphanumeric1(line)?;
            let (line, _) = tag(", ")(line)?;
            let (line, r_name) = alphanumeric1(line)?;
            let (line, _) = tag(")")(line)?;
            let (_, _) = eof(line)?;

            let source_idx = next_id(source_name);
            let l_idx = next_id(l_name);
            let r_idx = next_id(r_name);

            map.insert(source_idx, (l_idx, r_idx));
        }

        Ok((
            "",
            ParseResult {
                moves,
                map,
                name_lookup: name_to_index,
            },
        ))
    }

    parse(input).unwrap().1
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_1: &'static str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    const SAMPLE_2: &'static str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn sample_a_1() {
        assert_eq!(a_with_input(SAMPLE_1), 2);
    }

    #[test]
    fn sample_a_2() {
        assert_eq!(a_with_input(SAMPLE_2), 6);
    }

    const SAMPLE_B: &'static str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn sample_b() {
        assert_eq!(b_with_input(SAMPLE_B), 6);
    }
}
