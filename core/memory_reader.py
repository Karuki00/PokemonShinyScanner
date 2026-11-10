import pymem
import pymem.process
import struct

from .gen3_decoder import decode_pokemon
from .validator import validate_pokemon_data


PROCESS_NAME = "VisualBoyAdvance.exe"

SIGNATURE = bytes.fromhex(
    "81 E6 FF FF 03 00 8B 15"
)

PARTY_COUNT_OFFSET = 0x244E9
PARTY_DATA_OFFSET = 0x244EC

POKEMON_SIZE = 100
MAX_PARTY_SIZE = 6


def find_pattern(data, pattern):
    for i in range(len(data) - len(pattern) + 1):
        if data[i:i + len(pattern)] == pattern:
            return i

    return None


class VBAMemoryReader:

    def __init__(self):
        self.pm = None
        self.ewram_pointer_address = None

    def connect(self):
        """Connect to VBA and locate its EWRAM pointer."""

        if self.pm is not None:
            return True

        self.pm = pymem.Pymem(PROCESS_NAME)

        try:
            module = pymem.process.module_from_name(
                self.pm.process_handle,
                PROCESS_NAME
            )

            if module is None:
                raise RuntimeError("VBA module not found")

            base = module.lpBaseOfDll

            image = self.pm.read_bytes(
                base,
                module.SizeOfImage
            )

            offset = find_pattern(image, SIGNATURE)

            if offset is None:
                raise RuntimeError(
                    "VBA RAM signature not found"
                )

            # Resolve the address containing the EWRAM pointer.
            self.ewram_pointer_address = self.pm.read_uint(
                base + offset + 8
            )

            # Verify the actual EWRAM address.
            ewram = self.get_ewram()

            if not ewram:
                raise RuntimeError("Invalid EWRAM pointer")

            print(f"Pointer address: {self.ewram_pointer_address:#010x}")
            print(f"EWRAM address:   {ewram:#010x}")

            return True

        except Exception:
            self.disconnect()
            raise

    def get_ewram(self):
        """Resolve the current EWRAM base address."""

        if self.pm is None:
            raise RuntimeError("VBA is not connected")

        if self.ewram_pointer_address is None:
            raise RuntimeError("EWRAM pointer not initialized")

        # Dereference the pointer to obtain actual EWRAM.
        return self.pm.read_uint(
            self.ewram_pointer_address
        )

    
    def read_party(self):
        """Read, validate, and decode all Pokémon in the party."""
    
        if self.pm is None:
            raise RuntimeError("VBA is not connected")
    
        ewram = self.get_ewram()
    
        if not ewram:
            raise RuntimeError("Invalid EWRAM address")
    
        party_count = self.pm.read_uchar(
            ewram + PARTY_COUNT_OFFSET
        )
    
        if party_count > MAX_PARTY_SIZE:
            raise RuntimeError(
                f"Invalid party count: {party_count}"
            )
    
        if party_count == 0:
            return []
    
        raw_party = self.pm.read_bytes(
            ewram + PARTY_DATA_OFFSET,
            party_count * POKEMON_SIZE
        )
    
        pokemon_list = []
    
        for index in range(party_count):
            start = index * POKEMON_SIZE
            pokemon_data = raw_party[start:start + POKEMON_SIZE]
    
            if not validate_pokemon_data(pokemon_data):
                print(f"Party slot {index + 1} rejected: Invalid checksum")
                continue
            
            try:
                pokemon = decode_pokemon(pokemon_data)
                pokemon["party_slot"] = index + 1
                pokemon_list.append(pokemon)
    
            except (ValueError, struct.error) as error:
                print(f"Party slot {index + 1} rejected: {error}")
    
        return pokemon_list


    def disconnect(self):
        """Close the VBA process connection."""

        if self.pm is not None:
            self.pm.close_process()
            self.pm = None

        self.ewram_pointer_address = None
