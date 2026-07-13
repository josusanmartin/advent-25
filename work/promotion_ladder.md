# Promotion Ladder

Every promoted candidate should pass the gating steps in order:

1. `apply_or_build`: patch applies, code builds, or the candidate artifact can be loaded.
2. `correctness`: required correctness/reference/shape/seed checks pass.
3. `authoritative_metric`: the official metric improves outside the recorded noise or gate.
4. `regression_or_adversarial`: targeted regressions, hidden-risk cases, or no-exploit checks pass when applicable.
5. `fresh_verifier`: an independent/fresh retest sees only the artifact, contract, and commands.
6. `promote`: update `work/best.md`, `work/state.json`, ledgers, and dashboard.

Advisory steps such as profiles, local screening, style, and implementation neatness can explain a candidate but cannot promote it.
