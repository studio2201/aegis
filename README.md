# Aegis

[![aegis][b-aegis]][ci-aegis]
[![Release](https://img.shields.io/badge/version-v0.2.10-blue.svg)](https://github.com/studio2201/aegis/releases)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

[b-aegis]: https://img.shields.io/github/actions/workflow/status/studio2201/aegis/aegis.yml?label=aegis&logo=shield
[ci-aegis]: https://github.com/studio2201/aegis/actions/workflows/aegis.yml

**Post-quantum cryptography migration SDK.** Scans source trees for classical asymmetric cryptography
(RSA, ECC), plans transition roadmaps in OMB M-26-15 format, and emits drop-in NIST PQC replacement shims.

## Why This Action Is Needed

### The Quantum Threat & Statutory Mandates
Standard asymmetric encryption and signatures (RSA-2048/4096, ECDSA, ECDH) are provably vulnerable
to Shor's algorithm once cryptanalytically relevant quantum computers scale. Adversaries are actively
harvesting encrypted enterprise traffic today for retrospective decryption.
- **[White House OMB M-26-15](https://www.whitehouse.gov/wp-content/uploads/2022/11/M-23-02-M-Memo-on-Migrating-to-Post-Quantum-Cryptography.pdf)**:
  Federal directive mandating complete transition of vulnerable cryptography across critical infrastructure
  by December 31, 2030.
- **[NIST FIPS 203 (ML-KEM)](https://csrc.nist.gov/pubs/fips/203/final) & [FIPS 204 (ML-DSA)](https://csrc.nist.gov/pubs/fips/204/final)**:
  Federal post-quantum standards establishing ML-KEM and ML-DSA as replacements for RSA and ECC.
- **Automated CI Gates vs Manual Discipline**: Software engineers and AI models continue generating legacy
  RSA/ECC boilerplates by default. Manual code reviews rarely evaluate cryptographic obsolescence.
  Aegis provides an automated CI gate that intercepts legacy cryptographic primitives and enforces PQC migration.

## Autonomous Agent Integration

Deploy Aegis into your CI pipeline using your AI coding assistant or directly add the workflow below.

### Prompt for your AI Agent

Copy and paste this prompt to Cursor, Claude Code, Copilot Workspace, or Devin:

```text
Add a GitHub Actions workflow to this repository at .github/workflows/aegis.yml using studio2201/studio2201@v1:
- Name: aegis
- Triggers: pull_request, push to default branch (master/main), and workflow_dispatch.
- Concurrency: group ${{ github.workflow }}-${{ github.ref }} with cancel-in-progress on pull_request.
- Permissions: contents: read, checkout with fetch-depth: 0.
- Action: run studio2201/studio2201@v1 with tools: 'aegis' and fail-on: 'block'.
- Artifacts: upload aegis_report.md using actions/upload-artifact@v4 with if: always().
- Zero False Positives: @v1 provisions Aegis v0.2.10+ (skips comments & docstrings).
- README Badge: add dynamic Shields.io workflow status badge to top of README.md (replace <owner>/<repo>):
  [![aegis][b-aegis]][ci-aegis]
  [b-aegis]: https://img.shields.io/github/actions/workflow/status/<owner>/<repo>/aegis.yml?label=aegis&logo=shield
  [ci-aegis]: https://github.com/<owner>/<repo>/actions/workflows/aegis.yml
```

### GitHub Actions Workflow

Commit this complete, production-ready workflow at `.github/workflows/aegis.yml`:

```yaml
name: aegis
on:
  push: { branches: [ master, main ] }
  pull_request: { branches: [ master, main ] }
  workflow_dispatch:
concurrency:
  group: ${{ github.workflow }}-${{ github.ref }}
  cancel-in-progress: ${{ github.event_name == 'pull_request' }}
permissions: { contents: read }
jobs:
  aegis:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with: { fetch-depth: 0 }
      - uses: studio2201/studio2201@v1
        with: { tools: 'aegis', fail-on: 'block' }
      - uses: actions/upload-artifact@v4
        if: always()
        with: { name: aegis-report, path: aegis_report.md, if-no-files-found: ignore }
```

## How It Works Under the Hood

1. **Static AST Call Site Scanner (`src/scan.rs`)**: Performs lexical analysis searching for RSA key
   generation, classic ECC curves (secp256k1, prime256v1), OpenSSL `EVP_PKEY` pointers, and vulnerable hashes.
2. **OMB M-26-15 Compliance Planner (`src/plan.rs`)**: Generates 3-phase roadmaps mapping identified
   sites to statutory dates: Phase 1 Inventory (2026), Phase 2 Hybrid KEM (2028), and Phase 3 Full PQC (2030).
3. **Drop-in Quantum-Safe Shims (`src/migrate.rs`)**: Emits pure standard Rust and C shims replacing legacy
   primitives with ML-KEM-768 encapsulation and ML-DSA-65 signatures.
4. **Zero Crates & Bit-Reproducibility**: Pure `std::` Rust with strict $\le 256$ LOC per file, verified
   bit-reproducible via `tools/dev/repro.sh`.

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
