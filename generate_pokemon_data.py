
import csv
import re
import urllib.request
from pathlib import Path

BASE_DIR = Path(__file__).resolve().parent
CORE_DIR = BASE_DIR / "core"
OUTPUT_FILE = CORE_DIR / "pokemon_data.py"

SOURCES = {
    "species": (
        "https://raw.githubusercontent.com/pret/pokeemerald/"
        "master/include/constants/species.h"
    ),
    "species_info": (
        "https://raw.githubusercontent.com/pret/pokeemerald/"
        "master/src/data/pokemon/species_info.h"
    ),
    "national_dex": (
        "https://raw.githubusercontent.com/veekun/pokedex/"
        "master/pokedex/data/csv/pokemon_species.csv"
    ),
}

SPECIAL_NAMES = {
    "farfetchd": "Farfetch'd",
    "mr-mime": "Mr. Mime",
    "nidoran-f": "Nidoran♀",
    "nidoran-m": "Nidoran♂",
    "ho-oh": "Ho-Oh",
}


def download_text(url):
    request = urllib.request.Request(
        url,
        headers={"User-Agent": "EmeraldIVScanner/1.0"}
    )

    with urllib.request.urlopen(request, timeout=30) as response:
        return response.read().decode("utf-8")


def display_name(identifier):
    if identifier in SPECIAL_NAMES:
        return SPECIAL_NAMES[identifier]

    return " ".join(
        word.capitalize()
        for word in identifier.split("-")
    )


def main():
    print("Downloading Emerald source data...")

    species_text = download_text(SOURCES["species"])
    info_text = download_text(SOURCES["species_info"])
    dex_text = download_text(SOURCES["national_dex"])

    national_dex = {}

    for row in csv.DictReader(dex_text.splitlines()):
        national_dex[row["identifier"]] = int(row["id"])

    # Internal species name -> internal species ID
    species_ids = {}

    for name, value in re.findall(
        r"#define\s+SPECIES_([A-Z0-9_]+)\s+(\d+)",
        species_text
    ):
        species_ids[name] = int(value)

    # Internal species name -> two Emerald ability slots
    abilities = {}

    pattern = re.compile(
        r"\[SPECIES_([A-Z0-9_]+)\]\s*=\s*\{(.*?)(?=\n\s*\},)",
        re.DOTALL
    )

    for match in pattern.finditer(info_text):
        species_name = match.group(1)
        block = match.group(2)

        ability_match = re.search(
            r"\.abilities\s*=\s*\{\s*"
            r"ABILITY_([A-Z0-9_]+)\s*,\s*"
            r"ABILITY_([A-Z0-9_]+)",
            block
        )

        if not ability_match:
            continue

        slots = []

        for ability in ability_match.groups():
            if ability == "NONE":
                slots.append(None)
            else:
                slots.append(
                    " ".join(
                        word.capitalize()
                        for word in ability.split("_")
                    )
                )

        abilities[species_name] = slots

    pokemon_data = {}

    for internal_name, internal_id in species_ids.items():

        # Ignore empty, egg and obsolete Unown entries.
        if internal_name in ("NONE", "EGG"):
            continue

        if internal_name.startswith("OLD_UNOWN_"):
            continue

        identifier = internal_name.lower().replace("_", "-")

        dex_number = national_dex.get(identifier)

        if dex_number is None:
            print(
                f"Skipping {internal_name}: "
                "National Dex mapping not found"
            )
            continue

        species_abilities = abilities.get(
            internal_name,
            [None, None]
        )

        # Pokémon with one ability use that ability
        # regardless of the PID ability bit.
        if not species_abilities[1]:
            species_abilities[1] = species_abilities[0]

        pokemon_data[internal_id] = {
            "name": display_name(identifier),
            "national_dex": dex_number,
            "abilities": tuple(species_abilities),
        }

    if 277 not in pokemon_data:
        raise RuntimeError("Treecko mapping was not generated.")

    treecko = pokemon_data[277]

    if treecko["name"] != "Treecko":
        raise RuntimeError("Treecko name mapping is incorrect.")

    if treecko["national_dex"] != 252:
        raise RuntimeError("Treecko National Dex mapping is incorrect.")

    if treecko["abilities"][0] != "Overgrow":
        raise RuntimeError("Treecko ability mapping is incorrect.")

    CORE_DIR.mkdir(parents=True, exist_ok=True)

    with OUTPUT_FILE.open("w", encoding="utf-8") as file:
        file.write(
            "# Generated from pret/pokeemerald and veekun/pokedex.\n"
            "# Do not edit manually.\n\n"
        )

        file.write("POKEMON_DATA = {\n")

        for internal_id, entry in sorted(pokemon_data.items()):
            file.write(
                f"    {internal_id}: {entry!r},\n"
            )

        file.write("}\n\n")

        file.write(
            "def get_pokemon(species_id):\n"
            "    return POKEMON_DATA.get(species_id)\n\n"
        )

        file.write(
            "def get_ability(species_id, ability_number):\n"
            "    pokemon = get_pokemon(species_id)\n"
            "    if pokemon is None:\n"
            "        return 'Unknown'\n"
            "    abilities = pokemon['abilities']\n"
            "    index = 1 if ability_number else 0\n"
            "    return abilities[index] or abilities[0] or 'Unknown'\n"
        )

    print()
    print("Database generated successfully.")
    print(f"Pokémon entries: {len(pokemon_data)}")
    print(f"Output: {OUTPUT_FILE}")
    print()
    print("Treecko verification:")
    print(f"Internal ID: 277")
    print(f"Name: {treecko['name']}")
    print(f"National Dex: {treecko['national_dex']}")
    print(f"Abilities: {treecko['abilities']}")


if __name__ == "__main__":
    main()
