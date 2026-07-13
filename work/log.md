# Optimization Log

## 2026-07-10T08:08:06Z :: harness_boot

- objective: Reduce pure solver execution time for every implemented Advent of Code day (1-12), minimizing sequential total median while preserving each day's answers and avoiding per-day median regressions outside noise
- authoritative metric: sequential_total_median_ns (lower is better; per-day medians are guardrails)
- baseline: unknown, reproduce first
- validation: cargo test --release; exact embedded-input answers unchanged; repeated cargo run --release --bin benchmark
- progress chart: on
- multi-agent mode: off
- decision: BOOTSTRAP
- learning: harness initialized before candidate work

## 2026-07-10T08:10:04Z :: cand_0000

- candidate: checked-in release implementation
- parent: none
- mode: SEED
- branch: baseline
- hypothesis: establish reproducible correctness and timing guardrails before edits
- expected signal: README-scale solver times with stable median-of-run values
- validation command: cargo test --release
- benchmark command: cargo run --release --bin benchmark, seven processes of 100 iterations
- result: 5.09 ms sequential total median; 1.91 ms parallel median
- per-day medians (us): 35, 11, 131, 302, 31, 87, 39, 1770, 836, 1500, 170, 191
- correctness: PASS, 26/26 release tests
- usage snapshot UTC: 2026-07-10T08:10:04Z
- tokens_total: unknown (get_goal returned no active goal)
- tokens_delta: unknown
- stability: seven total medians ranged from 5.07 ms to 5.28 ms
- improved best: seed baseline
- decision: PROMOTE
- learning: days 8, 10, and 9 dominate; fast-day formatter resolution is coarse
- raw result paths: work/raw_logs/baseline_1.txt through work/raw_logs/baseline_7.txt

## 2026-07-10T08:26:34Z :: cand_0001

- candidate: fused day-1 both-parts scan
- parent: cand_0000 / 7d644f21bf8948a844b39b030cb6dc7ad39bd525
- mode: EXPLORE
- mechanism: work deletion / algebraic fusion
- hypothesis: remove the duplicate 16.6 KB parse from the Both path
- expected signal: at least 15% lower day-1 median
- validation command: cargo test --release; exact day-1 answer comparison
- benchmark command: five clean cargo run --release --bin benchmark processes
- result: 4.87 ms total median; day 1 16 us versus 35 us baseline
- screening: direct A/B was 288.4 ms separate versus 131.1 ms fused for 10,000 calls (2.20x)
- correctness: PASS, 26/26 release tests and answers 964 / 5872
- usage snapshot UTC: 2026-07-10T08:26:34Z
- tokens_total: unknown (get_goal returned no active goal)
- tokens_delta: unknown
- stability: day 1 was 16 us in all five authoritative runs
- improved best: yes
- decision: PROMOTE
- learning: shared parsing is a high-confidence lane for other Both implementations
- raw result paths: work/raw_logs/cand_0001_clean_1.txt through work/raw_logs/cand_0001_clean_5.txt
- note: an earlier run under system load average 45 was invalidated and excluded

## 2026-07-10T08:29:04Z :: cand_0002

- parent: cand_0001
- mode/mechanism: TUNE; work deletion / representation change
- change: replace day-3 part-1 stack with largest-prior-digit constant state
- correctness: PASS, 26 tests and exact answers 17085 / 169408143086082
- saved-parent A/B: 197.5 ms -> 173.7 ms for 3,000 solves, 1.14x faster
- authoritative result: 4.84 ms total median; day 3 124 us -> 114 us
- usage snapshot UTC: 2026-07-10T08:29:04Z; tokens unknown because no active goal
- decision: PROMOTE
- raw results: work/raw_logs/cand_0002_1.txt through work/raw_logs/cand_0002_5.txt

## 2026-07-10 :: cand_0003

- parent: cand_0002
- hypothesis: stream day-5 IDs after merging ranges to remove allocation and a traversal
- correctness: PASS, 26 tests and exact answers
- saved-parent A/B: parent 802.0 +/- 33.8 ms; candidate 808.8 +/- 49.6 ms for 10,000 solves
- authoritative metric: not run because the differential screen showed no positive signal
- decision: REJECT; source reverted to cand_0002
- learning: the ID Vec is not an active bottleneck; close allocation-only variants

## 2026-07-10T08:37:30Z :: cand_0004

