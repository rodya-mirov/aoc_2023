use nom::bytes::complete::tag;
use nom::character::complete::{digit1, newline, space0, space1};
use nom::combinator::{eof, map};
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::IResult;

const INPUT_FILE: &str = "input/05.txt";

pub fn a() -> String {
    let input = std::fs::read_to_string(INPUT_FILE).expect("Input should exist");
    a_with_input(&input).to_string()
}

fn a_with_input(input: &str) -> usize {
    let almanac = parse(input);
    almanac.least_location_a()
}

pub fn b() -> String {
    let input = std::fs::read_to_string(INPUT_FILE).expect("Input should exist");
    b_with_input(&input).to_string()
}

fn b_with_input(input: &str) -> usize {
    let almanac = parse(input);
    almanac.least_location_b()
}

fn parse(input: &str) -> Almanac {
    fn parse_num(input: &str) -> IResult<&str, usize> {
        map(digit1, |d: &str| d.parse::<usize>().unwrap())(input)
    }

    fn parse_seeds(input: &str) -> IResult<&str, Vec<usize>> {
        let (input, _) = tuple((tag("seeds:"), space1))(input)?;
        let (input, seeds) = separated_list1(space1, parse_num)(input)?;
        Ok((input, seeds))
    }

    fn parse_mapping(input: &str) -> IResult<&str, Mapping> {
        let (input, target_start) = parse_num(input)?;
        let (input, _) = space1(input)?;
        let (input, source_start) = parse_num(input)?;
        let (input, _) = space1(input)?;
        let (input, length) = parse_num(input)?;

        Ok((
            input,
            Mapping {
                source_start,
                target_start,
                source_end: source_start + length,
            },
        ))
    }

    fn blank_then_map<'a>(input: &'a str, map_name: &str) -> IResult<&'a str, FullMapping> {
        // two newlines before it starts
        let (input, _) = newline(input)?;
        let (input, _) = newline(input)?;

        let (input, _) = tuple((tag(map_name), tag(":"), space0, newline))(input)?;

        let (input, mut mappings) = separated_list1(newline, parse_mapping)(input)?;

        mappings.sort();

        for i in 1..mappings.len() {
            if mappings[i - 1].intersects(&mappings[i]) {
                panic!(
                    "OVERLAPPING INTERVALS OH NOOO {:?} and {:?}",
                    mappings[i - 1],
                    mappings[i]
                );
            }
        }

        Ok((input, FullMapping { mappings }))
    }

    fn full_parse(input: &str) -> IResult<&str, Almanac> {
        let (input, seeds) = parse_seeds(input)?;

        let (input, seed_to_soil) = blank_then_map(input, "seed-to-soil map")?;
        let (input, soil_to_fertilizer) = blank_then_map(input, "soil-to-fertilizer map")?;
        let (input, fertilizer_to_water) = blank_then_map(input, "fertilizer-to-water map")?;
        let (input, water_to_light) = blank_then_map(input, "water-to-light map")?;
        let (input, light_to_temperature) = blank_then_map(input, "light-to-temperature map")?;
        let (input, temperature_to_humidity) =
            blank_then_map(input, "temperature-to-humidity map")?;
        let (input, humidity_to_location) = blank_then_map(input, "humidity-to-location map")?;

        let (_, _) = eof(input)?;

        Ok((
            "",
            Almanac {
                seeds,
                mappings: vec![
                    seed_to_soil,
                    soil_to_fertilizer,
                    fertilizer_to_water,
                    water_to_light,
                    light_to_temperature,
                    temperature_to_humidity,
                    humidity_to_location,
                ],
            },
        ))
    }

    let (_, almanac) = full_parse(input).expect("Input should parse");
    almanac
}

struct Almanac {
    seeds: Vec<usize>,
    mappings: Vec<FullMapping>,
}

struct FullMapping {
    mappings: Vec<Mapping>,
}

impl FullMapping {
    fn resolve(&self, input: usize) -> usize {
        for m in &self.mappings {
            if let Some(out) = m.resolve(input) {
                return out;
            }
        }

        // anything that doesn't map stays the same
        input
    }

    // PRE: self.mappings are sorted (ascending)
    fn resolve_interval(&self, input: Interval) -> Vec<Interval> {
        for i in 1..self.mappings.len() {
            if !(self.mappings[i - 1] < self.mappings[i]) {
                panic!("Mappings must be sorted!");
            }
        }

        let mut start = input.start;
        let end = input.end;

        if start >= end {
            return vec![];
        }

        let mut out = Vec::new();

        for mapping in &self.mappings {
            // couldn't decide if a zero-length mapping actually cause a problem but let's just skip it
            if mapping.source_start == mapping.source_end {
                continue;
            }

            // basically there are six possible cases:
            //      1.  input starts and ends before mapping
            //      2.  input starts before mapping, but has some overlap; ends before mapping does
            //      3.  input starts before mapping, and continues after it
            //      4.  input starts and ends inside the mapping
            //      5.  input starts within the mapping, but ends after it
            //      6.  input starts and ends after the mapping

            // case 1: in this case, we don't touch this mapping or any subsequent mapping; done
            //      leftover interval bits are collected at the end
            if end <= mapping.source_start {
                break;
            }

            // now we know that end > mapping.source_start

            // if there is any portion of the interval before the mapping, preserve it and skip
            // ahead to the bit
            if start < mapping.source_start {
                out.push(Interval {
                    start,
                    end: mapping.source_start,
                });
                start = mapping.source_start;
            }

            // now we know that start >= mapping.source_start
            // if additionally start is before mapping.source_end, then some amount overlaps
            if start < mapping.source_end {
                let inside_end = if end < mapping.source_end {
                    end
                } else {
                    mapping.source_end
                };

                let mapped_start = start - mapping.source_start + mapping.target_start;
                let mapped_end = inside_end - mapping.source_start + mapping.target_start;

                if mapped_start < mapped_end {
                    out.push(Interval {
                        start: mapped_start,
                        end: mapped_end,
                    });
                }

                // this may make start > end, which ends the resolution
                start = mapping.source_end;
                if start >= end {
                    return out;
                }
            }
        }

        // any leftover interval space didn't match anything so we can just keep what we have
        if start < end {
            out.push(Interval { start, end });
        }

        unify_intervals(out)
    }
}

