
import struct


POKEMON_DATA_SIZE = 80
ENCRYPTED_DATA_OFFSET = 32
ENCRYPTED_DATA_SIZE = 48
CHECKSUM_OFFSET = 28


def validate_pokemon_data(data):
    """
    Validate a Generation III Pokemon data structure.

    Checks:
    - Minimum data length
    - Non-empty personality and trainer IDs
    - Pokemon data checksum
    """

    if data is None or len(data) < POKEMON_DATA_SIZE:
        return False

    pid = struct.unpack_from("<I", data, 0)[0]
    otid = struct.unpack_from("<I", data, 4)[0]

    # Empty or uninitialized data
    if pid == 0 and otid == 0:
        return False

    stored_checksum = struct.unpack_from(
        "<H", data, CHECKSUM_OFFSET
    )[0]

    encryption_key = pid ^ otid
    encrypted = data[
        ENCRYPTED_DATA_OFFSET:
        ENCRYPTED_DATA_OFFSET + ENCRYPTED_DATA_SIZE
    ]

    decrypted = bytearray(ENCRYPTED_DATA_SIZE)

    for i in range(0, ENCRYPTED_DATA_SIZE, 4):
        value = struct.unpack_from("<I", encrypted, i)[0]
        value ^= encryption_key
        struct.pack_into("<I", decrypted, i, value)

    calculated_checksum = 0

    for i in range(0, ENCRYPTED_DATA_SIZE, 2):
        value = struct.unpack_from("<H", decrypted, i)[0]
        calculated_checksum = (
            calculated_checksum + value
        ) & 0xFFFF

    return calculated_checksum == stored_checksum