- parent: cand_0002 (cand_0003 rejected/reverted)
- mechanism: compact label representation and one-pass day-11 adjacency construction
- correctness: PASS, 26 tests and exact answers 423 / 333657640517376
- isolated A/B: 2.41x faster over 3,000 solves
- adjacent official day-11 medians: parent 585/542/720 us; candidate 244/268/307 us under shared load
- global score: withheld; full runs were invalidated at system load >60
- decision: PROMOTE day-11 lane; keep cand_0002's 4.84 ms as protected global score
- usage: unknown, no active goal

## 2026-07-10T08:43:02Z :: cand_0006

- parent: cand_0005
- mechanism: stream day-12 regions with one reused count buffer
- correctness: PASS, 26 tests and exact answer 510
- saved-parent comparison: user CPU 0.846 s -> 0.502 s for 2,000 solves (40.7%)
- wall metric: deferred because unrelated load dominated scheduling
- decision: KEEP VARIANT pending final clean wall verification

## 2026-07-10 :: cand_0007

- parent: cand_0006
- hypothesis: fold day-9 part-1 maximum into part-2's pair loop
- correctness: PASS
- saved-parent comparison: user CPU 1.044 s -> 1.050 s for 500 solves; wall time regressed
- decision: REJECT; source reverted
- learning: the added max dependency harms the hot loop enough to offset the deleted standalone scan

## 2026-07-10T08:49:01Z :: cand_0008

- parent: cand_0006 (cand_0007 rejected/reverted)
- mechanism: borrow equal-width day-6 rows with Cow padding fallback
- correctness: PASS, 26 tests and exact answers
- differential: user CPU 456.0 ms -> 409.6 ms for 3,000 solves; mean wall speedup 1.19x
- decision: KEEP VARIANT pending clean global score

## 2026-07-10T09:24:59Z :: cand_0017

- parent: cand_0016
- mechanism: flatten profiled-hot day-9 green byte grid
- correctness: PASS, 26 tests and exact answers
- differential: user CPU 1.175 s -> 0.993 s for 500 solves (15.5%); wall speedup 2.35x under heavy load
- decision: KEEP VARIANT pending clean global score

## 2026-07-10T09:33:49Z :: final_verification

- code: cand_0017 working tree
- formatting: PASS for all changed Rust files; git diff --check PASS
- correctness: PASS, cargo test --release (26/26), exact all-days sequential output matches output/answers.txt
- extra verifier: day-2 exhaustive brute-force comparison for values 1..999999 PASS
- combined A/B versus cand_0002: default Rayon user CPU 0.419 s -> 0.367 s for 30 loops; one-thread 0.413 s -> 0.360 s
- wall-time verdict: INCONCLUSIVE because unrelated host load ranged from 100 to 196
- clippy: strict -D warnings is not a repository baseline; 38 existing style lints across original hot loops and APIs
- usage: unknown, no active goal
- decision: HANDOFF with clean official benchmark limitation

## 2026-07-10T09:18:27Z :: cand_0016

- parent: cand_0015
- mechanism: sparse active-column frontier for day 7
- evidence: 43.7 active columns on average out of 141
- correctness: PASS, 26 tests and exact answers
- differential: user CPU 506.7 ms -> 404.8 ms for 10,000 solves; mean wall speedup 1.33x
- decision: KEEP VARIANT pending clean global score

## 2026-07-10T09:16:18Z :: cand_0015

- parent: cand_0014
- mechanism: compact day-5 ranges in place after sorting
- correctness: PASS, 26 tests and exact answers
- differential: user CPU 722.0 ms -> 663.2 ms for 10,000 solves (8.1%)
- decision: KEEP VARIANT pending clean global score

## 2026-07-10 :: cand_0013

- parent: cand_0012
- hypothesis: construct day-9 prefix directly from row interval deltas
- correctness: PASS
- differential: user CPU 1.040 s -> 1.078 s for 500 solves; wall time also regressed
- decision: REJECT; source reverted
- learning: serial active-count prefix construction loses to the materialized byte grid

## 2026-07-10T09:13:18Z :: cand_0014

- parent: cand_0012 (cand_0013 rejected/reverted)
- mechanism: early return when day-10 feasible cost reaches max-joltage lower bound
- profile basis: hot samples concentrated in free-variable enumeration/back-substitution
- correctness: PASS, 26 tests and exact answers
- differential: 1.25x multithreaded wall speedup; one-thread user CPU 3.353 s -> 3.180 s for 1,000 solves
- decision: KEEP VARIANT pending clean global score

## 2026-07-10T09:02:02Z :: cand_0012

- parent: cand_0011
- mechanism: dense 24-byte day-8 Prim frontier records with i32 coordinates
- correctness: PASS, 26 tests and exact answers
- differential: user CPU 1.157 s -> 0.849 s for 300 solves; mean wall speedup 1.27x
- decision: KEEP VARIANT pending clean global score

