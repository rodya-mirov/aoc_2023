use ahash::{HashMap, HashSet};
use std::collections::VecDeque;

const INPUT_FILE: &'static str = "input/10.txt";

pub fn a() -> String {
    let input = std::fs::read_to_string(INPUT_FILE).expect("Input should exist");
    a_with_input(&input).to_string()
}

fn a_with_input(input: &str) -> usize {
    let map = parse(input);

    let mut to_process = VecDeque::new();
    let mut seen = HashSet::default();
    to_process.push_back((map.start, 0_usize));

    let mut max = 0;

    while let Some((pos, cost)) = to_process.pop_front() {
        if !seen.insert(pos) {
            continue;
        }

        max = max.max(cost);

        if let Some(conns) = map.edges.get(&pos) {
            for node in conns.iter().copied() {
                to_process.push_back((node, cost + 1));
            }
        }
    }

    max
}

pub fn b() -> String {
    let input = std::fs::read_to_string(INPUT_FILE).expect("Input should exist");
    b_with_input(&input).to_string()
}

fn b_with_input(input: &str) -> usize {
    let map = parse(input);

    // Basic idea:
    //      Double the resolution of the grid and add a buffer on the outside
    //          -- so (x, y) in the original grid maps to (2x+1, 2y+1)
    //      Populate the new grid with elements of the original main loop, along with their connections
    //          -- so if we have (x, y) connecting to (x+1, y), then the new grid would contain
    //          (2x+1, 2y+1) and (2x+3, 2y+1) from the nodes, and (2x+2, 2y+1) from the edge
    //      Flood the grid from the outside; everything reachable from the outside is not in the loop;
    //          everything that is not reachable from the outside is in the loop (or part of the loop)
    //      Map this back to the original grid, dropping new points -- so (x, y) becomes ((x-1)/2, (y-1)/2)
    //          if x and y are both odd, or ignored otherwise --and track which ones are connected
    //          to the outside
    //      Final answer is total_space - reachable_from_outside - num_main_loop_tiles, all computed
    //          in the final (original size) grid

    let small_to_big = |p: Pos| -> Pos {
        let Pos { x, y } = p;
        Pos {
            x: x * 2 + 1,
            y: y * 2 + 1,
        }
    };

    // PRE: a and b were adjacent in small coordinates, but are now big (and thus odd, and differ by two)
    let between_to_big = |a: Pos, b: Pos| -> Pos {
        Pos {
            x: (a.x + b.x) / 2,
            y: (a.y + b.y) / 2,
        }
    };

    let big_width = map.width * 2 + 2;
    let big_height = map.height * 2 + 2;

    let main_loop_pts_big_grid: HashSet<Pos> = {
        // big points that are part of the main loop
        let mut seen = HashSet::default();

        // small points from the map
        let mut to_process = VecDeque::new();

        to_process.push_back(map.start);

        while let Some(node) = to_process.pop_front() {
            let node_big = small_to_big(node);
            if !seen.insert(node_big) {
                continue;
            }

            for conn in map.edges.get(&node).unwrap().iter().copied() {
                let conn_big = small_to_big(conn);
                let edge = between_to_big(node_big, conn_big);
                seen.insert(edge); // likely redundant, it's fine

                to_process.push_back(conn);
            }
        }

        seen
    };

    let is_outside_reachable = {
        let mut reachable = vec![vec![false; big_width]; big_height];

        let mut to_process = VecDeque::new();
        for x in 0..big_width {
            to_process.push_back(Pos { x, y: 0 });
            to_process.push_back(Pos {
                x,
                y: big_height - 1,
            });
        }
        for y in 0..big_height {
            to_process.push_back(Pos { x: 0, y });
            to_process.push_back(Pos {
                x: big_width - 1,
                y,
            });
        }

        while let Some(node) = to_process.pop_front() {
            // already been processed
            if reachable[node.y][node.x] {
                continue;
            }

            reachable[node.y][node.x] = true;

            for next_node in vec![node.left(), node.right(), node.up(), node.down()]
                .into_iter()
                .filter_map(|n| n)
            {
                if next_node.x < big_width
                    && next_node.y < big_height
                    && !main_loop_pts_big_grid.contains(&next_node)
                {
                    to_process.push_back(next_node);
                }
            }
        }

        reachable
    };

    (0..map.width)
        .map(|x| {
            (0..map.height)
                .filter(|&y| {
                    let big_pos = small_to_big(Pos { x, y });
                    !main_loop_pts_big_grid.contains(&big_pos)
                        && !is_outside_reachable[big_pos.y][big_pos.x]
                })
                .count()
        })
        .sum()
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Ord, PartialOrd)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn left(self) -> Option<Self> {
        let Self { x, y } = self;
        if x > 0 {
            Some(Self { x: x - 1, y })
        } else {
            None
        }
    }

    fn right(self) -> Option<Self> {
        let Self { x, y } = self;
        Some(Self { x: x + 1, y })
    }

    fn up(self) -> Option<Self> {
        let Self { x, y } = self;
        if y > 0 {
            Some(Self { x, y: y - 1 })
        } else {
            None
        }
    }

    fn down(self) -> Option<Self> {
        let Self { x, y } = self;
        Some(Self { x, y: y + 1 })
    }
}

