use advent_25::{solve, Part, IMPLEMENTED_DAYS};
use std::collections::HashMap;
use std::env;
use std::io::{self, Read, Write};
use std::time::{Duration, Instant};

fn main() {
    let mut args = env::args().skip(1);
    let day_selection = parse_day(args.next());
    let part_arg = args.next();
    let show_timing = env::var("ADVENT_HIDE_TIMING").is_err();
    let part = match day_selection {
        DaySelection::All => {
            if let Some(part) = part_arg {
                eprintln!("Ignoring part argument '{}' when running all days.", part);
            }
            Part::Both
        }
        DaySelection::One(_) => parse_part(part_arg),
    };

    if args.next().is_some() {
        eprintln!("Unexpected extra arguments.\nUsage: advent-25 <day|all> [part] < input.txt");
        std::process::exit(1);
    }

    match day_selection {
        DaySelection::All => {
            let mut outcomes = Vec::new();
            for &day in &IMPLEMENTED_DAYS {
                let input = match day {
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
                    _ => unreachable!("IMPLEMENTED_DAYS contained unexpected day {}", day),
                };
                let outcome = run_day(day, Part::Both, input, show_timing);
                print_outcome(&outcome, true);
                outcomes.push(outcome);
            }

            if show_timing {
                let total_time: Duration = outcomes.iter().filter_map(|o| o.elapsed).sum();
                eprintln!("Total execution time: {:.3?}", total_time);
            }

            if let Err(err) = save_answers(&outcomes) {
                eprintln!("Warning: failed to write output/answers.txt: {}", err);
            }
        }
        DaySelection::One(day) => {
            let mut input_owned = String::new();
            let input = input_for_day(day, &mut input_owned);
            let outcome = run_day(day, part, input, show_timing);
            print_outcome(&outcome, false);
        }
    }
}

