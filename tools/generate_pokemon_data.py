
from pathlib import Path
import sys

PROJECT_ROOT = Path(__file__).resolve().parents[1]

SOURCE_FILE = PROJECT_ROOT / "core" / "pokemon_data.py"

OUTPUT_FILE = (
    PROJECT_ROOT
    / "tauri-app"
    / "src-tauri"
    / "src"
    / "games"
    / "gen3"
    / "pokemon_data.rs"
)


def generate():
    if not SOURCE_FILE.exists():
        raise FileNotFoundError(
            f"Pokemon database not found: {SOURCE_FILE}"
        )

    # Import the existing generated database.
    sys.path.insert(0, str(PROJECT_ROOT / "core"))

    from pokemon_data import POKEMON_DATA

    lines = [
        "// AUTO-GENERATED FILE. DO NOT EDIT MANUALLY.",
        "// Source: core/pokemon_data.py",
        "",
        "#[derive(Debug, Clone, Copy, PartialEq, Eq)]",
        "pub struct SpeciesInfo {",
        "    pub name: &'static str,",
        "    pub national_dex: u16,",
        "    pub abilities: [Option<&'static str>; 2],",
        "}",
        "",
        "pub fn species_info(species: u16) -> Option<SpeciesInfo> {",
        "    match species {",
    ]

    for species_id, pokemon in sorted(POKEMON_DATA.items()):
        name = pokemon["name"]
        national_dex = pokemon["national_dex"]
        abilities = pokemon["abilities"]

        ability_1 = (
            f'Some("{abilities[0]}")'
            if abilities[0]
            else "None"
        )

        ability_2 = (
            f'Some("{abilities[1]}")'
            if abilities[1]
            else "None"
        )

        lines.append(
            f'        {species_id} => Some(SpeciesInfo {{ '
            f'name: "{name}", '
            f'national_dex: {national_dex}, '
            f'abilities: [{ability_1}, {ability_2}], '
            f'}}),'
        )


    lines.extend([
        "        _ => None,",
        "    }",
        "}",
        "",
        "pub fn ability_name(species: u16, ability_number: u8) -> &'static str {",
        "    let Some(info) = species_info(species) else {",
        '        return "Unknown";',
        "    };",
        "",
        "    let index = if ability_number == 1 { 1 } else { 0 };",
        "",
        "    info.abilities[index]",
        "        .or(info.abilities[0])",
        '        .unwrap_or("Unknown")',
        "}",
        "",
        "pub fn nature_name(nature_id: u8) -> &'static str {",
        "    match nature_id {",
        '        0 => "Hardy",',
        '        1 => "Lonely",',
        '        2 => "Brave",',
        '        3 => "Adamant",',
        '        4 => "Naughty",',
        '        5 => "Bold",',
        '        6 => "Docile",',
        '        7 => "Relaxed",',
        '        8 => "Impish",',
        '        9 => "Lax",',
        '        10 => "Timid",',
        '        11 => "Hasty",',
        '        12 => "Serious",',
        '        13 => "Jolly",',
        '        14 => "Naive",',
        '        15 => "Modest",',
        '        16 => "Mild",',
        '        17 => "Quiet",',
        '        18 => "Bashful",',
        '        19 => "Rash",',
        '        20 => "Calm",',
        '        21 => "Gentle",',
        '        22 => "Sassy",',
        '        23 => "Careful",',
        '        24 => "Quirky",',
        '        _ => "Unknown",',
        "    }",
        "}",
        "",
    ])


    OUTPUT_FILE.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT_FILE.write_text("\n".join(lines), encoding="utf-8")

    print("Pokemon database generated successfully.")
    print(f"Species entries: {len(POKEMON_DATA)}")
    print(f"Output: {OUTPUT_FILE}")


if __name__ == "__main__":
    generate()
