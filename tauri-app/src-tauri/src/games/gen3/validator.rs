const POKEMON_DATA_SIZE: usize = 80;
const ENCRYPTED_DATA_OFFSET: usize = 32;
const ENCRYPTED_DATA_SIZE: usize = 48;
const CHECKSUM_OFFSET: usize = 28;

/// Validate a Generation III Pokémon data structure.
pub fn validate_pokemon_data(data: &[u8]) -> bool {
    if data.len() < POKEMON_DATA_SIZE {
        return false;
    }

    let pid = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);

    let otid = u32::from_le_bytes([data[4], data[5], data[6], data[7]]);

    // Reject empty or uninitialized data.
    if pid == 0 && otid == 0 {
        return false;
    }

    let stored_checksum = u16::from_le_bytes([data[CHECKSUM_OFFSET], data[CHECKSUM_OFFSET + 1]]);

    let encryption_key = pid ^ otid;

    let encrypted = &data[ENCRYPTED_DATA_OFFSET..ENCRYPTED_DATA_OFFSET + ENCRYPTED_DATA_SIZE];

    let mut calculated_checksum: u32 = 0;

    for chunk in encrypted.chunks_exact(4) {
        let value = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]) ^ encryption_key;

        let bytes = value.to_le_bytes();

        for half in bytes.chunks_exact(2) {
            let value = u16::from_le_bytes([half[0], half[1]]);
            calculated_checksum = (calculated_checksum + value as u32) & 0xFFFF;
        }
    }

    calculated_checksum as u16 == stored_checksum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_data_shorter_than_80_bytes() {
        assert!(!validate_pokemon_data(&[0; 79]));
    }

    #[test]
    fn rejects_empty_pokemon_data() {
        assert!(!validate_pokemon_data(&[0; 100]));
    }

    #[test]
    fn rejects_invalid_checksum() {
        let mut data = vec![0u8; 100];

        // Give the data nonzero PID and OTID.
        data[0] = 1;
        data[4] = 1;

        // Deliberately set an incorrect stored checksum.
        data[28] = 1;
        data[29] = 0;

        assert!(!validate_pokemon_data(&data));
    }
}