## 2026-07-10T08:58:55Z :: cand_0011

- parent: cand_0010
- mechanism: replace day-2 half-representation division with even-repeat invariant
- correctness: PASS, 26 tests, exact answers, and exhaustive brute-force comparison for 1..999999
- differential: user CPU 0.722 s -> 0.535 s for 30,000 solves; mean wall speedup 1.64x
- decision: KEEP VARIANT pending clean global score

## 2026-07-10 :: cand_0009

- parent: cand_0008
- hypothesis: fixed-stride direct grid route for day 7
- correctness: PASS
- differential: parent 132.7 ms / 127.6 ms user; candidate 180.1 ms / 172.6 ms user for 5,000 solves
- decision: REJECT; source reverted
- learning: the existing combined parser scan is cheaper than route validation plus direct iteration

## 2026-07-10T08:55:11Z :: cand_0010

- parent: cand_0008 (cand_0009 rejected/reverted)
- mechanism: zero-padded day-4 grid with fixed neighbor offsets
- correctness: PASS, 26 tests and exact answers
- differential: user CPU 1.110 s -> 0.971 s for 2,000 solves; mean wall speedup 1.29x
- decision: KEEP VARIANT pending clean global score

## 2026-07-10T08:39:36Z :: cand_0005

- parent: cand_0004
- mechanism: fuse seven day-11 target-specific DAG traversals into one categorized memo
- correctness: PASS, 26 tests and exact puzzle answers
- saved-parent comparison: user CPU 302.9 ms -> 255.5 ms for 2,000 solves (15.7%); wall samples too noisy under shared load
- decision: KEEP VARIANT pending final clean wall-time verification
- usage: unknown, no active goal

## 2026-07-10T10:50:53Z :: sub_1ms_phase_start

- objective: parallel solver-only wall median below 1.00 ms on Apple M3 Max
- authoritative metric: median of at least five clean independent 100-iteration benchmark-process parallel medians
- protected clean checkpoint: cand_0002 at 1.77 ms parallel median
- current stable code: cand_0017, clean current wall baseline pending
- correctness: cargo test --release plus exact output/answers.txt comparison
- gap: Day 8 and Day 10 parallel-tail co-binders; current host load 263 creates an evidence gap
- editable surface: Day 8, Day 10, parallel composition, and separately measured release settings
- immutable: benchmark harness, inputs, expected answers, existing tests, README until final verified measurements
- multi-agent mode: off
- decision: begin structural work with cand_0018; use matched CPU/component screens while load is high, but require clean official wall time for promotion

## 2026-07-10T10:57:13Z :: cand_0018

- candidate: split Day 8 exact MST and exact top-1,000 edge selection into two Rayon branches
- parent: cand_0017 / Day 8 hash 57948ac8bf73cab98cf6d91de17a79c91c27bbb167d97292f8ab27dc25a62d52
- mode/mechanism: EXPLORE; resource transfer / tail reshaping
- expected signal: each branch below the fused Day 8 latency, with matched whole-suite parallel improvement
- correctness: PASS, 26/26 release tests including Day 8 examples and puzzle input
- component screen: part 1 user CPU 1.35 s -> 0.89 s and part 2 1.31 s -> 0.71 s for 500 calls
- matched 12-thread screen: median wall 1.25 s -> 1.12 s for 300 calls; total user CPU regressed about 18% as expected
- authoritative metric: INCONCLUSIVE; host load remained 14.79-15.56 and the official run was contaminated
- usage snapshot UTC: 2026-07-10T10:57:13Z; get_goal returned no active goal, token fields unavailable
- improved best: no stable promotion; retained latency variant
- decision: KEEP VARIANT
- learning: the two independent branches are each materially cheaper than the fused solve, but the latency win needs a quiet-host whole-suite gate
- raw result: work/raw_logs/cand_0018_screen.txt

## 2026-07-10T11:12:14Z :: cand_0019

- candidate: exact objective-slope ordering and first-feasible stop for Day 10 free-variable slices
- parent: cand_0018 / Day 10 hash e93a75fc8c75a6059da718357976bebc70295ac511b30d0ca8e79bece25e3fda
- mode/mechanism: EXPLORE; algebraic work deletion
- profile basis: 82/46/38/5 machines have 0/1/2/3 free variables; machine 167 alone owns the Day 10 tail and has a 269,225-point raw search
- correctness: PASS, 27/27 release tests including 128 generated brute-force comparisons
- matched screen: one-thread median user CPU tied at 1.07 s for 200 calls; 12-thread 1.14 s -> 1.12 s, within noise
- authoritative metric: no promotable signal; host load ranged from 21.97 to 78.46
- usage snapshot UTC: 2026-07-10T11:12:14Z; get_goal returned no active goal
- decision: KEEP VARIANT only as a characterized stepping stone
- learning: most expensive prefixes have no feasible innermost value, so stopping after the first feasible value does not remove their scans; derive the feasible interval before enumeration
- raw result: work/raw_logs/cand_0019_screen.txt

