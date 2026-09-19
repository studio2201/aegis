//! cli.rs — Standard CLI arguments and subcommand parser for aegis.
use std::fmt;
use std::path::PathBuf;

pub const VERSION: &str = "0.2.8";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Text,
    Json,
    Markdown,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Subcommand {
    Scan,
    Plan,
    Shim,
    PolicyCheck,
    Doctor,
    Update,
    Help,
    Version,
}

#[derive(Debug)]
pub struct CliConfig {
    pub subcommand: Subcommand,
    pub target_path: Option<PathBuf>,
    pub format: OutputFormat,
    pub output_file: Option<PathBuf>,
    pub quiet: bool,
    pub verbose: bool,
}

impl Default for CliConfig {
    fn default() -> Self {
        CliConfig {
            subcommand: Subcommand::Scan,
            target_path: None,
            format: OutputFormat::Text,
            output_file: None,
            quiet: false,
            verbose: false,
        }
    }
}

#[derive(Debug)]
pub enum CliError {
    Parse(String),
    Runtime(String),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CliError::Parse(s) | CliError::Runtime(s) => write!(f, "{}", s),
        }
    }
}

impl From<String> for CliError {
    fn from(s: String) -> Self {
        CliError::Runtime(s)
    }
}

impl From<&str> for CliError {
    fn from(s: &str) -> Self {
        CliError::Runtime(s.to_string())
    }
}

impl From<std::io::Error> for CliError {
    fn from(e: std::io::Error) -> Self {
        CliError::Runtime(e.to_string())
    }
}

pub fn print_help() {
    println!(
        "aegis {} — PQC migration SDK & OMB M-26-15 scanner\n\
        \n\
        USAGE:\n\
          aegis [SUBCOMMAND] [OPTIONS] [PATH]\n\
        \n\
        SUBCOMMANDS:\n\
          scan             Scan codebase for pre-2030 cryptography (default)\n\
          plan             Generate migration plan and remediation checklist\n\
          shim, shims      Emit ML-KEM and ML-DSA compatibility shims\n\
          policy check     Assert compliance against cryptographic policy\n\
          doctor           Inspect system health and environment\n\
          update, upgrade  Update binary to latest release\n\
          help             Print help information\n\
          version          Print version information\n\
        \n\
        OPTIONS:\n\
          -h, --help              Print help information\n\
          -V, --version           Print version information\n\
          -f, --format <fmt>      Output format: text, json, markdown [default: text]\n\
          -o, --output <file>     Write report to file instead of stdout\n\
          -q, --quiet             Quiet mode; exit code only\n\
          -v, --verbose           Verbose diagnostic logging to stderr\n\
        \n\
        EXAMPLES:\n\
          aegis scan ./src\n\
          aegis plan -f markdown -o MIGRATION.md ./src\n\
          aegis shim -o shims.rs\n\
          aegis doctor\n",
        VERSION
    );
}

pub fn parse_args(args: &[String]) -> Result<CliConfig, CliError> {
    let mut config = CliConfig::default();
    let mut i = 1;

    while i < args.len() {
        match args[i].as_str() {
            "-h" | "--help" | "help" => {
                config.subcommand = Subcommand::Help;
                return Ok(config);
            }
            "-V" | "--version" | "version" => {
                config.subcommand = Subcommand::Version;
                return Ok(config);
            }
            "-q" | "--quiet" => config.quiet = true,
            "-v" | "--verbose" => config.verbose = true,
            "-f" | "--format" => {
                i += 1;
                if i >= args.len() {
                    return Err(CliError::Parse("Missing argument for format option".into()));
                }
                config.format = match args[i].to_lowercase().as_str() {
                    "json" => OutputFormat::Json,
                    "markdown" | "md" => OutputFormat::Markdown,
                    "text" => OutputFormat::Text,
                    other => return Err(CliError::Parse(format!("Unknown format: {}", other))),
                };
            }
            "-o" | "--output" => {
                i += 1;
                if i >= args.len() {
                    return Err(CliError::Parse("Missing argument for output option".into()));
                }
                config.output_file = Some(PathBuf::from(&args[i]));
            }
            "policy" => {
                if i + 1 < args.len() && args[i + 1] == "check" {
                    i += 1;
                    config.subcommand = Subcommand::PolicyCheck;
                } else {
                    return Err(CliError::Parse("Unknown policy subcommand: expected 'check'".into()));
                }
            }
            "scan" => config.subcommand = Subcommand::Scan,
            "plan" => config.subcommand = Subcommand::Plan,
            "shim" | "shims" => config.subcommand = Subcommand::Shim,
            "doctor" => config.subcommand = Subcommand::Doctor,
            "update" | "upgrade" => config.subcommand = Subcommand::Update,
            arg if !arg.starts_with('-') => config.target_path = Some(PathBuf::from(arg)),
            other => return Err(CliError::Parse(format!("Unknown option: {}", other))),
        }
        i += 1;
    }
    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_flags() {
        let args = vec!["aegis".into(), "-q".into(), "-f".into(), "json".into()];
        let cfg = parse_args(&args).unwrap();
        assert!(cfg.quiet);
        assert_eq!(cfg.format, OutputFormat::Json);
    }

    #[test]
    fn test_subcommands() {
        let args = vec!["aegis".into(), "shim".into()];
        assert_eq!(parse_args(&args).unwrap().subcommand, Subcommand::Shim);
        let args = vec!["aegis".into(), "shims".into()];
        assert_eq!(parse_args(&args).unwrap().subcommand, Subcommand::Shim);
        let args = vec!["aegis".into(), "doctor".into()];
        assert_eq!(parse_args(&args).unwrap().subcommand, Subcommand::Doctor);
    }

    #[test]
    fn test_parse_errors() {
        let args = vec!["aegis".into(), "--invalid".into()];
        assert!(matches!(parse_args(&args), Err(CliError::Parse(_))));
        let args = vec!["aegis".into(), "-f".into()];
        assert!(matches!(parse_args(&args), Err(CliError::Parse(_))));
    }
}
