# Advent-25 (Rust)

Highly optimized Advent of Code 2025 solutions with a performance-first runner.
All implemented days (1-12) execute in under ~10ms total in release mode.

This repository is an experiment and a learning project: everything here is written in Rust as a way
to learn the language. I did not have prior Rust experience and built these solutions with help from
Claude Code and Codex.

## Performance

| Day | Part 1 | Part 2 | Time |
|-----|--------|--------|------|
| 1 | 964 | 5872 | 40µs |
| 2 | 20223751480 | 30260171216 | 18µs |
| 3 | 17085 | 169408143086082 | 141µs |
| 4 | 1344 | 8112 | 328µs |
| 5 | 567 | 354149806372909 | 35µs |
| 6 | 6503327062445 | 9640641878593 | 95µs |
| 7 | 1581 | 73007003089792 | 42µs |
| 8 | 102816 | 100011612 | 1.65ms |
| 9 | 4750092396 | 1468516555 | 1.02ms |
| 10 | 459 | 18687 | 1.71ms |
| 11 | 423 | 333657640517376 | 198µs |
| 12 | 510 | - | 240µs |
| **Total** | | | **5.5ms** |

*Average of 100 runs on Apple M3 Pro, `cargo run --release -- all`*

## Highlights
- Sub-10ms total runtime for days 1-12 in release builds (hardware dependent).
- Embedded puzzle inputs for consistent benchmarking and repeatable runs.
- Parallel execution enabled by default via rayon (day 10 benefits most).
- Optimization details and profiling notes in `OPTIMIZATIONS.md`.

## Quick start
```bash
cargo run --release -- all
cargo run --release -- 8
cargo run --release -- 8 2
ADVENT_HIDE_TIMING=1 cargo run --release -- all
```

## Inputs
`inputs/*.txt` are compiled in via `include_str!` for days 1-12. Update those files
to rerun with different inputs. For other days, input is read from stdin.

## Performance and profiling
Timings are printed to stderr; the total time is reported when running `all`.
Set `PPROF=1` when running day 2 to generate `day2_flame.svg` and `day2_top.txt`.
Use `PPROF_LOOPS` to increase the number of iterations for more stable samples.

```bash
PPROF=1 PPROF_LOOPS=1000 cargo run --release -- 2
```

## Project layout
- `src/dayXX.rs`: per-day solutions (day 12 is part 1 only).
- `src/main.rs`: CLI runner and timing.
- `output/answers.txt`: generated when running `all`.
