# Emerald Shiny Scanner

A Pokémon Emerald companion application for Windows that reads Pokémon data directly from VisualBoyAdvance and displays information such as IVs, nature, shiny status, species, and abilities through a separate PySide6 interface.

**Project status:** Early Development — v0.1.0

**Repository:** [Karuki00/EmeraldShinyScanner](https://github.com/Karuki00/EmeraldShinyScanner)

**Developer:** [Karuki00](https://github.com/Karuki00)

---

## Overview

Emerald Shiny Scanner is a companion tool for Pokémon Emerald players who want to inspect their Pokémon without repeatedly navigating external calculators.

The application connects to a running VisualBoyAdvance process, reads relevant game memory, decodes Pokémon data using Generation III structures, and presents the results in a dedicated graphical interface.

The project is being developed with an emphasis on accuracy, lightweight operation, and a Pokémon-themed user experience.

## Current Features

- **Live Party Monitoring** — Reads Pokémon data from the player's party.
- **IV Calculation** — Extracts HP, Attack, Defense, Special Attack, Special Defense, and Speed IVs.
- **Shiny Detection** — Determines whether a Pokémon is shiny using its personality value and original trainer ID.
- **Nature Identification** — Displays the Pokémon's nature.
- **Ability Identification** — Resolves the Pokémon's ability name.
- **Species Identification** — Identifies Pokémon using Generation III species data.
- **PySide6 Interface** — Displays Pokémon information in a separate desktop window.

## Requirements

- Windows 10 or Windows 11
- Python 3.14 or a compatible version supported by the dependencies
- VisualBoyAdvance 1.8 Beta 3
- A legally obtained Pokémon Emerald ROM

## Installation

### 1. Clone the repository

```bash
git clone https://github.com/Karuki00/EmeraldShinyScanner.git
cd EmeraldShinyScanner
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
EmeraldShinyScanner/
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
├── EmeraldShinyScanner.spec
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
- [x] Wild Pokémon IV analysis
- [x] Encounter counter and statistics
- [ ] Further UI refinement
- [ ] Improved validation and error handling
- [ ] Public Windows release

## Building

The project includes a PyInstaller specification file.

To build the application:

```bash
pyinstaller EmeraldShinyScanner.spec
```

The resulting build is intended to be distributed with its required assets and dependencies.

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
