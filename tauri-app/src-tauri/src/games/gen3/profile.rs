use crate::decoders::gen3_decoder::{PokemonData, decode_pokemon};
use crate::games::gen3::validator::validate_pokemon_data;
use crate::memory::memory_access::ProcessMemory;
use crate::memory::memory_scanner::MemoryRegions;

/// Gen III games share the Pokémon structure, but not necessarily the
/// locations of party and battle data inside the emulator's EWRAM.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gen3Game {
    Emerald,
    Ruby,
    Sapphire,
    FireRed,
    LeafGreen,
}

#[derive(Debug, Clone, Copy)]
pub enum MemoryRegion {
    Ewram,
    Iwram,
}

impl Gen3Game {
    pub const fn display_name(self) -> &'static str {
        match self {
            Self::Emerald => "Pokémon Emerald",
            Self::Ruby => "Pokémon Ruby",
            Self::Sapphire => "Pokémon Sapphire",
            Self::FireRed => "Pokémon FireRed",
            Self::LeafGreen => "Pokémon LeafGreen",
        }
    }

    pub fn from_id(id: &str) -> Result<Self, String> {
        match id.to_ascii_lowercase().as_str() {
            "emerald" => Ok(Self::Emerald),
            "ruby" => Ok(Self::Ruby),
            "sapphire" => Ok(Self::Sapphire),
            "firered" => Ok(Self::FireRed),
            "leafgreen" => Ok(Self::LeafGreen),
            other => Err(format!("Unknown Gen III game profile: {}", other)),
        }
    }
}

/// Verified memory layout for one specific game/build profile.
#[derive(Debug, Clone, Copy)]
pub struct Gen3MemoryProfile {
    pub game: Gen3Game,
    pub party_region: MemoryRegion,
    pub party_count_offset: usize,
    pub party_data_offset: usize,
    pub wild_region: MemoryRegion,
    pub wild_party_offset: usize,
    pub battle_region: MemoryRegion,
    pub battle_type_flags_offset: usize,
    pub trainer_battle_mask: u32,
}

/// Verified profiles for the US GBA releases supported by the scanner.
pub const EMERALD_PROFILE: Gen3MemoryProfile = Gen3MemoryProfile {
    game: Gen3Game::Emerald,
    party_region: MemoryRegion::Ewram,
    party_count_offset: 0x244E9,
    party_data_offset: 0x244EC,
    wild_region: MemoryRegion::Ewram,
    wild_party_offset: 0x24744,
    battle_region: MemoryRegion::Ewram,
    battle_type_flags_offset: 0x22FEC,
    trainer_battle_mask: 0x08,
};

// Ruby/Sapphire (US) use IWRAM for party buffers and EWRAM for battle flags.
// The absolute addresses are documented by the Gen III data layout and the
// pokeruby decompilation symbols; the offsets below are relative to the VBA
// region pointers resolved by memory_scanner.rs.
pub const RUBY_PROFILE: Gen3MemoryProfile = Gen3MemoryProfile {
    game: Gen3Game::Ruby,
    party_region: MemoryRegion::Iwram,
    party_count_offset: 0x4350,
    party_data_offset: 0x4360,
    wild_region: MemoryRegion::Iwram,
    wild_party_offset: 0x45C0,
    battle_region: MemoryRegion::Ewram,
    battle_type_flags_offset: 0x239F8,
    trainer_battle_mask: 0x08,
};

pub const SAPPHIRE_PROFILE: Gen3MemoryProfile = Gen3MemoryProfile {
    game: Gen3Game::Sapphire,
    party_region: MemoryRegion::Iwram,
    party_count_offset: 0x4350,
    party_data_offset: 0x4360,
    wild_region: MemoryRegion::Iwram,
    wild_party_offset: 0x45C0,
    battle_region: MemoryRegion::Ewram,
    battle_type_flags_offset: 0x239F8,
    trainer_battle_mask: 0x08,
};

// FireRed (US) stores its party and battle buffers in EWRAM. These are the
// absolute US addresses expressed as offsets from VBA's resolved EWRAM base.
pub const FIRERED_PROFILE: Gen3MemoryProfile = Gen3MemoryProfile {
    game: Gen3Game::FireRed,
    party_region: MemoryRegion::Ewram,
    party_count_offset: 0x24029,
    party_data_offset: 0x24284,
    wild_region: MemoryRegion::Ewram,
    wild_party_offset: 0x2402C,
    battle_region: MemoryRegion::Ewram,
    battle_type_flags_offset: 0x22B4C,
    trainer_battle_mask: 0x08,
};

// LeafGreen uses the same US Kanto party, wild-party, and battle buffers for
// the shared scanner layout.
pub const LEAFGREEN_PROFILE: Gen3MemoryProfile = Gen3MemoryProfile {
    game: Gen3Game::LeafGreen,
    party_region: MemoryRegion::Ewram,
    party_count_offset: 0x24029,
    party_data_offset: 0x24284,
    wild_region: MemoryRegion::Ewram,
    wild_party_offset: 0x2402C,
    battle_region: MemoryRegion::Ewram,
    battle_type_flags_offset: 0x22B4C,
    trainer_battle_mask: 0x08,
};

pub const POKEMON_SIZE: usize = 100;
pub const MAX_PARTY_SIZE: usize = 6;

