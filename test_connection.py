
import pymem
import pymem.process

PROCESS_NAME = "VisualBoyAdvance.exe"

# Signatures taken from the VBA 1.8.0 Beta 3
# compatibility implementation in LiveSplit ASR.
EWRAM_SIGNATURE = bytes.fromhex(
    "81 E6 FF FF 03 00 8B 15"
)

RUNNING_SIGNATURE = bytes.fromhex(
    "8B 0D"
)


def find_pattern(data, pattern, mask):
    for i in range(len(data) - len(pattern) + 1):
        if all(
            mask[j] == "?" or data[i + j] == pattern[j]
            for j in range(len(pattern))
        ):
            return i
    return None


try:
    pm = pymem.Pymem(PROCESS_NAME)

    print("[+] Connected to VBA!")

    module = pymem.process.module_from_name(
        pm.process_handle,
        PROCESS_NAME
    )

    base = module.lpBaseOfDll
    size = module.SizeOfImage

    print(f"[+] Module base: {hex(base)}")
    print(f"[+] Module size: {hex(size)}")

    image = pm.read_bytes(base, size)

    # Find EWRAM pointer signature
    ewram_offset = find_pattern(
        image,
        EWRAM_SIGNATURE,
        "xxxxxxxx"
    )

    if ewram_offset is None:
        raise RuntimeError("EWRAM signature not found")

    ewram_pointer_address = pm.read_uint(
        base + ewram_offset + 8
    )

    # IWRAM pointer is stored immediately after EWRAM pointer
    iwram_pointer_address = ewram_pointer_address + 4

    ewram = pm.read_uint(ewram_pointer_address)
    iwram = pm.read_uint(iwram_pointer_address)

    print("[+] RAM pointers found!")
    print(f"EWRAM: {hex(ewram)}")
    print(f"IWRAM: {hex(iwram)}")

    # Read the beginning of EWRAM
    test = pm.read_bytes(ewram, 16)

    print("[+] EWRAM test:", test.hex(" "))

    # Read Emerald player party structure
    party = pm.read_bytes(
        ewram + 0x244EC,
        100
    )

    print("[+] Player party memory:", party.hex(" "))

    party_count = pm.read_uchar(ewram + 0x244E9)

    print(f"Party count: {party_count}")
    
    for i in range(6):
        pokemon = pm.read_bytes(
            ewram + 0x244EC + (i * 100),
            100
        )

    print(f"Party slot {i + 1}: {pokemon[:8].hex(' ')}")

except Exception as e:
    print("[-] Error:", e)
    print("Check that VBA is running and its process name is correct.")
