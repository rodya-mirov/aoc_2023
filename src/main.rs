// I disagree with specific applications of this lint often enough that I'm just turning it off
// globally
#![allow(clippy::needless_range_loop)]
// this lint just sucks
#![allow(clippy::manual_range_contains)]
#![allow(clippy::comparison_chain)]
#![allow(clippy::bool_to_int_with_if)]

use std::env;
use std::fmt::{Display, Formatter, Write};
use std::time::Instant;

mod helpers;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Side {
    A,
    B,
}

impl Display for Side {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Side::A => f.write_char('a'),
            Side::B => f.write_char('b'),
        }
    }
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        Err(
            "Usage: [run] [problemnumber] [subcase] ; eg:\n\tcargo run --release -- 1 a"
                .to_string(),
        )
    } else {
        let a: i32 = args[1]
            .parse::<i32>()
            .map_err(|s| format!("Cannot parse argument '{}' as int", s))?;
        let b: Side = match args[2].as_str() {
            "a" => Ok(Side::A),
            "b" => Ok(Side::B),
            _err => Err(format!(
                "Cannot parse argument '{}' as subcase; should be 'a' or 'b'",
                args[0].as_str()
            )),
        }?;

        let start = Instant::now();

        let out: String = match (a, b) {
            (1, Side::A) => Ok(day01::a().to_string()),
            (1, Side::B) => Ok(day01::b().to_string()),
            (2, Side::A) => Ok(day02::a().to_string()),
            (2, Side::B) => Ok(day02::b().to_string()),
            (3, Side::A) => Ok(day03::a().to_string()),
            (3, Side::B) => Ok(day03::b().to_string()),
            (4, Side::A) => Ok(day04::a().to_string()),
            (4, Side::B) => Ok(day04::b().to_string()),
            (5, Side::A) => Ok(day05::a().to_string()),
            (5, Side::B) => Ok(day05::b().to_string()),
            (6, Side::A) => Ok(day06::a().to_string()),
            (6, Side::B) => Ok(day06::b().to_string()),
            (7, Side::A) => Ok(day07::a()),
            (7, Side::B) => Ok(day07::b()),
            (8, Side::A) => Ok(day08::a()),
            (8, Side::B) => Ok(day08::b()),
            (9, Side::A) => Ok(day09::a()),
            (9, Side::B) => Ok(day09::b()),
            (10, Side::A) => Ok(day10::a()),
            (10, Side::B) => Ok(day10::b()),
            (11, Side::A) => Ok(day11::a()),
            (11, Side::B) => Ok(day11::b()),
            (12, Side::A) => Ok(day12::a()),
            (12, Side::B) => Ok(day12::b()),
            (13, Side::A) => Ok(day13::a()),
            (13, Side::B) => Ok(day13::b()),
            (14, Side::A) => Ok(day14::a()),
            (14, Side::B) => Ok(day14::b()),
            (15, Side::A) => Ok(day15::a()),
            (15, Side::B) => Ok(day15::b()),
            (16, Side::A) => Ok(day16::a()),
            (16, Side::B) => Ok(day16::b()),
            (17, Side::A) => Ok(day17::a()),
            (17, Side::B) => Ok(day17::b()),
            (18, Side::A) => Ok(day18::a()),
            (18, Side::B) => Ok(day18::b()),
            (19, Side::A) => Ok(day19::a()),
            (19, Side::B) => Ok(day19::b()),
            (20, Side::A) => Ok(day20::a()),
            (20, Side::B) => Ok(day20::b()),
            (21, Side::A) => Ok(day21::a()),
            (21, Side::B) => Ok(day21::b()),
            (22, Side::A) => Ok(day22::a()),
            (22, Side::B) => Ok(day22::b()),
            (23, Side::A) => Ok(day23::a()),
            (23, Side::B) => Ok(day23::b()),
            (24, Side::A) => Ok(day24::a()),
            (24, Side::B) => Ok(day24::b()),
            (25, Side::A) => Ok(day25::a()),
            (25, Side::B) => Ok(day25::b()),
            (day, side) => Err(format!("Day {}, side {} is not yet supported", day, side)),
        }?;

        let elapsed = start.elapsed();

        println!("Day {} -- {}:\n{}", a, b, out);
        println!("Took {0:3} ms", elapsed.as_secs_f32() * 1000.0);

        Ok(())
    }
}
