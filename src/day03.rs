use ahash::HashSet;

const INPUT_FILE: &str = "input/03.txt";

pub fn a() -> String {
    let input = std::fs::read_to_string(INPUT_FILE).expect("Input should exist");
    a_with_input(&input).to_string()
}

fn a_with_input(input: &str) -> u64 {
    // this is the worst thing i've ever written, maybe :thinking:
    let adj_locations: HashSet<(usize, usize)> = input
        // go through each line and collect all the coordinates of special characters ...
        .lines()
        .enumerate()
        .flat_map(|(row_ind, line)| {
            line.chars()
                .enumerate()
                .filter(|&(_, c)| !c.is_alphanumeric() && !(c == '.'))
                .map(move |(col_ind, _)| (row_ind as i32, col_ind as i32))
        })
        // ... then for each coordinate of same, emit every coordinate adjacent to any of those ...
        .flat_map(|(row, col)| {
            (-1..=1).flat_map(move |dx| (-1..=1).map(move |dy| (row + dy, col + dx)))
        })
        .filter(|&(row, col)| row >= 0 && col >= 0)
        .map(|(row, col)| (row as usize, col as usize))
        // ... and collection the result as a hashset
        .collect();

    let mut total_adj = 0;

    for (row_ind, line) in input.lines().enumerate() {
        let mut is_running = false;
        let mut is_adj = false;
        let mut running_total = 0;

        for (col_ind, c) in line.chars().enumerate() {
            if c.is_numeric() {
                if adj_locations.contains(&(row_ind, col_ind)) {
                    is_adj = true;
                }
                is_running = true;
                running_total = running_total * 10 + ((c as u64) - ('0' as u64));
            } else {
                if is_running && is_adj {
                    total_adj += running_total;
                }

                is_running = false;
                is_adj = false;
                running_total = 0;
            }
        }

        if is_running && is_adj {
            total_adj += running_total;
        }
    }

    total_adj
}

pub fn b() -> String {
    let input = std::fs::read_to_string(INPUT_FILE).expect("Input should exist");
    b_with_input(&input).to_string()
}

fn b_with_input(input: &str) -> u64 {
    // this is a little less bad than the previous
    let gear_adjacency: Vec<HashSet<(usize, usize)>> = input
        // go through each line and collect all the coordinates of gears ...
        .lines()
        .enumerate()
        .flat_map(|(row_ind, line)| {
            line.chars()
                .enumerate()
                .filter(|&(_, c)| c == '*')
                .map(move |(col_ind, _)| (row_ind as i32, col_ind as i32))
        })
        // ... then for each coordinate of same, emit every coordinate adjacent to any of those ...
        .map(|(row, col)| {
            (-1..=1)
                .flat_map(move |dx| (-1..=1).map(move |dy| (row + dy, col + dx)))
                .filter(|&(r, c)| r >= 0 && c >= 0)
                .map(|(r, c)| (r as usize, c as usize))
                .collect::<HashSet<_>>()
        })
        // then just blob them up into a list of sets
        .collect();

    let mut gear_parts: Vec<Vec<u64>> = vec![Vec::new(); gear_adjacency.len()];

    for (row_ind, line) in input.lines().enumerate() {
        let mut is_running = false;
        let mut adj_gears: HashSet<usize> = HashSet::default();
        let mut running_total = 0;

        for (col_ind, c) in line.chars().enumerate() {
            if c.is_numeric() {
                for (gear_ind, adj_set) in gear_adjacency.iter().enumerate() {
                    if adj_set.contains(&(row_ind, col_ind)) {
                        adj_gears.insert(gear_ind);
                    }
                }

                is_running = true;
                running_total = running_total * 10 + ((c as u64) - ('0' as u64));
            } else {
                if is_running {
                    for gear_ind in adj_gears.iter().copied() {
                        gear_parts[gear_ind].push(running_total);
                    }
                }

                is_running = false;
                adj_gears.clear();
                running_total = 0;
            }
        }

        if is_running {
            for gear_ind in adj_gears.iter().copied() {
                gear_parts[gear_ind].push(running_total);
            }
        }
    }

    // finally, add up the derived power of all the gears
    let mut actual_total = 0;

    for gear_part in gear_parts.into_iter() {
        if gear_part.len() == 2 {
            actual_total += gear_part[0] * gear_part[1];
        }
    }

    actual_total
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_A: &'static str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn sample_a_test() {
        assert_eq!(a_with_input(SAMPLE_A), 4361);
    }

    #[test]
    fn sample_a_test2() {
        let input = "
467..114
...*..!.
..35..63"
            .trim();
        assert_eq!(a_with_input(input), 467 + 114 + 35 + 63);
    }

    #[test]
    fn sample_b_test() {
        assert_eq!(b_with_input(SAMPLE_A), 467 * 35 + 755 * 598);
    }
}
