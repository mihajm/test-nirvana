use std::fmt;
use std::path::Path;
use std::process::{Command, Stdio};

use crate::error::Error;

pub const POWER_NUMBERS: [PowerNumber; 12] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 22, 33];

/// Checked at the type level only, same as the TypeScript source — `reduceToPowerNumber` ends in
/// an unchecked cast, not a runtime-validated newtype.
pub type PowerNumber = u32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Vibe {
    Immaculate,
    Good,
    Neutral,
    Chaotic,
    Cursed,
}

impl Vibe {
    pub fn as_str(&self) -> &'static str {
        match self {
            Vibe::Immaculate => "IMMACULATE",
            Vibe::Good => "GOOD",
            Vibe::Neutral => "NEUTRAL",
            Vibe::Chaotic => "CHAOTIC",
            Vibe::Cursed => "CURSED",
        }
    }
}

impl fmt::Display for Vibe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommitAura {
    pub destiny_number: PowerNumber,
    pub hash: String,
    pub total: u64,
    pub vibe: Vibe,
}

/// Mirrors `DESTINY_VIBES` in `src/core.ts`. Note `Vibe::Cursed` is never returned here — this is
/// a faithful port of the TypeScript source, where `CURSED` is dead code (see `error::Error::InvalidTotal`
/// and the `--hash 0` case: an invalid total is rejected by `reduce_to_power_number` before any
/// `Vibe` is ever produced, so `CURSED` can never actually surface from `inspect_commit_aura`).
fn destiny_vibe(power_number: PowerNumber) -> Vibe {
    match power_number {
        1 => Vibe::Neutral,
        2 => Vibe::Chaotic,
        3 => Vibe::Neutral,
        4 => Vibe::Chaotic,
        5 => Vibe::Neutral,
        6 => Vibe::Chaotic,
        7 => Vibe::Good,
        8 => Vibe::Good,
        9 => Vibe::Neutral,
        11 => Vibe::Immaculate,
        22 => Vibe::Immaculate,
        33 => Vibe::Immaculate,
        _ => unreachable!("destiny_vibe called with a value outside POWER_NUMBERS"),
    }
}

/// Digit characters use their numeric value; any other character (expected: 'a'-'f' lowercase,
/// since input is pre-validated hex) uses its alphabetic position — `'a' = 1` through `'f' = 6`,
/// not its real hexadecimal value.
fn character_value(character: char) -> u32 {
    match character.to_digit(10) {
        Some(digit) => digit,
        None => (character as u32) - 96,
    }
}

fn is_safe_integer(value: f64) -> bool {
    value.is_finite() && value.fract() == 0.0 && value.abs() <= 9_007_199_254_740_991.0
}

pub fn reduce_to_power_number(value: f64) -> Result<PowerNumber, Error> {
    if !is_safe_integer(value) || value < 1.0 {
        return Err(Error::InvalidTotal(value));
    }

    let mut reduced = value as u64;
    while reduced > 9 && reduced != 11 && reduced != 22 && reduced != 33 {
        reduced = reduced
            .to_string()
            .chars()
            .map(|digit| digit.to_digit(10).expect("decimal digit") as u64)
            .sum();
    }

    Ok(reduced as PowerNumber)
}

fn is_valid_hash(hash: &str) -> bool {
    !hash.is_empty() && hash.chars().all(|c| matches!(c, '0'..='9' | 'a'..='f'))
}

pub fn inspect_commit_aura(hash: &str) -> Result<CommitAura, Error> {
    let normalized_hash = hash.trim().to_lowercase();
    if !is_valid_hash(&normalized_hash) {
        return Err(Error::InvalidHash(normalized_hash));
    }

    let total: u64 = normalized_hash
        .chars()
        .map(|c| character_value(c) as u64)
        .sum();
    let destiny_number = reduce_to_power_number(total as f64)?;

    Ok(CommitAura {
        destiny_number,
        hash: normalized_hash,
        total,
        vibe: destiny_vibe(destiny_number),
    })
}

pub fn get_latest_commit_hash_in(cwd: &Path) -> Result<String, Error> {
    let output = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .current_dir(cwd)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output();

    match output {
        Ok(out) if out.status.success() => {
            Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
        }
        _ => Err(Error::NoAncestralCommit),
    }
}

pub fn get_latest_commit_hash() -> Result<String, Error> {
    let cwd = std::env::current_dir().map_err(|_| Error::NoAncestralCommit)?;
    get_latest_commit_hash_in(&cwd)
}

pub fn check_commit_vibes_in(cwd: &Path) -> Result<Vibe, Error> {
    Ok(inspect_commit_aura(&get_latest_commit_hash_in(cwd)?)?.vibe)
}

pub fn check_commit_vibes() -> Result<Vibe, Error> {
    Ok(inspect_commit_aura(&get_latest_commit_hash()?)?.vibe)
}
