use std::fmt;
use std::process::ExitCode;

use test_nirvana::{
    get_latest_commit_hash, inspect_commit_aura, next_power_version, parse_power_version, sigils,
    CommitAura, PowerVersionComponent, Vibe,
};

const HELP_TEXT: &str = "test-nirvana — deterministic commit-aura enforcement

Usage:
  test-nirvana [--hash <hex>] [--json]
  test-nirvana version validate <version>
  test-nirvana version next <version> [aura|master|power]

Options:
      --hash       Inspect a specific hexadecimal hash instead of HEAD
      --json       Emit the complete aura as JSON
  -h, --help       Show this canonical guidance
  -v, --version    Show the Power Version";

#[derive(Default)]
struct Args {
    hash: Option<String>,
    help: bool,
    json: bool,
    version: bool,
    positionals: Vec<String>,
}

/// CLI-only errors, distinct from the library's `test_nirvana::Error` — mirrors how `src/cli.ts`
/// throws its own ad hoc `TypeError`s separate from the typed errors thrown by `core.ts`/`version.ts`.
enum RunError {
    Cli(String),
    Lib(test_nirvana::Error),
}

impl RunError {
    fn cli(message: impl Into<String>) -> Self {
        RunError::Cli(message.into())
    }
}

impl fmt::Display for RunError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RunError::Cli(message) => write!(f, "{message}"),
            RunError::Lib(err) => write!(f, "{err}"),
        }
    }
}

impl From<test_nirvana::Error> for RunError {
    fn from(err: test_nirvana::Error) -> Self {
        RunError::Lib(err)
    }
}

fn parse_args(argv: &[String]) -> Result<Args, RunError> {
    let mut args = Args::default();
    let mut only_positionals = false;
    let mut i = 0;

    while i < argv.len() {
        let a = argv[i].as_str();

        if only_positionals {
            args.positionals.push(a.to_string());
            i += 1;
            continue;
        }

        match a {
            "--" => only_positionals = true,
            "--help" | "-h" => args.help = true,
            "--json" => args.json = true,
            "--version" | "-v" => args.version = true,
            "--hash" => {
                i += 1;
                let value = argv
                    .get(i)
                    .ok_or_else(|| RunError::cli("Option '--hash' expects a value."))?;
                args.hash = Some(value.clone());
            }
            s if s.starts_with("--hash=") => {
                args.hash = Some(s["--hash=".len()..].to_string());
            }
            s if s.starts_with('-') && s.len() > 1 => {
                return Err(RunError::cli(format!("Unknown option: {s}")));
            }
            other => args.positionals.push(other.to_string()),
        }

        i += 1;
    }

    Ok(args)
}

fn exit_code_for_vibe(vibe: Vibe) -> u8 {
    match vibe {
        Vibe::Immaculate | Vibe::Good | Vibe::Neutral => 0,
        Vibe::Chaotic => 1,
        Vibe::Cursed => 2,
    }
}

fn vibe_sigil(vibe: Vibe) -> sigils::Sigil {
    match vibe {
        Vibe::Immaculate => sigils::IMMACULATE,
        Vibe::Good => sigils::GOOD,
        Vibe::Neutral => sigils::NEUTRAL,
        Vibe::Chaotic => sigils::CHAOTIC,
        Vibe::Cursed => sigils::CURSED,
    }
}

/// Byte-for-byte equivalent of `JSON.stringify(aura, undefined, 2)` for a `CommitAura` — the hash
/// field never needs escaping since `inspect_commit_aura` only ever produces `[0-9a-f]+` hashes.
fn format_aura_json(aura: &CommitAura) -> String {
    format!(
        "{{\n  \"destinyNumber\": {},\n  \"hash\": \"{}\",\n  \"total\": {},\n  \"vibe\": \"{}\"\n}}",
        aura.destiny_number,
        aura.hash,
        aura.total,
        aura.vibe.as_str()
    )
}

fn run() -> Result<ExitCode, RunError> {
    let argv: Vec<String> = std::env::args().skip(1).collect();
    let args = parse_args(&argv)?;

    if args.help {
        println!("{HELP_TEXT}");
        return Ok(ExitCode::SUCCESS);
    }

    if args.version && args.positionals.is_empty() {
        println!("{}", env!("CARGO_PKG_VERSION"));
        return Ok(ExitCode::SUCCESS);
    }

    if args.positionals.first().map(String::as_str) == Some("version") {
        let operation = args.positionals.get(1).map(String::as_str);
        let version = args.positionals.get(2);
        let component_str = args.positionals.get(3).map(String::as_str).unwrap_or("aura");
        let invalid_operation = || {
            RunError::cli("Invalid version operation. Consult --help before proceeding.")
        };

        return match (operation, version) {
            (Some("validate"), Some(version)) => {
                parse_power_version(version)?;
                println!("{} {version} belongs to the Power Sequence.", sigils::IMMACULATE);
                Ok(ExitCode::SUCCESS)
            }
            (Some("next"), Some(version)) => match PowerVersionComponent::parse(component_str) {
                Some(component) => {
                    println!("{}", next_power_version(version, component)?);
                    Ok(ExitCode::SUCCESS)
                }
                None => Err(invalid_operation()),
            },
            _ => Err(invalid_operation()),
        };
    }

    if !args.positionals.is_empty() {
        return Err(RunError::cli(format!(
            "Unknown rite: {}.",
            args.positionals.join(" ")
        )));
    }

    let hash = match &args.hash {
        Some(h) => h.clone(),
        None => get_latest_commit_hash()?,
    };
    let aura = inspect_commit_aura(&hash)?;

    if args.json {
        println!("{}", format_aura_json(&aura));
    } else {
        println!(
            "{} {} — {} resolves through {} to {}",
            vibe_sigil(aura.vibe),
            aura.vibe,
            aura.hash,
            aura.total,
            aura.destiny_number
        );
    }

    Ok(ExitCode::from(exit_code_for_vibe(aura.vibe)))
}

fn main() -> ExitCode {
    match run() {
        Ok(code) => code,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::FAILURE
        }
    }
}
