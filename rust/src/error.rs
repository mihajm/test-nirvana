use std::fmt;

/// Mirrors the ad hoc `RangeError`/`TypeError`/`Error` throws scattered across the TypeScript
/// source. One flat enum, no per-module error types — matching CONTRIBUTING.md's own rule against
/// incidental abstraction.
#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    InvalidTotal(f64),
    InvalidHash(String),
    NoAncestralCommit,
    InvalidVersionFormat(String),
    NotAPowerNumber { component: &'static str, value: u64 },
    NoSuccessor,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidTotal(_) => write!(f, "Cosmic totals must be positive safe integers."),
            Error::InvalidHash(_) => {
                write!(f, "A commit aura can only be read from a hexadecimal hash.")
            }
            Error::NoAncestralCommit => write!(
                f,
                "No ancestral commit could be perceived from the current working tree."
            ),
            Error::InvalidVersionFormat(version) => write!(f, "Invalid Power Version: {version}."),
            Error::NotAPowerNumber { component, value } => write!(
                f,
                "{} must belong to the Power Sequence; received {value}.",
                component.to_uppercase()
            ),
            Error::NoSuccessor => {
                write!(f, "33.33.33 has no successor. Archive the project.")
            }
        }
    }
}

impl std::error::Error for Error {}
