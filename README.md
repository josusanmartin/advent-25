# Advent of Code 25 in <1ms (Rust)

Highly optimized Advent of Code 2025 solutions with a performance-first runner.
With this optimization pass, we've managed to solve all implemented days (1-12) in under 1ms:
a 0.805ms median solver time in release mode (parallel).

This repository is an experiment and a learning project: everything here is written in Rust as a way
to learn the language. I did not have prior Rust experience and built these solutions with help from
Claude Code and Codex.

## Performance

### Solver time (what we optimize for)

These times measure pure solver execution, excluding process startup overhead.
Measured by calling solver functions directly in a loop within an already-running process.

The original solver table is intentionally retained below so the improvement made by the
**Codex GPT 5.6 Sol** optimization pass remains visible. The original per-day snapshot was measured
on an M3 Pro and the optimized snapshot on an M3 Max, so the separate same-machine comparison in
the next section is the controlled before/after measurement.

#### Original solver snapshot

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

Original wall-clock time (solver only):

| Mode | Mean | Median | Min | Max |
|------|------|--------|-----|-----|
| **Parallel** | 1.87ms | 1.86ms | 1.72ms | 2.18ms |
| **Sequential** | 5.18ms | 5.04ms | 4.64ms | 9.37ms |

*Original 100-iteration snapshot on an Apple M3 Pro.*

#### Codex GPT 5.6 Sol optimized solver snapshot

| Day | Mean | Median | Min | Max |
|-----|------|--------|-----|-----|
| 1 | 19µs | 18µs | 16µs | 114µs |
| 2 | 11µs | 9µs | 7µs | 73µs |
| 3 | 128µs | 124µs | 106µs | 294µs |
| 4 | 291µs | 278µs | 228µs | 540µs |
| 5 | 33µs | 31µs | 29µs | 85µs |
| 6 | 94µs | 90µs | 74µs | 172µs |
| 7 | 40µs | 38µs | 33µs | 90µs |
| 8 | 763µs | 727µs | 689µs | 1.48ms |
| 9 | 576µs | 527µs | 492µs | 1.13ms |
| 10 | 307µs | 292µs | 220µs | 573µs |
| 11 | 68µs | 63µs | 57µs | 117µs |
| 12 | 124µs | 119µs | 109µs | 252µs |
| **Total** | **2.46ms** | **2.33ms** | **2.07ms** | **5.13ms** |

Optimized wall-clock time (solver only):

| Mode | Mean | Median | Min | Max |
|------|------|--------|-----|-----|
| **Parallel** | 811µs | 805µs | 742µs | 1.08ms |
| **Sequential** | 2.46ms | 2.34ms | 2.18ms | 4.25ms |

*Apple M3 Max, via `cargo run --release --bin benchmark`. Each value is the median of the
corresponding statistic from seven independent 100-iteration benchmark processes.*

### Codex optimization pass

This improvement was made by optimizing with **Codex GPT 5.6 Sol** at **Max effort**. The clean,
same-machine comparison below reports the median wall-clock time across seven independent runs of
the 100-iteration benchmark before and after optimization:

| Mode | Before optimization | Codex-optimized | Reduction |
|------|--------------------:|----------------:|----------:|
| **Parallel** | 1.91ms | 0.805ms | 57.9% |
| **Sequential** | 5.10ms | 2.34ms | 54.1% |

*All seven final parallel run medians were below 1ms, ranging from 0.796ms to 0.996ms. Exact puzzle
answers and all release tests were unchanged.*

The table shows per-day times measured sequentially. In parallel mode, days run concurrently on
multiple cores, so the wall-clock time (~0.81ms) is much less than the sum of individual day times
(~2.33ms). Days 8 and 10 also use rayon internally; Day 8 runs its exact MST and top-k edge searches
concurrently.

### End-to-end time (what you experience)

When running the binary directly, there's additional overhead from process startup, runtime
initialization, and output. [Hyperfine](https://github.com/sharkdp/hyperfine) measures this:

```text
$ hyperfine --warmup 10 --runs 100 -N \
    './target/release/advent-25 all' \
    './target/release/advent-25 all seq'
Parallel:    3.5 ms ± 0.3 ms
Sequential:  5.2 ms ± 0.2 ms
```

**Time breakdown:**

| Component | Time |
|-----------|------|
| Solver execution (parallel median) | ~0.81ms |
| Startup, rayon initialization, formatting, file I/O, and measurement | ~2.7ms |
| **Total (hyperfine mean)** | **~3.5ms** |

So the "<1ms" claim refers to solver time. The measured end-to-end parallel execution is ~3.5ms
when process startup and output are included.

### Running benchmarks

```bash
# Rust benchmark - measures solver time only (recommended for optimization)
cargo run --release --bin benchmark

# Hyperfine - measures end-to-end time including process startup
hyperfine --warmup 10 -N './target/release/advent-25 all'

# Compare parallel vs sequential with hyperfine
hyperfine --warmup 10 -N \
  './target/release/advent-25 all' \
  './target/release/advent-25 all seq'
```

### Disclaimer

These timings may be inaccurate due to the inherent difficulties of microbenchmarking and my lack
of experience with Rust profiling. Using [samply](https://github.com/mstange/samply) for more
rigorous profiling is on my to-do list.

## Highlights
- Sub-1ms parallel solver median for days 1-12 in release builds (hardware dependent).
- Embedded puzzle inputs for consistent benchmarking and repeatable runs.
- Parallel execution enabled by default via rayon, including internal parallelism in days 8 and 10.
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

## Profiling with pprof

Set `PPROF=1` when running day 2 to generate `day2_flame.svg` and `day2_top.txt`.
Use `PPROF_LOOPS` to increase the number of iterations for more stable samples.

```bash
PPROF=1 PPROF_LOOPS=1000 cargo run --release -- 2
```

## Project layout
- `src/dayXX.rs`: per-day solutions (day 12 is part 1 only).
- `src/main.rs`: CLI runner and timing.
- `output/answers.txt`: generated when running `all`.
