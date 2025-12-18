//! Benchmark tool for advent-25
//!
//! Run with: cargo run --release --bin benchmark

use advent_25::{solve, Part, IMPLEMENTED_DAYS};
use std::time::{Duration, Instant};

const ITERATIONS: usize = 100;

fn main() {
    println!("Running {} iterations...", ITERATIONS);

    // Collect per-day times
    let mut day_times: Vec<Vec<Duration>> = vec![Vec::new(); 13];
    let mut wall_times_seq: Vec<Duration> = Vec::new();
    let mut wall_times_par: Vec<Duration> = Vec::new();

    // Sequential runs (for per-day times)
    print!("Sequential: ");
    for i in 0..ITERATIONS {
        if (i + 1) % 25 == 0 {
            print!("{}/{} ", i + 1, ITERATIONS);
            use std::io::Write;
            std::io::stdout().flush().unwrap();
        }

        let total_start = Instant::now();
        for &day in &IMPLEMENTED_DAYS {
            let input = get_input(day);
            let start = Instant::now();
            let _ = solve(day, Part::Both, input);
            day_times[day as usize].push(start.elapsed());
        }
        wall_times_seq.push(total_start.elapsed());
    }
    println!();

    // Parallel runs (for wall-clock comparison)
    #[cfg(feature = "parallel")]
    {
        use rayon::prelude::*;

        print!("Parallel:   ");
        for i in 0..ITERATIONS {
            if (i + 1) % 25 == 0 {
                print!("{}/{} ", i + 1, ITERATIONS);
                use std::io::Write;
                std::io::stdout().flush().unwrap();
            }

            let total_start = Instant::now();
            let _: Vec<_> = IMPLEMENTED_DAYS
                .par_iter()
                .map(|&day| {
                    let input = get_input(day);
                    solve(day, Part::Both, input)
                })
                .collect();
            wall_times_par.push(total_start.elapsed());
        }
        println!();
    }

    println!();
    print_results(&day_times, &wall_times_seq, &wall_times_par);
}

fn get_input(day: u8) -> &'static str {
    match day {
        1 => advent_25::day01::INPUT,
        2 => advent_25::day02::INPUT,
        3 => advent_25::day03::INPUT,
        4 => advent_25::day04::INPUT,
        5 => advent_25::day05::INPUT,
        6 => advent_25::day06::INPUT,
        7 => advent_25::day07::INPUT,
        8 => advent_25::day08::INPUT,
        9 => advent_25::day09::INPUT,
        10 => advent_25::day10::INPUT,
        11 => advent_25::day11::INPUT,
        12 => advent_25::day12::INPUT,
        _ => panic!("No input for day {}", day),
    }
}

fn print_results(
    day_times: &[Vec<Duration>],
    wall_times_seq: &[Duration],
    wall_times_par: &[Duration],
) {
    println!("===============================================================================");
    println!("PER-DAY TIMES ({} iterations)", ITERATIONS);
    println!("===============================================================================");
    println!(
        "| {:^3} | {:^8} | {:^8} | {:^8} | {:^8} |",
        "Day", "Mean", "Median", "Min", "Max"
    );
    println!("|-----|----------|----------|----------|----------|");

    let mut total_mean = Duration::ZERO;
    let mut total_median = Duration::ZERO;
    let mut total_min = Duration::ZERO;
    let mut total_max = Duration::ZERO;

    for day in 1..=12 {
        let times = &day_times[day];
        if times.is_empty() {
            continue;
        }

        let (mean, median, min, max) = compute_stats(times);
        total_mean += mean;
        total_median += median;
        total_min += min;
        total_max += max;

        println!(
            "| {:^3} | {:^8} | {:^8} | {:^8} | {:^8} |",
            day,
            format_duration(mean),
            format_duration(median),
            format_duration(min),
            format_duration(max)
        );
    }

    println!("|-----|----------|----------|----------|----------|");
    println!(
        "| {:^3} | {:^8} | {:^8} | {:^8} | {:^8} |",
        "Tot",
        format_duration(total_mean),
        format_duration(total_median),
        format_duration(total_min),
        format_duration(total_max)
    );

    println!();
    println!("===============================================================================");
    println!("WALL-CLOCK TIMES ({} iterations)", ITERATIONS);
    println!("===============================================================================");
    println!(
        "| {:^10} | {:^8} | {:^8} | {:^8} | {:^8} |",
        "Mode", "Mean", "Median", "Min", "Max"
    );
    println!("|------------|----------|----------|----------|----------|");

    if !wall_times_par.is_empty() {
        let (mean, median, min, max) = compute_stats(wall_times_par);
        println!(
            "| {:^10} | {:^8} | {:^8} | {:^8} | {:^8} |",
            "Parallel",
            format_duration(mean),
            format_duration(median),
            format_duration(min),
            format_duration(max)
        );
    }

    let (mean, median, min, max) = compute_stats(wall_times_seq);
    println!(
        "| {:^10} | {:^8} | {:^8} | {:^8} | {:^8} |",
        "Sequential",
        format_duration(mean),
        format_duration(median),
        format_duration(min),
        format_duration(max)
    );

    println!();
}

fn compute_stats(times: &[Duration]) -> (Duration, Duration, Duration, Duration) {
    let mut sorted: Vec<Duration> = times.to_vec();
    sorted.sort();

    let sum: Duration = sorted.iter().sum();
    let mean = sum / sorted.len() as u32;
    let median = sorted[sorted.len() / 2];
    let min = sorted[0];
    let max = sorted[sorted.len() - 1];

    (mean, median, min, max)
}

fn format_duration(d: Duration) -> String {
    let us = d.as_micros();
    if us >= 1000 {
        format!("{:.2}ms", us as f64 / 1000.0)
    } else {
        format!("{}us", us)
    }
}
