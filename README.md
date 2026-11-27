# Aegis

<div align="center">

[![Release](https://img.shields.io/badge/version-v0.2.13-blue.svg)](https://github.com/studio2201/aegis/releases)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

| Security Pillar | Verification Badge |
| :--- | :---: |
| **Platform Standard** | [![secured by studio2201][b-studio]][u-home] |
| **Credential Defense** | [![snip][b-snip]][u-snip] |
| **Supply Chain Surface** | [![vigil][b-vigil]][u-vigil] |
| **Post-Quantum Cryptography** | [![aegis][b-aegis]][u-aegis] |
| **Build Provenance & SLSA** | [![proven][b-proven]][u-proven] |
| **Repository Governance** | [![boneyard][b-boneyard]][u-boneyard] |

[b-studio]: https://img.shields.io/badge/secured%20by-studio2201-1b5e4a?logo=shield
[u-home]: https://studio2201.com
[b-snip]: https://img.shields.io/badge/snip-0%20secrets-1b5e4a?logo=shield
[u-snip]: https://studio2201.com/snip
[b-vigil]: https://img.shields.io/badge/vigil-0%20dependencies-1b5e4a?logo=shield
[u-vigil]: https://studio2201.com/vigil
[b-aegis]: https://img.shields.io/badge/aegis-PQC%20compliant-1b5e4a?logo=shield
[u-aegis]: https://studio2201.com/aegis
[b-proven]: https://img.shields.io/badge/proven-HMAC--SHA256%20verified-1b5e4a?logo=shield
[u-proven]: https://studio2201.com/proven
[b-boneyard]: https://img.shields.io/badge/boneyard-maintained-1b5e4a?logo=shield
[u-boneyard]: https://studio2201.com/boneyard

</div>

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
Add a GitHub Actions workflow to this repository at .github/workflows/aegis.yml using studio2201/aegis@v1:
- Name: aegis
- Triggers: pull_request, push to default branch (master/main), and workflow_dispatch.
- Concurrency: group ${{ github.workflow }}-${{ github.ref }} with cancel-in-progress on pull_request.
- Permissions: contents: read, checkout with fetch-depth: 0.
- Action: run studio2201/aegis@v1.
- Artifacts: upload aegis_summary.md using actions/upload-artifact@v6 with if: always().
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
      - uses: actions/checkout@v6
        with: { fetch-depth: 0 }
      - uses: studio2201/aegis@v1
      - uses: actions/upload-artifact@v6
        if: always()
        with: { name: aegis-report, path: aegis_summary.md, if-no-files-found: ignore }
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
# Install from source (or download release tarballs from GitHub Releases)
cargo install --git https://github.com/studio2201/aegis

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
