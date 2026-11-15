#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ivs {
    pub hp: u8,
    pub attack: u8,
    pub defense: u8,
    pub speed: u8,
    pub special_attack: u8,
    pub special_defense: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PokemonData {
    pub pid: u32,
    pub otid: u32,

    pub trainer_id: u16,
    pub secret_id: u16,

    pub species: u16,
    pub nature_id: u8,
    pub shiny: bool,
    pub ability_number: u8,

    pub checksum_valid: bool,
    pub stored_checksum: u16,
    pub calculated_checksum: u16,

    pub ivs: Ivs,
}

// Generation III substructure permutations.
// G = Growth, A = Attacks, E = EVs, M = Misc.
const SUBSTRUCTURE_ORDERS: [[u8; 4]; 24] = [
    *b"GAEM", *b"GAME", *b"GEAM", *b"GEMA", *b"GMAE", *b"GMEA", *b"AGEM", *b"AGME", *b"AEGM",
    *b"AEMG", *b"AMGE", *b"AMEG", *b"EGAM", *b"EGMA", *b"EAGM", *b"EAMG", *b"EMGA", *b"EMAG",
    *b"MGAE", *b"MGEA", *b"MAGE", *b"MAEG", *b"MEGA", *b"MEAG",
];

// Map a substructure identifier to its destination index.
fn block_index(name: u8) -> usize {
    match name {
        b'G' => 0,
        b'A' => 1,
        b'E' => 2,
        b'M' => 3,
        _ => unreachable!("Invalid substructure identifier"),
    }
}

// Read a little-endian u16.
fn read_u16(data: &[u8], offset: usize) -> u16 {
    u16::from_le_bytes([data[offset], data[offset + 1]])
}

// Read a little-endian u32.
fn read_u32(data: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes([
        data[offset],
        data[offset + 1],
        data[offset + 2],
        data[offset + 3],
    ])
}

