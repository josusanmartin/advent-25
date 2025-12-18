# Advent of Code 25 in 2ms (Rust)

Highly optimized Advent of Code 2025 solutions with a performance-first runner.
All implemented days (1-12) execute in ~2ms total in release mode (parallel).

This repository is an experiment and a learning project: everything here is written in Rust as a way
to learn the language. I did not have prior Rust experience and built these solutions with help from
Claude Code and Codex.

## Performance

| Day | Mean | Median | Min | Max |
|-----|------|--------|-----|-----|
| 1 | 44µs | 42µs | 37µs | 104µs |
| 2 | 18µs | 17µs | 15µs | 65µs |
| 3 | 143µs | 144µs | 124µs | 186µs |
| 4 | 334µs | 339µs | 291µs | 368µs |
| 5 | 35µs | 35µs | 30µs | 54µs |
| 6 | 97µs | 97µs | 83µs | 126µs |
| 7 | 43µs | 42µs | 37µs | 66µs |
| 8 | 1.70ms | 1.70ms | 1.49ms | 1.91ms |
| 9 | 980µs | 984µs | 697µs | 1.44ms |
| 10 | 1.72ms | 1.70ms | 1.55ms | 2.22ms |
| 11 | 199µs | 196µs | 176µs | 347µs |
| 12 | 248µs | 239µs | 202µs | 431µs |
| **Total** | **5.56ms** | **5.54ms** | **4.73ms** | **7.32ms** |

*100 runs on Apple M3 Pro, `cargo run --release -- all seq`*

Wall-clock time (100 runs, Apple M3 Pro):

| Mode | Mean | Median | Min | Max |
|------|------|--------|-----|-----|
| **Parallel** | 2.08ms | 2.06ms | 1.92ms | 2.74ms |
| **Sequential** | 5.57ms | 5.52ms | 5.11ms | 7.27ms |

The table shows per-day times measured sequentially. In parallel mode, days run concurrently on
multiple cores, so the wall-clock time (~2ms) is much less than the sum of individual day times
(~6ms). Day 10 benefits most from parallelization as it uses rayon internally.

**Disclaimer**: These timings may be inaccurate due to the inherent difficulties of microbenchmarking
and my lack of experience with Rust profiling. Using [samply](https://github.com/mstange/samply) for
more rigorous profiling is on my to-do list.

<details>
<summary>Benchmark script</summary>

```python
import subprocess
import re
import statistics

def run_benchmark(args, iterations=100):
    wall_times = []
    for _ in range(iterations):
        result = subprocess.run(
            ['./target/release/advent-25'] + args,
            capture_output=True, text=True
        )
        m = re.search(r'Total execution time: ([\d.]+)(µs|ms)', result.stderr)
        if m:
            val = float(m.group(1))
            wall_times.append(val if m.group(2) == 'ms' else val / 1000)
    return wall_times

def stats(times, label):
    print(f'{label}: mean={statistics.mean(times):.2f}ms, '
          f'median={statistics.median(times):.2f}ms, '
          f'min={min(times):.2f}ms, max={max(times):.2f}ms')

par = run_benchmark(['all'])
seq = run_benchmark(['all', 'seq'])
stats(par, 'Parallel')
stats(seq, 'Sequential')
```

</details>

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
