
from core.memory_reader import VBAMemoryReader
from core.wild_reader import WildPokemonReader


reader = VBAMemoryReader()

try:
    reader.connect()

    wild_reader = WildPokemonReader(reader)

    flags = wild_reader.read_battle_type_flags()

    print(f"\nBattle Type Flags: {flags:#010X}")
    print(f"Battle Flag Value: {flags}")

    pokemon = wild_reader.read_candidate()

    if pokemon is None:
        print("\nNo valid opponent Pokemon found.")
    else:
        print("\n========== OPPONENT CANDIDATE ==========")
        print(f"PID:       {pokemon['pid']}")
        print(f"OTID:      {pokemon['otid']}")
        print(f"Species:   {pokemon['species']}")
        print(f"Nature ID: {pokemon['nature_id']}")
        print(f"Shiny:     {pokemon['shiny']}")
        print(f"IVs:       {pokemon['ivs']}")

finally:
    reader.disconnect()
