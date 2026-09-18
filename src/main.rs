//! main.rs — Aegis CLI entry point.
//! Standard CLI flags: -h/--help, -V/--version, --format, -o/--output, -q/--quiet, -v/--verbose.

use std::env;
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::process;
use aegis::{emit_json, emit_markdown, emit_ml_dsa_shim, emit_ml_kem_shim, load_source, policy, render_plan, scan_source, Policy, ShimDialect, Source};

const VERSION: &str = "0.2.0";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OutputFormat { Text, Json, Markdown }

#[derive(Debug)]
struct CliConfig {
    subcommand: String,
    target_path: Option<PathBuf>,
    format: OutputFormat,
    output_file: Option<PathBuf>,
    quiet: bool,
    verbose: bool,
}

impl Default for CliConfig {
    fn default() -> Self {
        CliConfig {
            subcommand: "scan".into(), target_path: None, format: OutputFormat::Text,
            output_file: None, quiet: false, verbose: false,
        }
    }
}

fn print_help() {
    println!(
        "aegis {} — PQC migration SDK & OMB M-26-15 scanner\n\
        USAGE:\n  aegis [SUBCOMMAND] [OPTIONS] [PATH]\n\
        SUBCOMMANDS:\n  scan, plan, policy check, shims\n\
        OPTIONS:\n  -h, --help; -V, --version; --format <text|json|markdown>; -o, --output <file>; -q, -v\n\
        EXAMPLES:\n  aegis scan ./src\n  aegis plan --format markdown -o MIGRATION.md ./src\n",
        VERSION
    );
}

fn parse_args(args: &[String]) -> Result<Option<CliConfig>, String> {
    let mut c = CliConfig::default();
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-h" | "--help" => { print_help(); return Ok(None); }
            "-V" | "--version" => { println!("aegis {}", VERSION); return Ok(None); }
            "-q" | "--quiet" => c.quiet = true,
            "-v" | "--verbose" => c.verbose = true,
            "--format" => {
                i += 1; if i >= args.len() { return Err("Missing format".into()); }
                c.format = match args[i].to_lowercase().as_str() {
                    "json" => OutputFormat::Json, "markdown" | "md" => OutputFormat::Markdown,
                    _ => OutputFormat::Text,
                };
            }
            "-o" | "--output" => {
                i += 1; if i >= args.len() { return Err("Missing -o path".into()); }
                c.output_file = Some(PathBuf::from(&args[i]));
            }
            "policy" => {
                if i + 1 < args.len() && args[i + 1] == "check" { i += 1; c.subcommand = "policy_check".into(); }
            }
            "scan" | "plan" | "shims" => c.subcommand = args[i].clone(),
            arg if !arg.starts_with('-') => c.target_path = Some(PathBuf::from(arg)),
            other => return Err(format!("Unknown option: {}", other)),
        }
        i += 1;
    }
    Ok(Some(c))
}

fn collect_sources(path: &Path) -> Result<Vec<Source>, String> {
    let mut sources = Vec::new();
    if path.is_file() {
        sources.push(load_source(path).map_err(|e| format!("Load failed: {}", e))?);
    } else if path.is_dir() {
        walk_dir(path, &mut sources)?;
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
                    if let Ok(s) = load_source(&p) { out.push(s); }
                }
            }
        }
    }
    Ok(())
}

fn write_output(content: &str, target: Option<&PathBuf>) -> Result<(), std::io::Error> {
    if let Some(path) = target { fs::write(path, content) } else { print!("{}", content); Ok(()) }
}

fn run() -> Result<i32, String> {
    let args: Vec<String> = env::args().collect();
    let config = match parse_args(&args)? { Some(c) => c, None => return Ok(0) };

    if config.subcommand == "shims" {
        let shims = format!("{}\n{}", emit_ml_kem_shim(ShimDialect::Rust), emit_ml_dsa_shim(ShimDialect::Rust));
        write_output(&shims, config.output_file.as_ref()).map_err(|e| format!("Write failed: {}", e))?;
        return Ok(0);
    }

    let sources = if let Some(ref p) = config.target_path {
        collect_sources(p)?
    } else {
        let mut buf = String::new();
        io::stdin().read_to_string(&mut buf).map_err(|e| format!("Stdin read failed: {}", e))?;
        vec![Source { path: "stdin".into(), content: buf }]
    };

    let mut rep = aegis::ScanReport {
        target: config.target_path.as_ref().map(|p| p.to_string_lossy().into_owned()).unwrap_or_else(|| "stdin".into()),
        total_lines: 0, rsa_count: 0, ecc_count: 0, evp_count: 0, weak_hash_count: 0, findings: Vec::new(),
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
    match config.subcommand.as_str() {
        "policy_check" => {
            let verdict = policy::evaluate(&rep, &Policy::default());
            if !config.quiet {
                if verdict.passed { println!("aegis policy check: PASSED (zero pre-2030 cryptographic regressions)"); }
                else { eprintln!("aegis policy check: FAILED"); for v in &verdict.violations { eprintln!("  - {}", v); } }
            }
            Ok(if verdict.passed { 0 } else { 1 })
        }
        "plan" => {
            let out = match config.format {
                OutputFormat::Json => emit_json(&plan, &rep),
                _ => emit_markdown(&plan, &rep),
            };
            write_output(&out, config.output_file.as_ref()).map_err(|e| format!("Write failed: {}", e))?;
            Ok(if rep.findings.is_empty() { 0 } else { 1 })
        }
        _ => {
            let out = match config.format {
                OutputFormat::Json => emit_json(&plan, &rep),
                OutputFormat::Markdown => emit_markdown(&plan, &rep),
                OutputFormat::Text => format!(
                    "aegis: scanned {} lines in {}\nRSA: {} | ECC: {} | EVP: {} | Weak Hash: {}\nOMB M-26-15 Status: {}\n",
                    rep.total_lines, rep.target, rep.rsa_count, rep.ecc_count, rep.evp_count, rep.weak_hash_count,
                    if rep.findings.is_empty() { "COMPLIANT" } else { "ACTION REQUIRED (Deadline: 2030-12-31)" }
                ),
            };
            write_output(&out, config.output_file.as_ref()).map_err(|e| format!("Write failed: {}", e))?;
            Ok(if rep.findings.is_empty() { 0 } else { 1 })
        }
    }
}

fn main() {
    match run() {
        Ok(code) => process::exit(code),
        Err(err) => { eprintln!("error: {}", err); process::exit(2); }
    }
}
