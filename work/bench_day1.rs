#[path = "../src/day01.rs"]
mod day01;

use std::hint::black_box;

fn main() {
    let mut args = std::env::args().skip(1);
    let mode = args.next().expect("mode: separate|fused");
    let iterations: usize = args
        .next()
        .unwrap_or_else(|| "10000".into())
        .parse()
        .expect("iterations must be an integer");

    for _ in 0..iterations {
        let answer = if mode == "fused" {
            day01::both(black_box(day01::INPUT)).unwrap()
        } else {
            (
                day01::part1(black_box(day01::INPUT)).unwrap(),
                day01::part2(black_box(day01::INPUT)).unwrap(),
            )
        };
        black_box(answer);
    }
}
