# Changelog — aegis

All notable changes to this project are documented here.
Format: [Keep a Changelog](https://keepachangelog.com/) 1.1.0.
This project adheres to [Semantic Versioning](https://semver.org/).

## [0.2.10] — 2026-09-19

### Fixed
- Skip block comment interior lines starting with asterisk in crypto scanner.
- Embedded canonical AI agent prompt and hardened GitHub Actions CI workflow in docs.

## [0.2.9] — 2026-09-19

### Added
- Multi-platform static binary release packaging and two-stage publish workflow.

## [0.2.8] — 2026-09-19

### Added
- Embedded Option 2 detailed governance scorecard into `README.md`.

## [0.2.7] — 2026-09-19

### Changed
- Updated README header to unified Option 1 Single Suite Badge.
- Bumped version to 0.2.7.

## [0.2.6] — 2026-09-19

### Added
- Native composite GitHub Action (`action.yml`) with sub-2s fast bootstrap via `install.sh`.
- Native `$GITHUB_STEP_SUMMARY` Markdown scorecard and PQC readiness audit reporting.
- Human-first 'Why This Action Is Needed' rationale with OMB M-26-15 and NSM-10 citations.
- Agent-first 'Prompt for your AI Agent' blocks and drop-in CI workflow YAML.

## [0.2.5] — 2026-09-18

### Added
- Expanded documentation in README with authoritative problem descriptions and citations:
  - White House OMB M-26-15 and NSM-10 post-quantum directives.
  - NIST FIPS 203 (ML-KEM) and FIPS 204 (ML-DSA) standards.
  - NSA CNSA 2.0 post-quantum algorithm transition advisory.
- Added comprehensive "How It Works Under the Hood" architectural breakdown.
- Upgraded release metadata and diagnostic baseline.

## [0.2.4] — 2026-09-18

### Added
- Tool-specific badges on README: PQC Readiness, Federal OMB M-26-15 Compliance, NIST FIPS 203/204 Standards, Algorithms (ML-KEM/ML-DSA), and Classical Crypto elimination.
- README guide for embedding PQC readiness and OMB compliance badges in repositories.
- Upgraded release metadata and diagnostic baseline.

## [0.2.0] — 2026-09-18

### Added
- Working pure `std::` Rust implementation of Aegis PQC migration scanner.
- Static scanner discovering classical cryptographic call sites: RSA, classic ECC, OpenSSL EVP_PKEY, and weak signature hashes.
- Automated OMB M-26-15 and Executive Order 14412 compliance migration plan generator.
- Drop-in replacement shims for ML-KEM-768 (FIPS 203) and ML-DSA-65 (FIPS 204).
- Policy verification gating against post-2030 cryptographic regressions.
- Standardized CLI flags: `-h/--help`, `-V/--version`, `--format`, `-o/--output`, `-q/--quiet`, `-v/--verbose`.
- Performance test verifying 10,000 lines scanned in ~1ms (budget 600ms).

## [0.1.2] — 2026-09-17

### Notes
- No content changes; aegis README had no openOODA substrate references.
  Bumped to keep cadence with the v0.1.2 doctrine-level cleanup.

## [0.1.1] — 2026-09-17

### Added
- §15 threat model: `docs/threat-model.md` (aegis-specific adversary:
  quantum-capable nation-state performing harvest-now-decrypt-later
  against RSA-encrypted traffic)
- §16 reproducible builds: `tools/dev/repro.sh` with per-host baselines
- §17 security disclosure: `SECURITY.md` pointing at GHSA tab
- §18 performance budgets: `tools/perf/budget.md` and
  `tests/integration.rs::perf_aegis_scan_within_budget` (std::time, median-of-5)

### Notes
- Pre-1.0.0: GHSA-only security advisories; CVEs reserved for 1.0.0+
- Budget defaults are first-cut placeholders, not aspirational

## [0.1.0] — 2026-09-17

### Added
- Initial scaffold: Apache-2.0 LICENSE, README, .gitignore
- One question (§0): "Which lines are keeping me RSA-encrypted past 2030?"