## 2026-07-10T11:15:18Z :: cand_0020

- candidate: exact pivot-box interval propagation for Day 10's innermost free variable
- parent: cand_0019 / hash a170f1942d390f0f0d2f476256d3c852a9f81247bf03742b8d64810e3f5e4d50
- mode/mechanism: TUNE; work deletion / constraint propagation
- correctness: PASS, 27 release tests, 128 generated brute-force comparisons, exact all-days answers unchanged
- matched user-CPU screen: one thread 0.55 s -> 0.11 s and 12 threads 0.56 s -> 0.12 s for 100 calls
- target-lane speedup: 4.7-5.0x; wall also improved under host load about 155
- authoritative whole-suite metric: pending clean host
- usage snapshot UTC: 2026-07-10T11:15:18Z; get_goal returned no active goal
- improved best: Day 10 lane yes; global stable promotion pending
- decision: KEEP VARIANT
- learning: empty-prefix rejection and tight admissible inner ranges remove the single-machine tail; Day 8 should now own the target
- raw result: work/raw_logs/cand_0020_screen.txt

## 2026-07-10T11:18:39Z :: cand_0021

- candidate: 16-chunk parallel top-k edge selection for Day 8
- parent: cand_0020 / Day 8 hash 679533c7466c384362d3defbc9e958aa78bb484dd474578cbc6ff756ef0df16f
- correctness: PASS, 27 release tests
- matched screen: Part 1 user CPU 0.19 s -> 0.55 s and Day 8 Both 0.34 s -> 0.70 s for 100 calls
- result: decisive CPU and loaded-host wall regression
- usage snapshot UTC: 2026-07-10T11:18:39Z; get_goal returned no active goal
- decision: REJECT
- learning: sixteen weak local thresholds and their merges erase the parallel benefit; test four chunks once
- raw result: work/raw_logs/cand_0021_screen.txt

## 2026-07-10T11:20:17Z :: cand_0022

- candidate: four-chunk retune of Day 8 parallel top-k
- parent: cand_0021 / Day 8 hash dfa3f4767fd9fa295a52b38766a84cf5016461f6b8aba18b31ba41257a6456c2
- correctness: PASS, Day 8 release tests
- matched screen: Part 1 user CPU 0.19 s -> 0.38 s and Both 0.35 s -> 0.52 s for 100 calls
- result: no wall signal and decisive CPU regression
- usage snapshot UTC: 2026-07-10T11:20:17Z; get_goal returned no active goal
- decision: REJECT; close per-chunk-heap parallel top-k
- learning: a different exact selection primitive or a shared strong threshold is required
- raw result: work/raw_logs/cand_0022_screen.txt

## 2026-07-13T08:55:22Z :: cand_0023

- candidate: exact shared-threshold top-k selection for Day 8
- parent: cand_0020 with cand_0018 Day 8 restored / Day 8 hash 679533c7466c384362d3defbc9e958aa78bb484dd474578cbc6ff756ef0df16f
- mode/mechanism: EXPLORE; work deletion / exact selection primitive
- correctness: PASS, 28 release tests including generated full-sort equivalence over n=2..80, cargo fmt, and exact answer hash unchanged
- matched screen: Part 1 user CPU 0.35 s -> 0.27 s per 500 calls; Day 8 Both wall 0.23 s -> 0.20 s per 300 calls
- clean parent baseline: seven official run medians, 0.941 ms parallel and 2.71 ms sequential
- authoritative result: seven official run medians [0.800, 0.805, 0.813, 0.996, 0.803, 0.805, 0.796] ms; median 0.805 ms parallel and 2.34 ms sequential
- original-baseline reduction: parallel 1.91 ms -> 0.805 ms (57.9%); sequential 5.10 ms -> 2.34 ms (54.1%)
- usage snapshot UTC: 2026-07-13T08:55:22Z; get_goal returned no active goal
- decision: PROMOTE; target achieved with all seven final run medians below 1 ms
- learning: a single strong exact prefix threshold avoids weak local heaps and turns the remaining all-pairs pass into cheap comparisons plus one in-place selection
- raw results: work/raw_logs/cand_0023_screen.txt and work/raw_logs/cand_0023_official_*.txt
