# Optimization Plan

- Target: <1.00 ms parallel wall-clock median on Apple M3 Max, with exact answers preserved
- Current best: cand_0023 at 0.805 ms parallel and 2.34 ms sequential across seven independent official processes
- Stagnation count: 0
- Multi-agent mode: off
- Next candidate: none; target and validation gates are complete

## Active Branches

- Baseline: completed; exact answers captured and the original 1.91 ms parallel / 5.10 ms sequential baseline measured over seven processes.
- Day 8 critical path: completed; concurrent MST/top-k branches plus exact shared-threshold selection reduce the per-day median to 727 us.
- Day 10 critical path: completed; exact feasible-interval propagation reduces the per-day median to 292 us and passes a generated brute-force oracle.
- Build/runtime: stopped without bundled flag changes because structural work cleared the target with a 19.5% median margin.
- Promotion: completed; 28 release tests, exact answers, matched saved-parent screens, and seven independent official processes pass.

## Closed Branches

- Day 5 allocation-only streaming: CLOSED after cand_0003 tied/regressed in saved-parent A/B. Reopen only with a parser/lookup representation change, not merely removal of the ID Vec.
- Day 9 pair-loop fusion: CLOSED after cand_0007 left user CPU flat and worsened wall time. Reopen day 9 only with a coverage/grid representation change.
- Day 9 interval-delta coverage: CLOSED after cand_0013 regressed CPU and wall. A new premise is required before another day-9 candidate.
- Day 7 parser-route specialization: CLOSED after cand_0009 regressed 36%. Reopen day 7 only with a simulation-state representation change.
- Day 8 per-chunk top-k heaps: CLOSED after eight- and four-chunk variants increased CPU 2-3x and failed to improve matched wall time. Reopen only with a shared strong threshold or a different selection primitive.

## Escape Ladder

Stuck signal:
Escape operator:
Divergence budget: three structural probes before any micro-tuning
Divergence probes: Day 8 work-graph split; Day 10 incremental residual primitive; compiler/scheduler route
Basin memory:
Diversity map: work graph / search primitive / build-and-schedule
Operator credit:
New-hill commitment:
Controlled regression allowed:
Anti-revisit rule:
Aspiration rule:
Kill criterion: reject any candidate that fails exact answers/tests or lacks a repeatable matched A/B signal; authoritative promotion still requires clean parallel wall time
