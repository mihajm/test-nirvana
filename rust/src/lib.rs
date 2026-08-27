//! Mirrors `src/index.ts` — module declarations and re-exports only.

pub mod core;
pub mod error;
pub mod sigils;
pub mod version;

pub use core::{
    check_commit_vibes, check_commit_vibes_in, get_latest_commit_hash, get_latest_commit_hash_in,
    inspect_commit_aura, reduce_to_power_number, CommitAura, PowerNumber, Vibe, POWER_NUMBERS,
};
pub use error::Error;
pub use version::{
    is_power_version, next_power_version, parse_power_version, PowerVersion, PowerVersionComponent,
};
