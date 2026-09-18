# Aegis

[![CI](https://github.com/studio2201/aegis/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/studio2201/aegis/actions/workflows/ci.yml)
[![Release](https://img.shields.io/badge/version-v0.2.4-blue.svg)](https://github.com/studio2201/aegis/releases)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![Pure std::](https://img.shields.io/badge/pure-std%3A%3A-success.svg)](https://studio2201.com)
[![Reproducible](https://img.shields.io/badge/reproducible-OK-brightgreen.svg)](tools/dev/repro.sh)
[![Max LOC](https://img.shields.io/badge/max%20LOC-%E2%89%A4256-brightgreen.svg)](https://studio2201.com)

[![PQC Readiness](https://img.shields.io/badge/PQC-Quantum--Safe-blueviolet.svg)](https://studio2201.com/aegis)
[![OMB M-26-15](https://img.shields.io/badge/OMB%20M--26--15-COMPLIANT-brightgreen.svg)](https://studio2201.com/aegis)
[![NIST Standards](https://img.shields.io/badge/NIST-FIPS%20203%20%7C%20204-blue.svg)](https://studio2201.com/aegis)
[![Algorithms](https://img.shields.io/badge/algorithms-ML--KEM--768%20%7C%20ML--DSA--65-blueviolet.svg)](https://studio2201.com/aegis)
[![Classical Crypto](https://img.shields.io/badge/classical%20crypto-0%20legacy-brightgreen.svg)](https://studio2201.com/aegis)

**PQC migration SDK & scanner.** Scans classical RSA/ECC cryptography, plans in OMB M-26-15 format, and emits ML-KEM-768 / ML-DSA-65 replacements.

## Quick Start

```bash
# Install via studio2201 installer
curl -fsSL https://studio2201.com/install.sh | sh -s aegis

# Scan source code for classical cryptography
aegis scan .

# Run system diagnostics
aegis doctor
```

## GitHub Action Usage

Scan pull requests for classical crypto patterns before merging:

```yaml
- name: Aegis PQC Migration Scanner
  uses: studio2201/aegis@master
  with:
    path: '.'
    format: 'text'
```

## What it does

- Scans source trees for RSA, ECC, and EVP_PKEY calls.
- Emits OMB M-26-15 compliance roadmaps with risk levels and target completion dates.
- Emits drop-in ML-KEM-768 key encapsulation and ML-DSA-65 digital signature shims.

## CLI Commands

- `aegis scan [path]` — Scan source code for vulnerable crypto
- `aegis plan [path]` — Generate OMB M-26-15 migration plan
- `aegis shim [rust|c]` — Emit PQC shims for ML-KEM and ML-DSA
- `aegis doctor` — Run 7-point system diagnostics
- `aegis update` / `aegis upgrade` — Self-update binary
- `aegis -h` / `--help` — Show help
- `aegis -V` / `--version` — Show version

## Badges & Status

Certify post-quantum cryptography readiness and federal OMB compliance:

```markdown
<!-- Post-Quantum Cryptography Readiness Badge -->
[![PQC Readiness](https://img.shields.io/badge/PQC-Quantum--Safe-blueviolet.svg)](https://studio2201.com/aegis)

<!-- Federal OMB M-26-15 Compliance Badge -->
[![OMB M-26-15](https://img.shields.io/badge/OMB%20M--26--15-COMPLIANT-brightgreen.svg)](https://studio2201.com/aegis)
```

## Why

- Dec 31, 2030 is the federal OMB deadline for post-quantum cryptography migration.
- Pure Rust, `std::` only. Zero crates.io dependencies. Strictly <= 256 LOC per source file.

## License

Apache-2.0.
