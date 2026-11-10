
import pymem
import pymem.process

from core.gen3_decoder import decode_pokemon

PROCESS_NAME = "VisualBoyAdvance.exe"

SIGNATURE = bytes.fromhex(
    "81 E6 FF FF 03 00 8B 15"
)


def find_pattern(data, pattern):
    for i in range(len(data) - len(pattern) + 1):
        if data[i:i + len(pattern)] == pattern:
            return i
    return None


pm = pymem.Pymem(PROCESS_NAME)

module = pymem.process.module_from_name(
    pm.process_handle,
    PROCESS_NAME
)

base = module.lpBaseOfDll

image = pm.read_bytes(
    base,
    module.SizeOfImage
)

offset = find_pattern(image, SIGNATURE)

if offset is None:
    raise RuntimeError("VBA RAM signature not found")

pointer_address = pm.read_uint(base + offset + 8)

ewram = pm.read_uint(pointer_address)

print("\n========== ORIGINAL MEMORY DEBUG ==========")

print(f"Module base:       {base:#010x}")
print(f"Signature offset:  {offset:#010x}")
print(f"Pointer address:   {pointer_address:#010x}")
print(f"EWRAM:             {ewram:#010x}")

print(
    f"Count address:     "
    f"{ewram + 0x244E9:#010x}"
)

print(
    f"Party count:       "
    f"{pm.read_uchar(ewram + 0x244E9)}"
)

print("===========================================")

party_count = pm.read_uchar(ewram + 0x244E9)

print(f"Party count: {party_count}")

if party_count == 0:
    print("No Pokemon in party.")
    raise SystemExit

pokemon_data = pm.read_bytes(
    ewram + 0x244EC,
    100
)

pokemon = decode_pokemon(pokemon_data)

print("\n========== POKEMON ==========")
print(f"PID: {pokemon['pid']}")
print(f"OTID: {pokemon['otid']}")
print(f"Species ID: {pokemon['species']}")
print(f"Nature ID: {pokemon['nature_id']}")
print(f"Shiny: {pokemon['shiny']}")
print(f"Ability: {pokemon['ability_number']}")

print("\n========== IVs ==========")

for stat, value in pokemon["ivs"].items():
    print(f"{stat:12}: {value}")
