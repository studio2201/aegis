# Aegis

[![CI](https://github.com/studio2201/aegis/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/studio2201/aegis/actions/workflows/ci.yml)
[![Release](https://img.shields.io/badge/version-v0.2.5-blue.svg)](https://github.com/studio2201/aegis/releases)
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

## Why This Matters & Authoritative Mandates

### 1. The Quantum Decryption Threat & "Harvest Now, Decrypt Later"
Every standard asymmetric encryption and digital signature algorithm in common use today (RSA-2048/4096, ECDSA, ECDH) will be broken by Shor's algorithm once quantum computers reach scale. Adversaries are actively intercepting and storing encrypted traffic today to decrypt it retrospectively once hardware arrives.
- **[White House OMB M-26-15](https://www.whitehouse.gov/wp-content/uploads/2022/11/M-23-02-M-Memo-on-Migrating-to-Post-Quantum-Cryptography.pdf)**: Federal directive mandating migration of vulnerable cryptography by December 31, 2030 across government and contractor software.
- **[National Security Memorandum 10 (NSM-10)](https://www.whitehouse.gov/briefing-room/statements-releases/2022/05/04/national-security-memorandum-on-promoting-united-states-leadership-in-quantum-computing-while-mitigating-risks-to-vulnerable-cryptographic-systems/)**: Mandates executive branch transition to post-quantum cryptography.
- **[NIST FIPS 203 (ML-KEM)](https://csrc.nist.gov/pubs/fips/203/final) & [FIPS 204 (ML-DSA)](https://csrc.nist.gov/pubs/fips/204/final)**: Official federal post-quantum cryptographic standards published in August 2024.
- **[NSA CNSA 2.0 Cybersecurity Advisory](https://media.defense.gov/2022/Sep/07/2003071834/-1/-1/0/CSA_CNSA_2.0_ALGORITHMS_.PDF)**: Sets mandatory post-quantum algorithm suite selection requirements.

## How It Works Under the Hood

1. **Static AST Call Site Scanner (`src/scan.rs`)**: Performs high-speed lexical analysis searching for RSA key generation, classic ECC curves (secp256k1, prime256v1), OpenSSL `EVP_PKEY` pointers, and vulnerable hashes.
2. **OMB M-26-15 Compliance Planner (`src/plan.rs`)**: Generates 3-phase roadmaps mapping identified sites to statutory milestone dates: Phase 1 Inventory (2026), Phase 2 Hybrid KEM (2028), and Phase 3 Full PQC (2030).
3. **Drop-in Quantum-Safe Shims (`src/migrate.rs`)**: Emits pure standard Rust and C shims replacing legacy primitives with ML-KEM-768 encapsulation and ML-DSA-65 signatures.
4. **Zero Crates & Bit-Reproducibility**: Pure `std::` Rust with strict $\le 256$ LOC per file, verified bit-reproducible via `tools/dev/repro.sh`.

## Quick Start

```bash
# Install via studio2201 installer
curl -fsSL https://studio2201.com/install.sh | sh -s aegis

# Scan source code for classical cryptography
aegis scan .

# Generate federal OMB M-26-15 migration plan
aegis plan .

# Emit drop-in PQC replacement shims
aegis shim rust

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

## License

Apache-2.0.
