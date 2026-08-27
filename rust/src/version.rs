use crate::core::{PowerNumber, POWER_NUMBERS};
use crate::error::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerVersionComponent {
    Power,
    Master,
    Aura,
}

impl Default for PowerVersionComponent {
    fn default() -> Self {
        PowerVersionComponent::Aura
    }
}

impl PowerVersionComponent {
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "power" => Some(PowerVersionComponent::Power),
            "master" => Some(PowerVersionComponent::Master),
            "aura" => Some(PowerVersionComponent::Aura),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PowerVersion {
    pub power: PowerNumber,
    pub master: PowerNumber,
    pub aura: PowerNumber,
    pub prerelease: Option<String>,
    pub build: Option<String>,
}

// -- hand-written replacement for --
// ^(\d+)\.(\d+)\.(\d+)(-[0-9A-Za-z-]+(\.[0-9A-Za-z-]+)*)?(\+[0-9A-Za-z-]+(\.[0-9A-Za-z-]+)*)?$
//
// The grammar has no ambiguous overlaps (identifier characters never include '+', and '-' only
// ever acts as a delimiter in the one fixed position right after the third digit run), so a single
// greedy left-to-right scan with no backtracking is behaviorally equivalent to the regex.

fn take_digits(s: &str) -> Option<(&str, &str)> {
    let end = s.find(|c: char| !c.is_ascii_digit()).unwrap_or(s.len());
    if end == 0 {
        None
    } else {
        Some((&s[..end], &s[end..]))
    }
}

fn expect_char(s: &str, expected: char) -> Option<&str> {
    let mut chars = s.chars();
    if chars.next() == Some(expected) {
        Some(chars.as_str())
    } else {
        None
    }
}

fn is_identifier_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '-'
}

fn take_identifier_segment(s: &str) -> Option<(&str, &str)> {
    let end = s.find(|c: char| !is_identifier_char(c)).unwrap_or(s.len());
    if end == 0 {
        None
    } else {
        Some((&s[..end], &s[end..]))
    }
}

/// One-or-more `[0-9A-Za-z-]+` segments separated by a literal `.`.
fn take_dot_separated_identifiers(s: &str) -> Option<(&str, &str)> {
    let (first_segment, mut rest) = take_identifier_segment(s)?;
    let mut consumed = first_segment.len();
    loop {
        match rest.strip_prefix('.').and_then(take_identifier_segment) {
            Some((segment, remainder)) => {
                consumed += 1 + segment.len();
                rest = remainder;
            }
            None => break,
        }
    }
    Some((&s[..consumed], &s[consumed..]))
}

fn assert_power_number(value: u64, component: &'static str) -> Result<PowerNumber, Error> {
    if POWER_NUMBERS.iter().any(|&n| u64::from(n) == value) {
        Ok(value as PowerNumber)
    } else {
        Err(Error::NotAPowerNumber { component, value })
    }
}

fn parse_power_component(digits: &str, component: &'static str) -> Result<PowerNumber, Error> {
    let value: u64 = digits.parse().unwrap_or(u64::MAX);
    assert_power_number(value, component)
}

pub fn parse_power_version(version: &str) -> Result<PowerVersion, Error> {
    let invalid = || Error::InvalidVersionFormat(version.to_string());

    let (power_str, rest) = take_digits(version).ok_or_else(invalid)?;
    let rest = expect_char(rest, '.').ok_or_else(invalid)?;
    let (master_str, rest) = take_digits(rest).ok_or_else(invalid)?;
    let rest = expect_char(rest, '.').ok_or_else(invalid)?;
    let (aura_str, mut rest) = take_digits(rest).ok_or_else(invalid)?;

    let mut prerelease = None;
    if let Some(after_dash) = rest.strip_prefix('-') {
        let (segment, remainder) = take_dot_separated_identifiers(after_dash).ok_or_else(invalid)?;
        prerelease = Some(segment.to_string());
        rest = remainder;
    }

    let mut build = None;
    if let Some(after_plus) = rest.strip_prefix('+') {
        let (segment, remainder) = take_dot_separated_identifiers(after_plus).ok_or_else(invalid)?;
        build = Some(segment.to_string());
        rest = remainder;
    }

    if !rest.is_empty() {
        return Err(invalid());
    }

    Ok(PowerVersion {
        power: parse_power_component(power_str, "power")?,
        master: parse_power_component(master_str, "master")?,
        aura: parse_power_component(aura_str, "aura")?,
        prerelease,
        build,
    })
}

pub fn is_power_version(version: &str) -> bool {
    parse_power_version(version).is_ok()
}

fn successor(value: PowerNumber) -> Option<PowerNumber> {
    let index = POWER_NUMBERS.iter().position(|&n| n == value)?;
    POWER_NUMBERS.get(index + 1).copied()
}

pub fn next_power_version(
    version: &str,
    component: PowerVersionComponent,
) -> Result<String, Error> {
    let current = parse_power_version(version)?;
    let power = current.power;
    let mut master = current.master;
    let mut aura = current.aura;

    match component {
        PowerVersionComponent::Power => {
            let next = successor(power).ok_or(Error::NoSuccessor)?;
            return Ok(format!("{next}.1.1"));
        }
        PowerVersionComponent::Master => {
            aura = 1;
            if let Some(next) = successor(master) {
                return Ok(format!("{power}.{next}.1"));
            }
            master = 1;
        }
        PowerVersionComponent::Aura => {
            if let Some(next) = successor(aura) {
                return Ok(format!("{power}.{master}.{next}"));
            }
            aura = 1;
            if let Some(next_master) = successor(master) {
                return Ok(format!("{power}.{next_master}.{aura}"));
            }
            master = 1;
        }
    }

    let next_power = successor(power).ok_or(Error::NoSuccessor)?;
    Ok(format!("{next_power}.{master}.{aura}"))
}