pub fn verified_profile(game: Gen3Game) -> Result<&'static Gen3MemoryProfile, String> {
    match game {
        Gen3Game::Emerald => Ok(&EMERALD_PROFILE),
        Gen3Game::Ruby => Ok(&RUBY_PROFILE),
        Gen3Game::Sapphire => Ok(&SAPPHIRE_PROFILE),
        Gen3Game::FireRed => Ok(&FIRERED_PROFILE),
        Gen3Game::LeafGreen => Ok(&LEAFGREEN_PROFILE),
    }
}

fn region_base(regions: &MemoryRegions, region: MemoryRegion) -> usize {
    match region {
        MemoryRegion::Ewram => regions.ewram_address,
        MemoryRegion::Iwram => regions.iwram_address,
    }
}

/// Read and decode the first opponent-party slot for a game profile.
pub fn read_wild_candidate(
    memory: &ProcessMemory,
    regions: &MemoryRegions,
    profile: &Gen3MemoryProfile,
) -> Result<Option<PokemonData>, String> {
    let address = region_base(regions, profile.wild_region) + profile.wild_party_offset;
    let raw_data = memory.read(address, POKEMON_SIZE)?;

    if !validate_pokemon_data(&raw_data) {
        return Ok(None);
    }

    Ok(decode_pokemon(&raw_data).ok())
}

pub fn read_battle_type_flags(
    memory: &ProcessMemory,
    regions: &MemoryRegions,
    profile: &Gen3MemoryProfile,
) -> Result<u32, String> {
    let address = region_base(regions, profile.battle_region) + profile.battle_type_flags_offset;
    let data = memory.read(address, 4)?;

    if data.len() != 4 {
        return Err("Failed to read battle type flags".to_string());
    }

    Ok(u32::from_le_bytes([data[0], data[1], data[2], data[3]]))
}

pub fn is_trainer_battle(flags: u32, profile: &Gen3MemoryProfile) -> bool {
    flags & profile.trainer_battle_mask != 0
}

#[derive(Debug, Clone)]
pub struct PartySnapshot {
    pub count: u8,
    pub pokemon_data: Vec<u8>,
}

pub fn read_party(
    memory: &ProcessMemory,
    regions: &MemoryRegions,
    profile: &Gen3MemoryProfile,
) -> Result<PartySnapshot, String> {
    let (count_address, data_address) = party_addresses(regions, profile)?;

    let count = memory.read(count_address, 1)?[0];

    if count as usize > MAX_PARTY_SIZE {
        return Err(format!(
            "Invalid party count: {}. Expected a value from 0 to {}.",
            count, MAX_PARTY_SIZE
        ));
    }

    if count == 0 {
        return Ok(PartySnapshot {
            count,
            pokemon_data: Vec::new(),
        });
    }

    let data_size = (count as usize)
        .checked_mul(POKEMON_SIZE)
        .ok_or_else(|| "Party data size overflowed.".to_string())?;

    Ok(PartySnapshot {
        count,
        pokemon_data: memory.read(data_address, data_size)?,
    })
}

/// Resolve the live addresses used by the player's party count and data.
///
/// This is kept public so the scanner can include the exact addresses in its
/// diagnostics when validating a new game profile against an emulator.
pub fn party_addresses(
    regions: &MemoryRegions,
    profile: &Gen3MemoryProfile,
) -> Result<(usize, usize), String> {
    let count_address = region_base(regions, profile.party_region)
        .checked_add(profile.party_count_offset)
        .ok_or_else(|| "Party count address overflowed.".to_string())?;

    let data_address = region_base(regions, profile.party_region)
        .checked_add(profile.party_data_offset)
        .ok_or_else(|| "Party data address overflowed.".to_string())?;

    Ok((count_address, data_address))
}

#[cfg(test)]
mod tests {
    use super::{FIRERED_PROFILE, LEAFGREEN_PROFILE, RUBY_PROFILE, SAPPHIRE_PROFILE};

    #[test]
    fn ruby_and_sapphire_party_layout_uses_correct_count_and_data_offsets() {
        assert_eq!(RUBY_PROFILE.party_count_offset, 0x4350);
        assert_eq!(RUBY_PROFILE.party_data_offset, 0x4360);
        assert_eq!(SAPPHIRE_PROFILE.party_count_offset, 0x4350);
        assert_eq!(SAPPHIRE_PROFILE.party_data_offset, 0x4360);
    }

    #[test]
    fn firered_us_party_layout_matches_known_ewram_addresses() {
        assert_eq!(FIRERED_PROFILE.party_count_offset, 0x24029);
        assert_eq!(FIRERED_PROFILE.party_data_offset, 0x24284);
        assert_eq!(FIRERED_PROFILE.wild_party_offset, 0x2402C);
        assert_eq!(FIRERED_PROFILE.battle_type_flags_offset, 0x22B4C);
    }

    #[test]
    fn leafgreen_uses_the_same_kanto_layout() {
        assert_eq!(
            LEAFGREEN_PROFILE.party_count_offset,
            FIRERED_PROFILE.party_count_offset
        );
        assert_eq!(
            LEAFGREEN_PROFILE.party_data_offset,
            FIRERED_PROFILE.party_data_offset
        );
        assert_eq!(
            LEAFGREEN_PROFILE.wild_party_offset,
            FIRERED_PROFILE.wild_party_offset
        );
        assert_eq!(
            LEAFGREEN_PROFILE.battle_type_flags_offset,
            FIRERED_PROFILE.battle_type_flags_offset
        );
    }
}
