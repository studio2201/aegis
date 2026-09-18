//! policy.rs — Policy assertion against post-2030 compliance rules.

use crate::scan::ScanReport;

#[derive(Debug, Clone)]
pub struct Policy {
    pub allow_rsa: bool,
    pub allow_ecc: bool,
    pub max_legacy_sites: usize,
}

impl Default for Policy {
    fn default() -> Self {
        Policy {
            allow_rsa: false,
            allow_ecc: false,
            max_legacy_sites: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PolicyVerdict {
    pub passed: bool,
    pub violations: Vec<String>,
}

pub fn evaluate(report: &ScanReport, policy: &Policy) -> PolicyVerdict {
    let mut violations = Vec::new();

    if !policy.allow_rsa && report.rsa_count > 0 {
        violations.push(format!(
            "Policy strictly prohibits RSA past 2030 (found {} call sites)",
            report.rsa_count
        ));
    }

    if !policy.allow_ecc && report.ecc_count > 0 {
        violations.push(format!(
            "Policy prohibits classical ECC algorithms without PQC encapsulation (found {} call sites)",
            report.ecc_count
        ));
    }

    let total = report.findings.len();
    if total > policy.max_legacy_sites {
        violations.push(format!(
            "Total legacy cryptographic sites {} exceeds policy cap {}",
            total, policy.max_legacy_sites
        ));
    }

    let passed = violations.is_empty();
    PolicyVerdict { passed, violations }
}
