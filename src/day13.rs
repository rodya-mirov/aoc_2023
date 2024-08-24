const INPUT_FILE: &str = "input/13.txt";

pub fn a() -> String {
    let input = std::fs::read_to_string(INPUT_FILE).expect("Input should exist");
    a_with_input(&input).to_string()
}

fn a_with_input(input: &str) -> usize {
    parse_blocks(input.lines())
        .iter()
        .enumerate()
        .map(|(i, b)| {
            println!("Checking block {i}");
            b.symmetry()
        })
        .map(|s| s.score())
        .sum()
}

pub fn b() -> String {
    let input = std::fs::read_to_string(INPUT_FILE).expect("Input should exist");
    b_with_input(&input).to_string()
}

fn b_with_input(_input: &str) -> String {
    unimplemented!()
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Symmetry {
    Vertical { col: usize },
    Horizontal { row: usize },
}

impl Symmetry {
    fn score(&self) -> usize {
        match self {
            Symmetry::Vertical { col } => *col,
            Symmetry::Horizontal { row } => 100 * row,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Block {
    // true: ash (.); false: rocks (#)
    // each line has the same length
    lines: Vec<Vec<bool>>,
}

impl Block {
    fn symmetry(&self) -> Symmetry {
        let width = self.lines[0].len();

        for col in 1..(width) {
            if self.is_vertical_symmetry(col) {
                return Symmetry::Vertical { col };
            }
        }

        let height = self.lines.len();

        for row in 1..(height) {
            if self.is_horizontal_symmetry(row) {
                return Symmetry::Horizontal { row };
            }
        }

        panic!("Couldn't find symmetry");
    }

    fn is_vertical_symmetry(&self, col: usize) -> bool {
        if col == 0 {
            return true; // i guess?
        }

        let width = self.lines[0].len();
        let height = self.lines.len();

        let mut rev_col = col;
        let mut col = col - 1;

        if col == width - 1 {
            return true; // i guess?
        }

        loop {
            // check if these lines are symmetric
            for row in 0..height {
                if self.lines[row][col] != self.lines[row][rev_col] {
                    return false;
                }
            }

            // if we didn't fail and got to the end, they're symmetric
            if col == 0 || rev_col == width - 1 {
                return true;
            }

            col -= 1;
            rev_col += 1;
        }
    }

    fn is_horizontal_symmetry(&self, row: usize) -> bool {
        if row == 0 {
            return true; // i guess?
        }

        let width = self.lines[0].len();
        let height = self.lines.len();

        let mut rev_row = row;
        let mut row = row - 1;

        if row == height - 1 {
            return true; // i guess?
        }

        loop {
            for col in 0..width {
                if self.lines[row][col] != self.lines[rev_row][col] {
                    return false;
                }
            }

            if row == 0 || rev_row == height - 1 {
                return true;
            }

            row -= 1;
            rev_row += 1;
        }
    }
}

fn parse_blocks<'a>(lines: impl Iterator<Item = &'a str>) -> Vec<Block> {
    let mut blocks: Vec<Block> = Vec::new();

    let mut running: Vec<Vec<bool>> = Vec::new();

    for line in lines {
        if line.is_empty() {
            let block = std::mem::take(&mut running);
            blocks.push(Block { lines: block });
        } else {
            running.push(parse_line(line));
        }
    }

    if !running.is_empty() {
        blocks.push(Block { lines: running });
    }

    blocks
}

fn parse_line(line: &str) -> Vec<bool> {
    line.chars().map(|c| c == '.').collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_13a_parse() {
        const INPUT: &str = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#;

        let actual = parse_blocks(INPUT.lines());

        let f = false;
        let t = true;

        let expected = vec![
            Block {
                lines: vec![
                    vec![f, t, f, f, t, t, f, f, t],
                    vec![t, t, f, t, f, f, t, f, t],
                    vec![f, f, t, t, t, t, t, t, f],
                    vec![f, f, t, t, t, t, t, t, f],
                    vec![t, t, f, t, f, f, t, f, t],
                    vec![t, t, f, f, t, t, f, f, t],
                    vec![f, t, f, t, f, f, t, f, t],
                ],
            },
            Block {
                lines: vec![
                    vec![f, t, t, t, f, f, t, t, f],
                    vec![f, t, t, t, t, f, t, t, f],
                    vec![t, t, f, f, t, t, f, f, f],
                    vec![f, f, f, f, f, t, f, f, t],
                    vec![f, f, f, f, f, t, f, f, t],
                    vec![t, t, f, f, t, t, f, f, f],
                    vec![f, t, t, t, t, f, t, t, f],
                ],
            },
        ];

        assert_eq!(actual, expected);
    }

    #[test]
    fn sample_13a() {
        const INPUT: &str = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#;

        let expected = 405;
        let actual = a_with_input(INPUT);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_13a_funny_symmetry() {
        const INPUT: &str = r#".##.....#....#.
.##.....#....#.
..#.#..##....##
...###...#..#..
#..##.....##...
.#.#...#.####.#
#.#.....######.
##..###...##...
#..#..##..##..#
.##.#.#........
#..#..##..##..#
#####.#.##.###.
.#....##.#..#.#
..##.#...#..#..
#..####..#..#.."#;

        let mut blocks = parse_blocks(INPUT.lines());

        let block = blocks.remove(0);

        assert!(block.is_horizontal_symmetry(1));

        let symmetry = block.symmetry();

        assert_eq!(symmetry, Symmetry::Horizontal { row: 1 });
    }

    #[test]
    fn test_13a_funny_symmetry_too() {
        const INPUT: &str = r#"
.#.####.#....
#.#....#.#...
###....##.###
#.##..##.#.##
.#.#..#.#.###
#.######.#...
#.##..##.####
"#;

        let block = parse_blocks(INPUT.trim().lines()).remove(0);

        assert!(block.is_vertical_symmetry(12));

        let expected = Symmetry::Vertical { col: 12 };
        let actual = block.symmetry();

        assert_eq!(actual, expected);
    }
}
