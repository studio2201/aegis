//! lib.rs — Aegis: PQC migration SDK and OMB M-26-15 scanner.
//! Pure std:: Rust (Edition 2021). Zero external dependencies.

pub mod migrate;
pub mod plan;
pub mod policy;
pub mod scan;

pub use migrate::{emit_ml_dsa_shim, emit_ml_kem_shim, ShimDialect};
pub use plan::{emit_json, emit_markdown, render_plan, MigrationPlan};
pub use policy::{evaluate, Policy, PolicyVerdict};
pub use scan::{load_source, scan_source, CryptoFinding, CryptoPrimitive, ScanError, ScanReport, Source};

/// Scans a source file or codebase and produces a cryptographic inventory report.
pub fn scan(source: &Source, _policy: &Policy) -> Result<ScanReport, ScanError> {
    Ok(scan_source(source))
}
