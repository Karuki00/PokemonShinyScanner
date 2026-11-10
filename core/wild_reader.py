
import struct

from .gen3_decoder import decode_pokemon
from .validator import validate_pokemon_data


# Pokemon Emerald opponent party in EWRAM.
WILD_PARTY_OFFSET = 0x24744

# gBattleTypeFlags in EWRAM.
BATTLE_TYPE_FLAGS_OFFSET = 0x22FEC

# Trainer battle bit.
BATTLE_TYPE_TRAINER = 0x08

POKEMON_SIZE = 100


class WildPokemonReader:
    def __init__(self, memory_reader):
        self.reader = memory_reader

    def read_candidate(self):
        """Read and decode the first opponent-party slot."""

        if self.reader.pm is None:
            raise RuntimeError("VBA is not connected")

        ewram = self.reader.get_ewram()

        raw_data = self.reader.pm.read_bytes(
            ewram + WILD_PARTY_OFFSET,
            POKEMON_SIZE
        )

        if not validate_pokemon_data(raw_data):
            return None

        try:
            pokemon = decode_pokemon(raw_data)
            pokemon["party_slot"] = 1
            return pokemon

        except (ValueError, struct.error):
            return None

    def read_battle_type_flags(self):
        """Read Emerald's battle type flags."""

        if self.reader.pm is None:
            raise RuntimeError("VBA is not connected")

        ewram = self.reader.get_ewram()

        return self.reader.pm.read_uint(
            ewram + BATTLE_TYPE_FLAGS_OFFSET
        )

    def is_trainer_battle(self, flags=None):
        """Check whether the current battle is a trainer battle."""

        if flags is None:
            flags = self.read_battle_type_flags()

        return bool(flags & BATTLE_TYPE_TRAINER)
