//! tests/integration.rs — integration tests for aegis.
//! Includes the §18 bench harness from tools/perf/bench.rs so it runs as
//! a `#[test]` via `cargo test --release perf_aegis_scan_within_budget`.
//! Per plan decision (6): no separate [[bin]]; bench lives in the test target.

include!("../tools/perf/bench.rs");