fn run_day(day: u8, part: Part, input: &str, show_timing: bool) -> RunOutcome {
    let profiling = env::var_os("PPROF").is_some() && day == 2;
    let loops: usize = env::var("PPROF_LOOPS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(1);
    let guard = if profiling {
        pprof::ProfilerGuardBuilder::default()
            .frequency(5000)
            .build()
            .ok()
    } else {
        None
    };

    let timer_start = show_timing.then(Instant::now);
    let mut answer = String::new();
    for iter in 0..loops {
        let res = match solve(day, part, input) {
            Ok(answer) => answer,
            Err(err) => {
                eprintln!("Error (day {}): {}", day, err);
                std::process::exit(1);
            }
        };
        if iter + 1 == loops {
            answer = res;
        }
    }
    let elapsed = timer_start.map(|start| start.elapsed());

    if let Some(g) = guard {
        if let Ok(report) = g.report().build() {
            emit_profile_outputs(&report);
        }
    }

    RunOutcome {
        day,
        answer,
        elapsed,
    }
}

fn emit_profile_outputs(report: &pprof::Report) {
    if let Ok(mut file) = std::fs::File::create("day2_flame.svg") {
        let _ = report.flamegraph(&mut file);
        let _ = writeln!(io::stderr(), "Wrote day2_flame.svg");
    }

    // Inclusive counts: every symbol on the stack gets the sample.
    let mut inclusive: HashMap<String, isize> = HashMap::new();
    let mut leaf: HashMap<String, isize> = HashMap::new();
    let mut total_samples: isize = 0;
    for (frames, &cnt) in report.data.iter() {
        total_samples += cnt;
        let mut last: Option<String> = None;
        for symbol in frames.frames.iter().flat_map(|frame| frame.iter()) {
            let name = symbol.name();
            *inclusive.entry(name.clone()).or_default() += cnt;
            last = Some(name);
        }
        if let Some(name) = last {
            *leaf.entry(name).or_default() += cnt;
        }
    }

    let to_vec = |map: HashMap<String, isize>| {
        let mut v: Vec<(String, isize)> = map.into_iter().collect();
        v.sort_by(|a, b| b.1.cmp(&a.1));
        v
    };
    let incl_vec = to_vec(inclusive);
    let leaf_vec = to_vec(leaf);

    let period_ns = if report.timing.frequency > 0 {
        1_000_000_000u128 / report.timing.frequency as u128
    } else {
        0
    };

    let fmt_entries = |out: &mut std::fs::File,
                       title: &str,
                       entries: &[(String, isize)],
                       total_samples: isize,
                       period_ns: u128| {
        let _ = writeln!(out, "{} (top 50):", title);
        for (idx, (name, cnt)) in entries.iter().take(50).enumerate() {
            let pct = if total_samples > 0 {
                (*cnt as f64 / total_samples as f64) * 100.0
            } else {
                0.0
            };
            let ns = *cnt as u128 * period_ns;
            let ms = ns as f64 / 1_000_000.0;
            let _ = writeln!(
                out,
                "{:>3}. {:>8} samples {:>9.3} ms ({:>5.2}%): {}",
                idx + 1,
                cnt,
                ms,
                pct,
                name
            );
        }
        let _ = writeln!(out);
    };

    if let Ok(mut file) = std::fs::File::create("day2_top.txt") {
        fmt_entries(&mut file, "Inclusive", &incl_vec, total_samples, period_ns);
        fmt_entries(&mut file, "Leaf", &leaf_vec, total_samples, period_ns);
        let _ = writeln!(io::stderr(), "Wrote day2_top.txt");
    }
}

fn parse_day(raw: Option<String>) -> DaySelection {
    let value = match raw {
        Some(v) => v,
        None => {
            eprintln!("Missing day.\nUsage: advent-25 <day|all> [part] < input.txt");
            std::process::exit(1);
        }
    };

    match value.as_str() {
        "all" | "All" | "ALL" | "*" | "0" => DaySelection::All,
        _ => DaySelection::One(value.parse().unwrap_or_else(|_| {
            eprintln!("Day must be a number between 1 and 25, got '{}'", value);
            std::process::exit(1);
        })),
    }
}

fn parse_part(raw: Option<String>) -> Part {
    match raw {
        Some(value) => Part::from_str(&value).unwrap_or_else(|err| {
            eprintln!("{}", err);
            std::process::exit(1);
        }),
        None => Part::Both,
    }
}

fn input_for_day<'a>(day: u8, input_owned: &'a mut String) -> &'a str {
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
        _ => {
            if let Err(err) = io::stdin().read_to_string(input_owned) {
                eprintln!("Failed to read input: {}", err);
                std::process::exit(1);
            }
            input_owned
        }
    }
}

fn print_outcome(outcome: &RunOutcome, prefix_day: bool) {
    if prefix_day {
        println!("Day {}:\n{}", outcome.day, outcome.answer);
        println!();
    } else {
        println!("{}", outcome.answer);
    }

    // Keep stdout and stderr messages in order when both streams are visible.
    let _ = io::stdout().flush();

    if let Some(elapsed) = outcome.elapsed {
        if prefix_day {
            eprintln!("Execution time (day {}): {:.3?}", outcome.day, elapsed);
        } else {
            eprintln!("Execution time: {:.3?}", elapsed);
        }
    }
}

fn save_answers(outcomes: &[RunOutcome]) -> std::io::Result<()> {
    use std::fs;
    use std::io::Write;

    fs::create_dir_all("output")?;
    let mut buf = String::new();
    for outcome in outcomes {
        buf.push_str(&format!("Day {}:\n{}\n\n", outcome.day, outcome.answer));
    }
    let mut file = fs::File::create("output/answers.txt")?;
    file.write_all(buf.as_bytes())
}

#[derive(Copy, Clone)]
enum DaySelection {
    One(u8),
    All,
}

struct RunOutcome {
    day: u8,
    answer: String,
    elapsed: Option<Duration>,
}
