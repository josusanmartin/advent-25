# Advent of Code 25 in 2.1ms (Rust)

Highly optimized Advent of Code 2025 solutions with a performance-first runner.
All implemented days (1-12) execute in just 2.1ms total in release mode.

This repository is an experiment and a learning project: everything here is written in Rust as a way
to learn the language. I did not have prior Rust experience and built these solutions with help from
Claude Code and Codex.

## Performance

| Day | Time |
|-----|------|
| 1 | 41µs |
| 2 | 16µs |
| 3 | 139µs |
| 4 | 329µs |
| 5 | 34µs |
| 6 | 94µs |
| 7 | 41µs |
| 8 | 1.65ms |
| 9 | 1.08ms |
| 10 | 2.75ms |
| 11 | 189µs |
| 12 | 248µs |
| **Total** | **6.6ms** |

*Average of 100 runs on Apple M3 Pro, `cargo run --release -- all seq`*

Wall-clock time (average of 100 runs, Apple M3 Pro):
- **Parallel (default)**: 2.1ms
- **Sequential**: 5.6ms

The table total (6.6ms) is the sum of individual day timings, while wall-clock (5.6ms) measures
start-to-finish execution. The difference (~1ms) is due to measurement overhead: each day's timer
includes `Instant::now()` calls that slightly overestimate duration, whereas the wall-clock timer
runs continuously and captures actual elapsed time more accurately.

<details>
<summary>Benchmark script</summary>

```python
import subprocess
import re
from collections import defaultdict

day_times = defaultdict(list)

for _ in range(100):
    result = subprocess.run(
        ['./target/release/advent-25', 'all', 'seq'],
        capture_output=True, text=True
    )
    output = result.stdout + result.stderr
    for m in re.finditer(
        r'Execution time \(day (\d+)\): ([\d.]+)(µs|ms|ns)', output
    ):
        day = int(m.group(1))
        val = float(m.group(2))
        unit = m.group(3)
        if unit == 'ms':
            val *= 1000
        elif unit == 'ns':
            val /= 1000
        day_times[day].append(val)

total = 0
for d in range(1, 13):
    if day_times[d]:
        avg = sum(day_times[d]) / len(day_times[d])
        total += avg
        if avg >= 1000:
            print(f'| {d} | {avg/1000:.2f}ms |')
        else:
            print(f'| {d} | {int(round(avg))}µs |')

print(f'| **Total** | **{total/1000:.1f}ms** |')
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
