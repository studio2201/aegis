//! scan.rs — Static scanner identifying classical cryptographic call sites.
//! Scans for RSA, classic ECC, EVP_PKEY, and weak hashes in pure std::.

use std::fmt;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CryptoPrimitive {
    Rsa,
    Ecc,
    EvpPkey,
    WeakHash,
}

impl fmt::Display for CryptoPrimitive {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CryptoPrimitive::Rsa => write!(f, "RSA"),
            CryptoPrimitive::Ecc => write!(f, "Classical ECC"),
            CryptoPrimitive::EvpPkey => write!(f, "OpenSSL EVP_PKEY"),
            CryptoPrimitive::WeakHash => write!(f, "Weak Hash (MD5/SHA1)"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CryptoFinding {
    pub file: String,
    pub line: usize,
    pub primitive: CryptoPrimitive,
    pub pattern: String,
    pub pqc_replacement: String,
    pub snippet: String,
}

#[derive(Debug, Clone)]
pub struct Source {
    pub path: String,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct ScanReport {
    pub target: String,
    pub total_lines: usize,
    pub rsa_count: usize,
    pub ecc_count: usize,
    pub evp_count: usize,
    pub weak_hash_count: usize,
    pub findings: Vec<CryptoFinding>,
}

#[derive(Debug)]
pub enum ScanError {
    Io(std::io::Error),
}

impl fmt::Display for ScanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScanError::Io(e) => write!(f, "I/O error: {}", e),
        }
    }
}

impl std::error::Error for ScanError {}

impl From<std::io::Error> for ScanError {
    fn from(e: std::io::Error) -> Self {
        ScanError::Io(e)
    }
}

pub fn load_source(path: &Path) -> Result<Source, ScanError> {
    let content = fs::read_to_string(path)?;
    Ok(Source {
        path: path.to_string_lossy().into_owned(),
        content,
    })
}

pub fn scan_source(source: &Source) -> ScanReport {
    let mut findings = Vec::new();
    let mut rsa_count = 0;
    let mut ecc_count = 0;
    let mut evp_count = 0;
    let mut weak_hash_count = 0;
    let mut total_lines = 0;

    for (idx, line) in source.content.lines().enumerate() {
        total_lines += 1;
        let line_num = idx + 1;
        let trimmed = line.trim();
        if trimmed.starts_with("//")
            || trimmed.starts_with('#')
            || trimmed.starts_with("/*")
            || trimmed.starts_with('*')
        {
            continue;
        }

        // 1. RSA detection
        if trimmed.contains("RSA_generate_key") || trimmed.contains("RSA_public_encrypt")
            || trimmed.contains("RSASSA-PKCS1") || trimmed.contains("crypto/rsa")
            || trimmed.contains("import rsa") || trimmed.contains("RSAKey")
            || trimmed.contains("RS256") || trimmed.contains("from cryptography.hazmat.primitives.asymmetric import rsa")
        {
            rsa_count += 1;
            findings.push(CryptoFinding {
                file: source.path.clone(),
                line: line_num,
                primitive: CryptoPrimitive::Rsa,
                pattern: "RSA asymmetric operation".into(),
                pqc_replacement: "ML-KEM-768 (KEM) / ML-DSA-65 (Signatures)".into(),
                snippet: trimmed.to_string(),
            });
            continue;
        }

        // 2. Classical ECC detection
        if trimmed.contains("ECDSA") || trimmed.contains("secp256k1") || trimmed.contains("prime256v1")
            || trimmed.contains("EC_KEY") || trimmed.contains("ES256") || trimmed.contains("crypto/ecdsa")
            || trimmed.contains("ECDH")
        {
            ecc_count += 1;
            findings.push(CryptoFinding {
                file: source.path.clone(),
                line: line_num,
                primitive: CryptoPrimitive::Ecc,
                pattern: "Classical Elliptic Curve operation".into(),
                pqc_replacement: "ML-DSA-65 (FIPS 204) / ML-KEM-768 (FIPS 203)".into(),
                snippet: trimmed.to_string(),
            });
            continue;
        }

        // 3. OpenSSL EVP_PKEY
        if trimmed.contains("EVP_PKEY_RSA") || trimmed.contains("EVP_PKEY_EC") || trimmed.contains("EVP_PKEY_new") {
            evp_count += 1;
            findings.push(CryptoFinding {
                file: source.path.clone(),
                line: line_num,
                primitive: CryptoPrimitive::EvpPkey,
                pattern: "Legacy OpenSSL EVP_PKEY primitive".into(),
                pqc_replacement: "EVP_PKEY PQC Provider / OQS-OpenSSL Shim".into(),
                snippet: trimmed.to_string(),
            });
            continue;
        }

        // 4. Weak Hash
        if (trimmed.contains("MD5") || trimmed.contains("SHA1") || trimmed.contains("SHA-1"))
            && (trimmed.contains("sign") || trimmed.contains("verify") || trimmed.contains("digest"))
        {
            weak_hash_count += 1;
            findings.push(CryptoFinding {
                file: source.path.clone(),
                line: line_num,
                primitive: CryptoPrimitive::WeakHash,
                pattern: "Quantum-vulnerable / collided hash in signature".into(),
                pqc_replacement: "SHA-384 / SHA-512 / SHAKE256".into(),
                snippet: trimmed.to_string(),
            });
        }
    }

    ScanReport {
        target: source.path.clone(),
        total_lines,
        rsa_count,
        ecc_count,
        evp_count,
        weak_hash_count,
        findings,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comment_skipping() {
        let code = r#"
            // ECDSA in line comment
            # ECDSA in python comment
            /* ECDSA in block comment start */
            /*
             * ECDSA in block comment interior
             * RSA_generate_key in block comment interior
             */
            let valid = 1;
        "#;
        let src = Source {
            path: "test.c".into(),
            content: code.into(),
        };
        let report = scan_source(&src);
        assert_eq!(report.rsa_count, 0);
        assert_eq!(report.ecc_count, 0);
        assert_eq!(report.findings.len(), 0);
    }

    #[test]
    fn test_detection_active_crypto() {
        let code = "let key = ECDSA::generate();\nlet rsa = RSA_generate_key(2048);";
        let src = Source {
            path: "test.rs".into(),
            content: code.into(),
        };
        let report = scan_source(&src);
        assert_eq!(report.rsa_count, 1);
        assert_eq!(report.ecc_count, 1);
        assert_eq!(report.findings.len(), 2);
    }
}
