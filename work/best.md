# Best Known State

## Objective
Mode: local deterministic CPU optimization
Target: Reduce the parallel solver-only wall-clock median for all implemented days (1-12) below 1.00 ms on the local Apple M3 Max while preserving exact answers
Objective source: explicit user request (2026-07-10)
Authoritative metric: parallel_wall_median_ns from the immutable 100-iteration benchmark, promoted by the median of at least five clean independent processes; lower is better
Baseline: cand_0000 measured 1.91 ms parallel and 5.10 ms sequential; the protected pre-cand_0023 state reproduced at 0.941 ms parallel and 2.71 ms sequential
Ambitious target: <1,000,000 ns parallel wall median; sequential wall time and per-day medians are regression guardrails
Budget / stopping rule: Continue through structural Day 8 and Day 10 critical-path changes, then measured compiler/scheduler tuning; stop only at verified <1 ms or when every correctness-safe structural branch is measured and closed
Validation: cargo test --release; exact embedded-input answers unchanged; repeated cargo run --release --bin benchmark
Progress chart: on
Fresh-run isolation: on
Multi-agent mode: off

## Current Best
Current best stable and benchmark: cand_0023, 805,000 ns parallel median and 2.34 ms sequential median across seven independent official processes
Per-day median-of-run medians (us): d1 18, d2 9, d3 124, d4 278, d5 31, d6 90, d7 38, d8 727, d9 527, d10 292, d11 63, d12 119
Why it wins: retains all prior lane wins, runs Day 8's MST/top-k branches concurrently, cuts Day 10 search through exact feasible intervals, and replaces Day 8's all-pairs heap maintenance with a shared exact threshold plus one selection

Retained lane evidence: d1 54.3% median reduction; d2 25.9% CPU; d3 1.14x; d4 12.5% CPU; d5 8.1% CPU; d6 10.2% CPU; d7 20.1% CPU; d8 26.6% CPU; d9 15.5% CPU; d10 5.2% one-thread CPU and 1.25x default-Rayon wall; d11 2.41x parser plus 15.7% DP CPU; d12 40.7% CPU.
Combined verifier: all 28 release tests pass, generated Day 8 and Day 10 oracle tests pass, output/answers.txt retains SHA-256 eed3f681c172a8b5f98f21b1058afd38375209b80ea6f174df1e80506b7fa0cb, and all seven final parallel process medians are below 1 ms.

Sub-1ms phase lane bests: cand_0018 split Day 8's MST/top-k tail; cand_0020 cut Day 10 user CPU 4.7-5.0x; cand_0023 cut Day 8 Part 1 CPU 22.9% and achieved a clean 0.805 ms global parallel median.

## Boundaries
Editable files: all solver sources plus README.md as explicitly authorized; Cargo.toml release settings only as separate measured candidates
Immutable files: inputs/*.txt; src/bin/benchmark.rs; existing test cases and expected values; output/answers.txt reference contents; work harness schemas/scripts
Draft patch only: no
Untrusted code execution boundary: clean env, no credentials, restricted egress when possible

## Bottleneck
Profiling plan: repeated built-in per-day benchmark on the target machine is authoritative; use pprof/sampling or generated assembly only to choose candidates. Profiles cannot promote changes. Save raw timings under work/raw_logs/ and profiles under work/profiles/.
Confirmed bottlenecks: Day 8 and Day 10 were the parallel-tail co-binders; structural changes shortened both enough to clear the target. Day 9 is the next floor if optimization resumes.
Exhausted branches:
Open directions: none required by the current contract; compiler/scheduler tuning remains deliberately unbundled because the verified 0.805 ms result satisfies the stopping rule
