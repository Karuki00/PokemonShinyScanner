from core.memory_reader import VBAMemoryReader


reader = VBAMemoryReader()

try:
    reader.connect()

    print("Connected to VisualBoyAdvance!")

    pokemon_list = reader.read_party()

    print(f"\nParty count: {len(pokemon_list)}")

    for pokemon in pokemon_list:

        print("\n========== POKEMON ==========")

        print(f"Party Slot: {pokemon['party_slot']}")
        print(f"PID: {pokemon['pid']}")
        print(f"OTID: {pokemon['otid']}")
        print(f"Species ID: {pokemon['species']}")
        print(f"Nature ID: {pokemon['nature_id']}")
        print(f"Shiny: {pokemon['shiny']}")
        print(f"Ability: {pokemon['ability_number']}")

        print("\n========== IVs ==========")

        for stat, value in pokemon["ivs"].items():
            print(f"{stat:12}: {value}")

finally:
    reader.disconnect()
