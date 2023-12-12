use ahash::{HashMap, HashSet};

const INPUT_FILE: &'static str = "input/11.txt";

pub fn a() -> String {
    let input = std::fs::read_to_string(INPUT_FILE).expect("Input should exist");
    a_with_input(&input).to_string()
}

fn a_with_input(input: &str) -> usize {
    expanding_galaxy(input, 2)
}

pub fn b() -> String {
    let input = std::fs::read_to_string(INPUT_FILE).expect("Input should exist");
    b_with_input(&input).to_string()
}

fn b_with_input(input: &str) -> usize {
    expanding_galaxy(input, 1000000)
}

fn expanding_galaxy(input: &str, expansion: usize) -> usize {
    let width = input.lines().next().unwrap().len(); // ASCII input
    let height = input.lines().count();

    // True: galaxy; False: empty space
    let grid: Vec<Vec<bool>> = input
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect::<Vec<bool>>())
        .collect();

    let (empty_rows, empty_columns, galaxy_locations) = {
        let mut galaxy_rows = HashSet::default();
        let mut galaxy_cols = HashSet::default();
        let mut galaxy_locations = Vec::default();

        for (y, row) in grid.iter().enumerate() {
            for (x, is_galaxy) in row.iter().copied().enumerate() {
                if is_galaxy {
                    galaxy_cols.insert(x);
                    galaxy_rows.insert(y);
                    galaxy_locations.push((x, y));
                }
            }
        }

        let empty_rows: Vec<usize> = (0..height).filter(|y| !galaxy_rows.contains(y)).collect();
        let empty_cols: Vec<usize> = (0..width).filter(|x| !galaxy_cols.contains(x)).collect();

        (empty_rows, empty_cols, galaxy_locations)
    };

    // then get the new (expanded) galaxy positions; intentionally consumes / shadows the old one
    let galaxy_locations: Vec<(usize, usize)> = {
        let mut x_lookup = HashMap::default();
        let mut y_lookup = HashMap::default();

        let mut new_x = 0;
        for old_x in 0..width {
            if empty_columns.contains(&old_x) {
                new_x += expansion - 1;
            } else {
                x_lookup.insert(old_x, new_x);
            }

            new_x += 1;
        }

        let mut new_y = 0;
        for old_y in 0..height {
            if empty_rows.contains(&old_y) {
                new_y += expansion - 1;
            } else {
                y_lookup.insert(old_y, new_y);
            }

            new_y += 1;
        }

        galaxy_locations
            .into_iter()
            .map(|(old_x, old_y)| {
                (
                    x_lookup.get(&old_x).copied().unwrap(),
                    y_lookup.get(&old_y).copied().unwrap(),
                )
            })
            .collect()
    };

    let mut total_dist = 0;

    for i in 1..galaxy_locations.len() {
        let a = galaxy_locations[i];

        for j in 0..i {
            let b = galaxy_locations[j];

            let my_dist = (b.1.abs_diff(a.1)) + (b.0.abs_diff(a.0));

            total_dist += my_dist;
        }
    }

    total_dist
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_a() {
        let sample_str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        assert_eq!(a_with_input(sample_str), 374);
    }

    #[test]
    fn sample_b1() {
        let sample_str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        assert_eq!(expanding_galaxy(sample_str, 10), 1030);
    }

    #[test]
    fn sample_b2() {
        let sample_str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        assert_eq!(expanding_galaxy(sample_str, 100), 8410);
    }
}
