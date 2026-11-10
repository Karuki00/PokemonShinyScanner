
import struct


# ============================================================
# Generation 3 Pokemon Data Decoder
# Pokemon Ruby / Sapphire / Emerald / FireRed / LeafGreen
# ============================================================

def decode_pokemon(data):
    if len(data) < 80:
        raise ValueError("Pokemon data must be at least 80 bytes")

    # --------------------------------------------------------
    # Basic information
    # --------------------------------------------------------
    pid = struct.unpack_from("<I", data, 0)[0]
    otid = struct.unpack_from("<I", data, 4)[0]

    personality = pid
    trainer_id = otid & 0xFFFF
    secret_id = (otid >> 16) & 0xFFFF

    # --------------------------------------------------------
    # Encryption key
    # --------------------------------------------------------
    encryption_key = pid ^ otid

    # --------------------------------------------------------
    # Determine substructure order
    # --------------------------------------------------------
    # Growth, Attacks, EVs, Misc
    orders = [
        "GAEM", "GAME", "GEAM", "GEMA",
        "GMAE", "GMEA", "AGEM", "AGME",
        "AEGM", "AEMG", "AMGE", "AMEG",
        "EGAM", "EGMA", "EAGM", "EAMG",
        "EMGA", "EMAG", "MGAE", "MGEA",
        "MAGE", "MAEG", "MEGA", "MEAG"
    ]

    order = orders[pid % 24]

    encrypted = data[32:80]

    # Decrypt the 48-byte payload
    decrypted = bytearray(48)

    for i in range(0, 48, 4):
        value = struct.unpack_from("<I", encrypted, i)[0]
        value ^= encryption_key
        struct.pack_into("<I", decrypted, i, value)

    # --------------------------------------------------------
    # Extract substructures
    # --------------------------------------------------------
    blocks = {}

    for i, block_name in enumerate(order):
        blocks[block_name] = decrypted[i * 12:(i + 1) * 12]

    growth = blocks["G"]
    attacks = blocks["A"]
    evs = blocks["E"]
    misc = blocks["M"]

    # --------------------------------------------------------
    # Checksum validation
    # --------------------------------------------------------
    stored_checksum = struct.unpack_from("<H", data, 28)[0]

    calculated_checksum = sum(
        struct.unpack_from("<H", decrypted, i)[0]
        for i in range(0, 48, 2)
    ) & 0xFFFF

    checksum_valid = stored_checksum == calculated_checksum

    if not checksum_valid:
        raise ValueError(
            f"Invalid Pokemon checksum: "
            f"expected {stored_checksum:04X}, "
            f"calculated {calculated_checksum:04X}"
        )

    # --------------------------------------------------------
    # Species
    # --------------------------------------------------------
    species = struct.unpack_from("<H", growth, 0)[0]

    # --------------------------------------------------------
    # IVs
    # --------------------------------------------------------
    iv_data = struct.unpack_from("<I", misc, 4)[0]

    hp_iv  = (iv_data >>  0) & 0x1F
    atk_iv = (iv_data >>  5) & 0x1F
    def_iv = (iv_data >> 10) & 0x1F
    spe_iv = (iv_data >> 15) & 0x1F
    spa_iv = (iv_data >> 20) & 0x1F
    spd_iv = (iv_data >> 25) & 0x1F


    # --------------------------------------------------------
    # Nature
    # --------------------------------------------------------
    nature = pid % 25

    # --------------------------------------------------------
    # Shiny calculation
    # --------------------------------------------------------
    shiny_value = trainer_id ^ secret_id ^ (pid & 0xFFFF) ^ (pid >> 16)

    shiny = shiny_value < 8

    # --------------------------------------------------------
    # Ability
    # --------------------------------------------------------
    ability = pid & 1

    return {
        "pid": f"{pid:08X}",
        "otid": f"{otid:08X}",
        "species": species,
        "nature_id": nature,
        "shiny": shiny,
        "ability_number": ability,
        "checksum_valid": checksum_valid,
        "stored_checksum": f"{stored_checksum:04X}",
        "calculated_checksum": f"{calculated_checksum:04X}",
        "ivs": {
            "HP": hp_iv,
            "Attack": atk_iv,
            "Defense": def_iv,
            "SpAttack": spa_iv,
            "SpDefense": spd_iv,
            "Speed": spe_iv
        }
    }


if __name__ == "__main__":
    print("Gen 3 Pokemon Decoder loaded successfully.")
