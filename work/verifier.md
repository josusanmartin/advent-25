# Fresh Verifier Gate

Use this gate before updating `work/best.md` for any meaningful promotion.

- Recreate or reset the environment when practical.
- Carry only the candidate artifact or diff, the validation command, and the recorded contract across the boundary.
- Rerun correctness before the authoritative metric.
- Treat local screening, profile deltas, and candidate rationale as advisory.
- If a fresh environment is unavailable, record the limitation in the candidate result and run the cleanest independent retest available.
- Do not expose credentials or unrelated host files to untrusted/generated target code.

Verdict values: `PASS`, `FAIL`, `INCONCLUSIVE`, or `SKIPPED_WITH_LIMITATION`.
