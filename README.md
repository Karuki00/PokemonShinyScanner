# Pokémon Shiny Scanner

A multi-generation Pokémon companion application for Windows, starting with Pokémon Emerald. It reads Pokémon data directly from VisualBoyAdvance and displays information such as IVs, nature, shiny status, species, and abilities through a separate PySide6 interface.

**Project status:** Early Development — v0.1.0

**Current game support:** Pokémon Emerald (Generation III)

**Repository:** [Karuki00/PokemonShinyScanner](https://github.com/Karuki00/PokemonShinyScanner)

**Developer:** [Karuki00](https://github.com/Karuki00)

---

## Overview

Pokémon Shiny Scanner is a companion application designed to help Pokémon players inspect Pokémon data and track their shiny-hunting progress without repeatedly navigating external calculators.

The application connects to a running VisualBoyAdvance process, reads relevant game memory, decodes Pokémon data using Generation III structures, and presents the results in a dedicated graphical interface.

The project is designed with future multi-generation compatibility in mind. However, the initial v0.1.0 release focuses exclusively on Pokémon Emerald.

The long-term goal is to expand support to other Pokémon games across multiple generations.

## Current Features

- **Live Party Monitoring** — Reads and displays Pokémon data from the player's party.
- **Starter Pokémon Scanning** — Detects and analyzes starter Pokémon data.
- **Wild Encounter Detection** — Detects Pokémon encountered in the wild.
- **Wild Pokémon IV Analysis** — Calculates and displays IVs for wild Pokémon.
- **IV Calculation** — Extracts HP, Attack, Defense, Special Attack, Special Defense, and Speed IVs.
- **Shiny Detection** — Determines whether a Pokémon is shiny using its personality value and original trainer ID.
- **Nature Identification** — Identifies and displays the Pokémon's nature.
- **Ability Identification** — Resolves the Pokémon's ability name.
- **Species Identification** — Identifies Pokémon using Generation III species data.
- **Encounter Counter and Statistics** — Tracks shiny-hunting encounters and related statistics.
- **PySide6 Interface** — Displays Pokémon information through a separate desktop window.

## Supported Games and Compatibility

### Current Support

| Game | Generation | Status |
|---|---|---|
| Pokémon Emerald | III | Supported |

### Planned Expansion

| Game | Generation | Status |
|---|---|---|
| Pokémon Ruby | III | Planned |
| Pokémon Sapphire | III | Planned |
| Pokémon FireRed | III | Planned |
| Pokémon LeafGreen | III | Planned |
| Pokémon Gold / Silver / Crystal | II | Future consideration |
| Pokémon Diamond / Pearl / Platinum | IV | Future consideration |
| Pokémon HeartGold / SoulSilver | IV | Future consideration |

Game compatibility requires appropriate memory-reading profiles and, where necessary, generation-specific Pokémon data decoders.

## Requirements

- Windows 10 or Windows 11
- Python 3.14 or a compatible version supported by the dependencies
- VisualBoyAdvance 1.8 Beta 3
- A legally obtained Pokémon Emerald ROM

## Installation

### 1. Clone the repository

```bash
git clone https://github.com/Karuki00/PokemonShinyScanner.git
cd PokemonShinyScanner
```

### 2. Create a virtual environment

```bash
python -m venv .venv
```

Activate it:

```bash
.venv\Scripts\activate
```

### 3. Install dependencies

```bash
pip install -r requirements.txt
```

### 4. Prepare application data and sprites

The repository includes the generated Pokémon data module. If you need to regenerate it or download the sprite collection, run:

```bash
python generate_pokemon_data.py
python download_emerald_sprites.py
```

These scripts require an internet connection.

### 5. Launch the application

```bash
python main.py
```

Start Pokémon Emerald in VisualBoyAdvance before connecting the scanner.

## Project Structure

```text
PokemonShinyScanner/
│
├── assets/
│   ├── pc_box/
│   ├── sound_effects/
│   └── pokemon/
│       └── shiny/
│
├── core/
│   ├── __init__.py
│   ├── gen3_decoder.py
│   ├── memory_reader.py
│   ├── memory_scanner.py
│   ├── pokemon_data.py
│   ├── validator.py
│   └── wild_reader.py
│
├── gui/
│   ├── __init__.py
│   ├── main_window.py
│   ├── party_view.py
│   └── pokemon_card.py
│
├── download_emerald_sprites.py
├── generate_pokemon_data.py
├── PokemonShinyScanner.spec
├── main.py
├── requirements.txt
├── LICENSE
└── README.md
```

## Roadmap

- [x] Generation III Pokémon data decoding
- [x] Live party memory reading
- [x] IV calculation
- [x] Nature identification
- [x] Shiny detection
- [x] Species and ability identification
- [x] Initial PySide6 graphical interface
- [x] Wild encounter detection
- [x] Starter Pokémon scanning
- [x] Wild Pokémon IV analysis
- [x] Encounter counter and statistics
- [ ] Further UI refinement
- [ ] Improved validation and error handling
- [x] Public Windows release

## Building

The project includes a PyInstaller specification file.

To build the application:

```bash
pyinstaller PokemonShinyScanner.spec
```

The resulting Windows application is intended to be distributed with its required assets and dependencies.

## Contributing

Suggestions, bug reports, and contributions are welcome.

If you encounter an issue, please provide:

- A description of the issue.
- Steps to reproduce it.
- Relevant error messages or screenshots.
- Your VisualBoyAdvance version.

## License

The project's source code is distributed under the MIT License. See [LICENSE](LICENSE) for details.

Pokémon and all related names, characters, and assets are trademarks and copyrights of their respective owners. This project is an unofficial fan-made tool and is not affiliated with, endorsed by, or associated with Nintendo, The Pokémon Company, or Game Freak.

Pokémon game data and sprite assets retain their respective ownership and licensing considerations.
