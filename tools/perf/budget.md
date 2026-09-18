# Performance budget — aegis

| Verb | Canonical input | Budget (median) | Tolerance |
|------|------------------|------------------|-----------|
| `aegis scan` | 10,000 LoC source file (synthetic) | 600 ms | ±25% |

## How to run the bench

```bash
cargo test --release perf_aegis_scan_within_budget -- --nocapture
```

## When to update the budget

Update the budget only after a documented change to the verb's algorithm
or to the canonical input. A budget change without an algorithm change
is a regression hiding in plain sight — review will reject it.

## What the bench does NOT measure

- Cold-start latency (process spawn, dynamic linking). Out of scope.
- Network latency. Out of scope (the bench is offline).
- Memory ceiling. §14 covers resource-envelope tests separately.
