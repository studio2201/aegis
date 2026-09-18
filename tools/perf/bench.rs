// tools/perf/bench.rs — §18 bench harness for aegis.
//
// Lives as a `#[test]` in tests/integration.rs via `include!` so it is
// exercised by `cargo test --release perf_aegis_scan_within_budget`.
// Honors §18: std::time only, median-of-5, line-oriented output.
//
// Budget: aegis scan on 10k LoC source ≤ 600 ms median ±25%.

use std::time::Instant;

#[test]
fn perf_aegis_scan_within_budget() {
    let source = synth_source_with_n_loc(10_000);
    let budget_ms: f64 = 600.0;
    let tolerance: f64 = 0.25;
    let ceiling_ms = budget_ms * (1.0 + tolerance);

    // Warm-up: prime allocator and caches, but do not measure.
    let _ = aegis::scan(&source, &policy());

    // Five timed runs.
    let mut samples: Vec<f64> = Vec::with_capacity(5);
    for _ in 0..5 {
        let t = Instant::now();
        let _ = aegis::scan(&source, &policy());
        let elapsed_ms = t.elapsed().as_secs_f64() * 1000.0;
        samples.push(elapsed_ms);
    }
    samples.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let median_ms = samples[2]; // median of 5

    let pass = median_ms <= ceiling_ms;
    println!(
        "verb=aegis_scan median_ms={:.3} budget_ms={:.0} pass={}",
        median_ms, budget_ms, pass
    );
    assert!(
        pass,
        "aegis_scan regression: median {:.1}ms > ceiling {:.1}ms",
        median_ms, ceiling_ms
    );
}

// Fixture builders — synthetic, committed (per §18-C5).
fn synth_source_with_n_loc(n: usize) -> aegis::Source {
    let mut content = String::with_capacity(n * 40);
    for i in 0..n {
        if i % 250 == 0 {
            content.push_str("let key = RSASSA_PKCS1_v1_5::new();\n");
        } else if i % 400 == 0 {
            content.push_str("let curve = secp256k1::verify(&msg);\n");
        } else {
            content.push_str("let val = compute_safe_operation(i);\n");
        }
    }
    aegis::Source {
        path: "synthetic_source.rs".to_string(),
        content,
    }
}

fn policy() -> aegis::Policy {
    aegis::Policy::default()
}
