# Advent of Code 25 in <2ms (Rust)

Highly optimized Advent of Code 2025 solutions with a performance-first runner.
All implemented days (1-12) execute in under 2ms total in release mode (parallel).

This repository is an experiment and a learning project: everything here is written in Rust as a way
to learn the language. I did not have prior Rust experience and built these solutions with help from
Claude Code and Codex.

## Performance

| Day | Mean | Median | Min | Max |
|-----|------|--------|-----|-----|
| 1 | 35µs | 33µs | 32µs | 80µs |
| 2 | 12µs | 12µs | 9µs | 47µs |
| 3 | 126µs | 124µs | 108µs | 200µs |
| 4 | 300µs | 289µs | 253µs | 561µs |
| 5 | 31µs | 30µs | 27µs | 62µs |
| 6 | 85µs | 82µs | 68µs | 200µs |
| 7 | 38µs | 37µs | 33µs | 86µs |
| 8 | 1.74ms | 1.68ms | 1.60ms | 4.14ms |
| 9 | 905µs | 941µs | 588µs | 2.16ms |
| 10 | 1.48ms | 1.44ms | 1.34ms | 2.71ms |
| 11 | 184µs | 177µs | 157µs | 381µs |
| 12 | 238µs | 228µs | 191µs | 593µs |
| **Total** | **5.17ms** | **5.07ms** | **4.41ms** | **11.22ms** |

Wall-clock time:

| Mode | Mean | Median | Min | Max |
|------|------|--------|-----|-----|
| **Parallel** | 1.87ms | 1.86ms | 1.72ms | 2.18ms |
| **Sequential** | 5.18ms | 5.04ms | 4.64ms | 9.37ms |

*100 iterations on Apple M3 Pro*

The table shows per-day times measured sequentially. In parallel mode, days run concurrently on
multiple cores, so the wall-clock time (~1.9ms) is much less than the sum of individual day times
(~5ms). Day 10 benefits most from parallelization as it uses rayon internally.

**Disclaimer**: These timings may be inaccurate due to the inherent difficulties of microbenchmarking
and my lack of experience with Rust profiling. Using [samply](https://github.com/mstange/samply) for
more rigorous profiling is on my to-do list.

### Running benchmarks

```bash
# Rust benchmark (recommended) - runs 100 iterations, shows stats
cargo run --release --bin benchmark

# Python benchmark - compares current run against README reference values
python3 scripts/benchmark.py
```

## Highlights
- Sub-10ms total runtime for days 1-12 in release builds (hardware dependent).
- Embedded puzzle inputs for consistent benchmarking and repeatable runs.
- Parallel execution enabled by default via rayon (day 10 benefits most).
- Optimization details and profiling notes in `OPTIMIZATIONS.md`.

## Quick start
```bash
cargo run --release -- all
cargo run --release -- all seq
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
