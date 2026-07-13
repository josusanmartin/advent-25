use advent_25::{day08, day10, solve, Part, IMPLEMENTED_DAYS};
use std::hint::black_box;

fn main() {
    let mut args = std::env::args().skip(1);
    let mode = args
        .next()
        .expect("mode: day8|day8-p1|day8-p2|day10|day10-p1|day10-p2|all-seq");
    let iterations: usize = args
        .next()
        .unwrap_or_else(|| "1000".into())
        .parse()
        .expect("iterations must be an integer");

    for _ in 0..iterations {
        match mode.as_str() {
            "day8" => {
                black_box(day08::both(black_box(day08::INPUT)).unwrap());
            }
            "day8-p1" => {
                black_box(day08::part1(black_box(day08::INPUT)).unwrap());
            }
            "day8-p2" => {
                black_box(day08::part2(black_box(day08::INPUT)).unwrap());
            }
            "day10" => {
                black_box(day10::both(black_box(day10::INPUT)).unwrap());
            }
            "day10-p1" => {
                black_box(day10::part1(black_box(day10::INPUT)).unwrap());
            }
            "day10-p2" => {
                black_box(day10::part2(black_box(day10::INPUT)).unwrap());
            }
            "all-seq" => {
                for &day in &IMPLEMENTED_DAYS {
                    let input = match day {
                        1 => advent_25::day01::INPUT,
                        2 => advent_25::day02::INPUT,
                        3 => advent_25::day03::INPUT,
                        4 => advent_25::day04::INPUT,
                        5 => advent_25::day05::INPUT,
                        6 => advent_25::day06::INPUT,
                        7 => advent_25::day07::INPUT,
                        8 => day08::INPUT,
                        9 => advent_25::day09::INPUT,
                        10 => day10::INPUT,
                        11 => advent_25::day11::INPUT,
                        12 => advent_25::day12::INPUT,
                        _ => unreachable!(),
                    };
                    black_box(solve(day, Part::Both, black_box(input)).unwrap());
                }
            }
            _ => panic!("unknown mode: {mode}"),
        }
    }
}