fn unify_intervals(mut intervals: Vec<Interval>) -> Vec<Interval> {
    if intervals.is_empty() {
        return intervals;
    }

    intervals.sort();

    let mut iter = intervals.into_iter();

    let mut running = iter.next().unwrap();

    let mut out = Vec::new();

    for next in iter {
        assert!(running.start <= next.start);

        if running.end < next.start {
            out.push(running);
            running = next;
        } else {
            running = Interval {
                start: running.start,
                end: running.end.max(next.end),
            }
        }
    }

    out.push(running);

    out
}

impl Almanac {
    fn resolve_seed(&self, seed: usize) -> usize {
        let mut running = seed;

        for mapping in &self.mappings {
            running = mapping.resolve(running);
        }

        running
    }

    fn least_location_a(&self) -> usize {
        self.seeds
            .iter()
            .copied()
            .map(|seed| self.resolve_seed(seed))
            .min()
            .expect("Seeds should be nonempty")
    }

    fn least_location_b(&self) -> usize {
        assert_eq!(
            self.seeds.len() % 2,
            0,
            "Should have an even number of seeds"
        );

        let seed_intervals: Vec<Interval> = self
            .seeds
            .chunks_exact(2)
            .map(|chunk| {
                if let [start, len] = chunk {
                    Interval {
                        start: *start,
                        end: start + len,
                    }
                } else {
                    panic!();
                }
            })
            .collect();

        let mut total_min = usize::MAX;

        for interval in seed_intervals.iter().copied() {
            let mut running = vec![interval];
            for m in &self.mappings {
                let mut next_running = vec![];

                for interval in running {
                    next_running.extend(m.resolve_interval(interval));
                }

                running = unify_intervals(next_running);
            }

            let my_min = running[0].start;
            total_min = total_min.min(my_min);
        }

        total_min
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Ord, PartialOrd)]
struct Interval {
    // inclusive
    start: usize,
    // exclusive
    end: usize,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Ord, PartialOrd)]
struct Mapping {
    // inclusive
    source_start: usize,
    // exclusive
    source_end: usize,
    target_start: usize,
}

impl Mapping {
    fn resolve(&self, input: usize) -> Option<usize> {
        if input < self.source_start || input >= self.source_end {
            None
        } else {
            Some(input - self.source_start + self.target_start)
        }
    }

    fn intersects(&self, other: &Self) -> bool {
        self.source_start < other.source_end && other.source_start < self.source_end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &'static str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn examples_a() {
        let almanac = parse(SAMPLE_INPUT);

        assert_eq!(almanac.mappings[0].resolve(79), 81);
        assert_eq!(almanac.mappings[0].resolve(14), 14);
        assert_eq!(almanac.mappings[0].resolve(55), 57);
        assert_eq!(almanac.mappings[0].resolve(13), 13);
    }

    #[test]
    fn sample_a() {
        assert_eq!(a_with_input(SAMPLE_INPUT), 35);
    }

    #[test]
    fn examples_b() {
        let mut mapping = FullMapping {
            mappings: vec![
                Mapping {
                    source_start: 98,
                    source_end: 98 + 2,
                    target_start: 50,
                },
                Mapping {
                    source_start: 50,
                    source_end: 50 + 48,
                    target_start: 52,
                },
            ],
        };

        mapping.mappings.sort();

        assert_eq!(mapping.resolve(97), 52 + 47);
        assert_eq!(mapping.resolve(98), 50);

        // 50 to 98 maps directly (with the latter mapping) to 52 to 100
        assert_eq!(
            mapping.resolve_interval(Interval { start: 50, end: 98 }),
            vec![Interval {
                start: 52,
                end: 100
            }]
        );
        // 98 to 100 maps directly (with the former mapping) to 50 to 52
        assert_eq!(
            mapping.resolve_interval(Interval {
                start: 98,
                end: 100
            }),
            vec![Interval { start: 50, end: 52 }]
        );
        // 0 to 50 doesn't touch anything, so it stays alone
        assert_eq!(
            mapping.resolve_interval(Interval { start: 0, end: 50 }),
            vec![Interval { start: 0, end: 50 }]
        );
        // likewise 100 to (whatever) stays alone
        assert_eq!(
            mapping.resolve_interval(Interval {
                start: 100,
                end: 400
            }),
            vec![Interval {
                start: 100,
                end: 400
            }]
        );

        // then for some overlap examples
        // this has everything, so basically it covers the whole space, super easy
        assert_eq!(
            mapping.resolve_interval(Interval { start: 0, end: 400 }),
            vec![Interval { start: 0, end: 400 }]
        );
    }

    #[test]
    fn sample_b() {
        assert_eq!(b_with_input(SAMPLE_INPUT), 46);
    }
}
