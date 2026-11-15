//! Pokémon Emerald's verified Gen III memory profile.
//!
//! The shared scanner implementation lives in `profile.rs`; this module keeps
//! Emerald-specific identity separate so Ruby/Sapphire can be added without
//! spreading game-specific offsets through the scanner.

pub use super::profile::EMERALD_PROFILE as PROFILE;
