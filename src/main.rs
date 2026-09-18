//! main.rs — Aegis CLI entry point.
//! Standard CLI flags: -h/--help, -V/--version, -f/--format, -o/--output, -q/--quiet, -v/--verbose.

mod cli;
mod doctor;
mod update;
mod xdg;

use aegis::{
    emit_json, emit_markdown, emit_ml_dsa_shim, emit_ml_kem_shim, load_source, policy,
    render_plan, scan_source, Policy, ShimDialect, Source,
};
use cli::{parse_args, print_help, CliConfig, CliError, OutputFormat, Subcommand, VERSION};
use std::env;
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::process;

fn write_output(content: &str, target: Option<&PathBuf>) -> Result<(), std::io::Error> {
    if let Some(path) = target {
        fs::write(path, content)
    } else {
        print!("{}", content);
        Ok(())
    }
}

fn collect_sources(path: &Path) -> Result<Vec<Source>, String> {
    let mut sources = Vec::new();
    if path.is_file() {
        sources.push(load_source(path).map_err(|e| format!("Load failed: {}", e))?);
    } else if path.is_dir() {
        walk_dir(path, &mut sources)?;
    } else {
        return Err(format!("Target path does not exist: {}", path.display()));
    }
    Ok(sources)
}

fn walk_dir(dir: &Path, out: &mut Vec<Source>) -> Result<(), String> {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_dir() {
                let name = p.file_name().and_then(|n| n.to_str()).unwrap_or("");
                if !name.starts_with('.') && name != "target" && name != "node_modules" {
                    let _ = walk_dir(&p, out);
                }
            } else if let Some(ext) = p.extension().and_then(|e| e.to_str()) {
                if ["rs", "c", "h", "cpp", "go", "ts", "js", "py"].contains(&ext) {
                    if let Ok(s) = load_source(&p) {
                        out.push(s);
                    }
                }
            }
        }
    }
    Ok(())
}

fn run_scan_app(config: &CliConfig) -> Result<i32, CliError> {
    let sources = if let Some(ref p) = config.target_path {
        collect_sources(p)?
    } else {
        let mut buf = String::new();
        io::stdin()
            .read_to_string(&mut buf)
            .map_err(|e| format!("Stdin read failed: {}", e))?;
        vec![Source {
            path: "stdin".into(),
            content: buf,
        }]
    };

    let mut rep = aegis::ScanReport {
        target: config
            .target_path
            .as_ref()
            .map(|p| p.to_string_lossy().into_owned())
            .unwrap_or_else(|| "stdin".into()),
        total_lines: 0,
        rsa_count: 0,
        ecc_count: 0,
        evp_count: 0,
        weak_hash_count: 0,
        findings: Vec::new(),
    };

    for s in &sources {
        let r = scan_source(s);
        rep.total_lines += r.total_lines;
        rep.rsa_count += r.rsa_count;
        rep.ecc_count += r.ecc_count;
        rep.evp_count += r.evp_count;
        rep.weak_hash_count += r.weak_hash_count;
        rep.findings.extend(r.findings);
    }

    let plan = render_plan(&rep);
    match config.subcommand {
        Subcommand::PolicyCheck => {
            let verdict = policy::evaluate(&rep, &Policy::default());
            if !config.quiet {
                if verdict.passed {
                    println!("aegis policy check: PASSED (zero pre-2030 cryptographic regressions)");
                } else {
                    eprintln!("aegis policy check: FAILED");
                    for v in &verdict.violations {
                        eprintln!("  - {}", v);
                    }
                }
            }
            Ok(if verdict.passed { 0 } else { 1 })
        }
        Subcommand::Plan => {
            let out = match config.format {
                OutputFormat::Json => emit_json(&plan, &rep),
                _ => emit_markdown(&plan, &rep),
            };
            write_output(&out, config.output_file.as_ref())?;
            Ok(if rep.findings.is_empty() { 0 } else { 1 })
        }
        _ => {
            let out = match config.format {
                OutputFormat::Json => emit_json(&plan, &rep),
                OutputFormat::Markdown => emit_markdown(&plan, &rep),
                OutputFormat::Text => format!(
                    "aegis: scanned {} lines in {}\nRSA: {} | ECC: {} | EVP: {} | Weak Hash: {}\nOMB M-26-15 Status: {}\n",
                    rep.total_lines,
                    rep.target,
                    rep.rsa_count,
                    rep.ecc_count,
                    rep.evp_count,
                    rep.weak_hash_count,
                    if rep.findings.is_empty() {
                        "COMPLIANT"
                    } else {
                        "ACTION REQUIRED (Deadline: 2030-12-31)"
                    }
                ),
            };
            write_output(&out, config.output_file.as_ref())?;
            Ok(if rep.findings.is_empty() { 0 } else { 1 })
        }
    }
}

fn run() -> Result<i32, CliError> {
    let args: Vec<String> = env::args().collect();
    let config = parse_args(&args)?;

    match config.subcommand {
        Subcommand::Help => {
            print_help();
            Ok(0)
        }
        Subcommand::Version => {
            println!("aegis {}", VERSION);
            Ok(0)
        }
        Subcommand::Doctor => Ok(doctor::run_doctor("aegis", VERSION, config.format)),
        Subcommand::Update => update::run_update("aegis", VERSION).map_err(CliError::Runtime),
        Subcommand::Shim => {
            let shims = format!(
                "{}\n{}",
                emit_ml_kem_shim(ShimDialect::Rust),
                emit_ml_dsa_shim(ShimDialect::Rust)
            );
            write_output(&shims, config.output_file.as_ref())?;
            Ok(0)
        }
        _ => run_scan_app(&config),
    }
}

fn main() {
    match run() {
        Ok(code) => process::exit(code),
        Err(CliError::Parse(err)) => {
            eprintln!("error: {}", err);
            process::exit(2);
        }
        Err(CliError::Runtime(err)) => {
            eprintln!("error: {}", err);
            process::exit(1);
        }
    }
}
