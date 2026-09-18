# Threat model — aegis

## 1. Adversary
A Federal contractor that must migrate key establishment off RSA by 2030.
Their adversaries are quantum-capable nation-state actors performing
"harvest now, decrypt later" against RSA-encrypted traffic the contractor
emits today. The attacker cannot break RSA *now*, but they can record the
ciphertext and decrypt it in five to fifteen years when they have a
sufficiently large quantum computer.

## 2. Trust boundaries
We trust: the source tree the contractor scans, the contractor's
classification of "PQC-required" vs. "PQC-optional" zones. We do not
trust: the timeline the adversary is on (it is unknown), the contractor's
ability to enumerate every RSA call site (they will miss some), or the
compiler's RSA intrinsics (Aegis scans source, not binary).

## 3. Out of scope
Aegis does not defend against: RSA keys held in HSMs the contractor does
not scan, post-quantum attacks on PQC itself (Aegis is a *migrator*, not
a cryptographer), and RSA uses embedded in vendored C code that Aegis
cannot reach (the contractor must run Aegis against vendored source too).

## 4. Residual risk
Aegis's scanner is keyword-and-API-pattern based. A contractor that uses
RSA via an indirect wrapper (e.g., a custom `SecureChannel::encrypt`
that internally calls `rsa::encrypt`) will not be flagged unless the
wrapper is itself enumerated. The buyer is accepting "best-effort
RSA-discovery, not cryptographic certainty."