#[derive(Clone, Debug)]
struct Map {
    start: Pos,
    edges: HashMap<Pos, Vec<Pos>>,
    height: usize,
    width: usize,
}

fn parse(input: &str) -> Map {
    let mut edges = HashMap::default();
    let mut start: Option<Pos> = None;

    let height = input.lines().count();
    let width = input.lines().next().unwrap().len(); // ASCII text

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let pos = Pos { x, y };
            let my_edges = match c {
                '|' => {
                    vec![pos.up(), pos.down()]
                }
                '-' => {
                    vec![pos.left(), pos.right()]
                }
                'L' => {
                    vec![pos.up(), pos.right()]
                }
                'J' => {
                    vec![pos.up(), pos.left()]
                }
                '7' => {
                    vec![pos.down(), pos.left()]
                }
                'F' => {
                    vec![pos.down(), pos.right()]
                }
                '.' => {
                    vec![]
                }
                'S' => {
                    start = Some(pos);
                    // we'll figure this out at the end
                    vec![]
                }
                other => {
                    panic!("Bad input char: {}", other);
                }
            };

            let my_edges: Vec<Pos> = my_edges
                .iter()
                .filter_map(|p| p.as_ref().copied())
                .collect();

            edges.insert(pos, my_edges);
        }
    }

    if start.is_none() {
        panic!("Didn't find start");
    }

    let start = start.unwrap();

    let mut start_edges = Vec::with_capacity(2);

    for (node, node_edges) in edges.iter() {
        if node_edges.contains(&start) {
            start_edges.push(*node);
        }
    }

    edges.insert(start, start_edges);

    Map {
        start,
        edges,
        height,
        width,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_a1() {
        const SAMPLE_01: &'static str = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";

        assert_eq!(a_with_input(SAMPLE_01), 4);
    }

    #[test]
    fn sample_a2() {
        const SAMPLE_02: &'static str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

        assert_eq!(a_with_input(SAMPLE_02), 8);
    }

    #[test]
    fn sample_b1() {
        const SAMPLE_03: &'static str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

        assert_eq!(b_with_input(SAMPLE_03), 4);
    }

    #[test]
    fn sample_b1_more() {
        // solution -- note the I's are truly in the loop, but the O's can sort of squeeze through
        // not even sure where to get started here, i might actually need to simulate the squeezing?
        // maybe like ... expand the map so the squeezable space is an actual tile, flood that,
        // then shrink back down ...
        const _SAMPLE_03: &'static str = "..........
.S------7.
.|F----7|.
.||OOOO||.
.||OOOO||.
.|L-7F-J|.
.|II||II|.
.L--JL--J.
..........";

        // unmarked
        const SAMPLE: &'static str = "..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";

        assert_eq!(b_with_input(SAMPLE), 4);
    }

    #[test]
    fn sample_b2() {
        const SAMPLE_04: &'static str = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

        assert_eq!(b_with_input(SAMPLE_04), 8);
    }

    #[test]
    fn sample_b3() {
        const SAMPLE_05: &'static str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

        assert_eq!(b_with_input(SAMPLE_05), 10);
    }
}
