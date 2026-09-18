# Aegis

**PQC migration SDK.** Scans for RSA/ECC usage. Plans the migration. Ships the replacement primitives.

**Status:** pre-release scaffold (2026-09-17). No source code yet.

## What it does

1. Find every RSA / ECC / ECDSA / DH / Ed25519 call site in your C / C++ / Rust code.
2. Produce an OMB M-26-15-aligned migration plan.
3. Emit ML-KEM-768 / ML-DSA-65 replacements with the right EVP_PKEY shims.
4. Gate CI on `pkey_algo ∈ {ML-KEM, ML-DSA, hybrid}`.

## Why

- EO 14412 (June 2026): federal HVAs must use PQC for key establishment by Dec 31, 2030.
- FAR Council rule (proposed Dec 2026) cascades to covered federal contractors.
- RSA / ECC deprecated 2030, disallowed 2035.
- Federal integrators and edge-platform companies (Cloudflare / Akamai / Zscaler / F5) need this; nobody ships it open.

## Commercial plane

PQC Migration Concierge for federal integrators. FIPS-validation-track offering.

## License

Apache-2.0.
