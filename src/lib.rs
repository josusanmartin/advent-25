pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;

pub const IMPLEMENTED_DAYS: [u8; 12] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];

#[derive(Copy, Clone, Debug)]
pub enum Part {
    One,
    Two,
    Both,
}

impl Part {
    pub fn from_str(raw: &str) -> Result<Self, String> {
        match raw {
            "1" | "one" | "One" => Ok(Self::One),
            "2" | "two" | "Two" => Ok(Self::Two),
            "both" | "Both" | "all" | "*" | "0" => Ok(Self::Both),
            other => Err(format!("part must be 1, 2, or both, got '{}'", other)),
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Part::One => "1",
            Part::Two => "2",
            Part::Both => "both",
        }
    }
}

pub fn solve(day: u8, part: Part, input: &str) -> Result<String, String> {
    if !(1..=25).contains(&day) {
        return Err(format!("day must be between 1 and 25, got {}", day));
    }

    match (day, part) {
        (1, Part::One) => day01::part1(input).map(|n| n.to_string()),
        (1, Part::Two) => day01::part2(input).map(|n| n.to_string()),
        (1, Part::Both) => {
            let p1 = day01::part1(input)?;
            let p2 = day01::part2(input)?;
            Ok(format!("Part 1: {}\nPart 2: {}", p1, p2))
        }
        (2, Part::One) => day02::part1(input).map(|n| n.to_string()),
        (2, Part::Two) => day02::part2(input).map(|n| n.to_string()),
        (2, Part::Both) => {
            let (p1, p2) = day02::both(input)?;
            Ok(format!("Part 1: {}\nPart 2: {}", p1, p2))
        }
        (3, Part::One) => day03::part1(input).map(|n| n.to_string()),
        (3, Part::Two) => day03::part2(input).map(|n| n.to_string()),
        (3, Part::Both) => {
            let (p1, p2) = day03::both(input)?;
            Ok(format!("Part 1: {}\nPart 2: {}", p1, p2))
        }
        (4, Part::One) => day04::part1(input).map(|n| n.to_string()),
        (4, Part::Two) => day04::part2(input).map(|n| n.to_string()),
        (4, Part::Both) => {
            let (p1, p2) = day04::both(input)?;
            Ok(format!("Part 1: {}\nPart 2: {}", p1, p2))
        }
        (5, Part::One) => day05::part1(input).map(|n| n.to_string()),
        (5, Part::Two) => day05::part2(input).map(|n| n.to_string()),
        (5, Part::Both) => {
            let (p1, p2) = day05::both(input)?;
            Ok(format!("Part 1: {}\nPart 2: {}", p1, p2))
        }
        (6, Part::One) => day06::part1(input).map(|n| n.to_string()),
        (6, Part::Two) => day06::part2(input).map(|n| n.to_string()),
        (6, Part::Both) => {
            let (p1, p2) = day06::both(input)?;
            Ok(format!("Part 1: {}\nPart 2: {}", p1, p2))
        }
        (7, Part::One) => day07::part1(input).map(|n| n.to_string()),
        (7, Part::Two) => day07::part2(input).map(|n| n.to_string()),
        (7, Part::Both) => {
            let (p1, p2) = day07::both(input)?;
            Ok(format!("Part 1: {}\nPart 2: {}", p1, p2))
        }
        (8, Part::One) => day08::part1(input).map(|n| n.to_string()),
        (8, Part::Two) => day08::part2(input).map(|n| n.to_string()),
        (8, Part::Both) => {
            let (p1, p2) = day08::both(input)?;
            Ok(format!("Part 1: {}\nPart 2: {}", p1, p2))
        }
        (9, Part::One) => day09::part1(input).map(|n| n.to_string()),
        (9, Part::Two) => day09::part2(input).map(|n| n.to_string()),
        (9, Part::Both) => {
            let (p1, p2) = day09::both(input)?;
            Ok(format!("Part 1: {}\nPart 2: {}", p1, p2))
        }
        (10, Part::One) => day10::part1(input).map(|n| n.to_string()),
        (10, Part::Two) => day10::part2(input).map(|n| n.to_string()),
        (10, Part::Both) => {
            let (p1, p2) = day10::both(input)?;
            Ok(format!("Part 1: {}\nPart 2: {}", p1, p2))
        }
        (11, Part::One) => day11::part1(input).map(|n| n.to_string()),
        (11, Part::Two) => day11::part2(input).map(|n| n.to_string()),
        (11, Part::Both) => {
            let (p1, p2) = day11::both(input)?;
            Ok(format!("Part 1: {}\nPart 2: {}", p1, p2))
        }
        // Day 12 has no part 2 (it's the final day)
        (12, Part::One) | (12, Part::Two) | (12, Part::Both) => {
            day12::part1(input).map(|n| format!("Part 1: {}", n))
        }
        (d, p) => Err(format!("Day {} part {} not implemented yet", d, p.label())),
    }
}