/// Decode a Generation III Pokémon structure.
///
/// Accepts at least 80 bytes of Pokémon data.
/// Returns decoded information or an error.
pub fn decode_pokemon(data: &[u8]) -> Result<PokemonData, String> {
    if data.len() < 80 {
        return Err("Pokemon data must be at least 80 bytes".to_string());
    }

    // --------------------------------------------------------
    // Basic information
    // --------------------------------------------------------

    let pid = read_u32(data, 0);
    let otid = read_u32(data, 4);

    let trainer_id = (otid & 0xFFFF) as u16;
    let secret_id = (otid >> 16) as u16;

    // --------------------------------------------------------
    // Encryption key
    // --------------------------------------------------------

    let encryption_key = pid ^ otid;

    // --------------------------------------------------------
    // Determine substructure order
    // --------------------------------------------------------

    let order_index = (pid % 24) as usize;
    let order = SUBSTRUCTURE_ORDERS[order_index];

    let encrypted = &data[32..80];

    // --------------------------------------------------------
    // Decrypt the 48-byte payload
    // --------------------------------------------------------

    let mut decrypted = [0u8; 48];

    for i in (0..48).step_by(4) {
        let value = read_u32(encrypted, i);
        let decrypted_value = value ^ encryption_key;

        decrypted[i..i + 4].copy_from_slice(&decrypted_value.to_le_bytes());
    }

    // --------------------------------------------------------
    // Extract substructures
    // --------------------------------------------------------

    let mut blocks = [[0u8; 12]; 4];

    for (i, &block_name) in order.iter().enumerate() {
        let start = i * 12;
        let end = start + 12;

        let destination = block_index(block_name);

        blocks[destination].copy_from_slice(&decrypted[start..end]);
    }

    let growth = &blocks[0];
    let _attacks = &blocks[1];
    let _evs = &blocks[2];
    let misc = &blocks[3];

    // --------------------------------------------------------
    // Checksum validation
    // --------------------------------------------------------

    let stored_checksum = read_u16(data, 28);

    let calculated_checksum = decrypted.chunks_exact(2).fold(0u16, |sum, chunk| {
        sum.wrapping_add(u16::from_le_bytes([chunk[0], chunk[1]]))
    });

    let checksum_valid = stored_checksum == calculated_checksum;

    if !checksum_valid {
        return Err(format!(
            "Invalid Pokemon checksum: expected {:04X}, calculated {:04X}",
            stored_checksum, calculated_checksum
        ));
    }

    // --------------------------------------------------------
    // Species
    // --------------------------------------------------------

    let species = read_u16(growth, 0);

    // --------------------------------------------------------
    // IVs
    // --------------------------------------------------------

    let iv_data = read_u32(misc, 4);

    let ivs = Ivs {
        hp: (iv_data & 0x1F) as u8,
        attack: ((iv_data >> 5) & 0x1F) as u8,
        defense: ((iv_data >> 10) & 0x1F) as u8,
        speed: ((iv_data >> 15) & 0x1F) as u8,
        special_attack: ((iv_data >> 20) & 0x1F) as u8,
        special_defense: ((iv_data >> 25) & 0x1F) as u8,
    };

    // --------------------------------------------------------
    // Nature
    // --------------------------------------------------------

    let nature_id = (pid % 25) as u8;

    // --------------------------------------------------------
    // Shiny calculation
    // --------------------------------------------------------

    let shiny_value = trainer_id ^ secret_id ^ (pid as u16) ^ ((pid >> 16) as u16);

    let shiny = shiny_value < 8;

    // --------------------------------------------------------
    // Ability
    // --------------------------------------------------------

    let ability_number = (pid & 1) as u8;

    // --------------------------------------------------------
    // Return decoded Pokémon
    // --------------------------------------------------------

    Ok(PokemonData {
        pid,
        otid,
        trainer_id,
        secret_id,
        species,
        nature_id,
        shiny,
        ability_number,
        checksum_valid,
        stored_checksum,
        calculated_checksum,
        ivs,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_data_shorter_than_80_bytes() {
        let pokemon_data = [0u8; 79];

        let result = decode_pokemon(&pokemon_data);

        assert!(result.is_err());
    }

    #[test]
    fn rejects_invalid_checksum() {
        let mut pokemon_data = [0u8; 80];

        // Set stored checksum to 1 while calculated checksum remains 0.
        pokemon_data[28] = 1;

        let result = decode_pokemon(&pokemon_data);

        assert!(result.is_err());
    }

    #[test]
    fn decodes_real_pokemon_correctly() {
        let pokemon_data: [u8; 80] = [
            0xE2, 0xA9, 0xB0, 0x25, 0x26, 0xF3, 0xBB, 0x4A, 0xCE, 0xCC, 0xBF, 0xBF, 0xBD, 0xC5,
            0xC9, 0xFF, 0x01, 0x00, 0x02, 0x02, 0xC5, 0xD5, 0xE6, 0xE9, 0xDF, 0xDD, 0xFF, 0x00,
            0x49, 0xBC, 0x00, 0x00, 0xD1, 0x5B, 0x0B, 0x6F, 0x5C, 0x5A, 0x0B, 0x6F, 0xC4, 0x1C,
            0x0B, 0x6F, 0xC4, 0x5A, 0x0B, 0x6E, 0xC4, 0x5A, 0x0B, 0x6F, 0xC4, 0x5A, 0x0B, 0x6F,
            0xC5, 0x5A, 0x20, 0x6F, 0xC4, 0x5A, 0x0B, 0x6F, 0xE7, 0x44, 0x0B, 0x6F, 0xC4, 0x4A,
            0x8E, 0x4E, 0xE7, 0x50, 0xAE, 0x76, 0xC4, 0x5A, 0x0B, 0x6F,
        ];

        let pokemon =
            decode_pokemon(&pokemon_data).expect("Real Pokemon data should decode successfully");

        assert_eq!(pokemon.pid, 0x25B0A9E2);
        assert_eq!(pokemon.otid, 0x4ABBF326);

        assert_eq!(pokemon.trainer_id, 0xF326);
        assert_eq!(pokemon.secret_id, 0x4ABB);

        assert_eq!(pokemon.species, 277);
        assert_eq!(pokemon.nature_id, 18);

        assert!(!pokemon.shiny);
        assert_eq!(pokemon.ability_number, 0);

        assert!(pokemon.checksum_valid);
        assert_eq!(pokemon.stored_checksum, 0xBC49);
        assert_eq!(pokemon.calculated_checksum, 0xBC49);

        assert_eq!(pokemon.ivs.hp, 3);
        assert_eq!(pokemon.ivs.attack, 17);
        assert_eq!(pokemon.ivs.defense, 2);
        assert_eq!(pokemon.ivs.speed, 10);
        assert_eq!(pokemon.ivs.special_attack, 26);
        assert_eq!(pokemon.ivs.special_defense, 12);
    }
}
