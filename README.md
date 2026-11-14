# Emerald IV Scanner

A Windows desktop application for monitoring and analyzing Pokémon IVs in **Pokémon Emerald** in real time.

Emerald IV Scanner reads Pokémon party data directly from VisualBoyAdvance memory, decodes Generation III Pokémon data, and presents the results through a graphical interface built with Python and PySide6.

> **Development status:** Early development (v0.1.0). Party Pokémon scanning is implemented. Wild encounter scanning is planned.

## Features

- **Live Party Scanning** — Reads Pokémon data directly from VisualBoyAdvance.
- **Generation III Decoder** — Decodes Pokémon personality values, Original Trainer IDs, species, natures, abilities, shininess, and individual values.
- **IV Analysis** — Displays HP, Attack, Defense, Special Attack, Special Defense, and Speed IVs.
- **Pokémon Interface** — Displays Pokémon using species data and Pokémon Emerald normal/shiny sprites.
- **VisualBoyAdvance Integration** — Connects to the running emulator process through Windows process memory reading.
- **Desktop GUI** — Built with PySide6.

## Screenshots

\<!-- Add application screenshots here -->

## Requirements

- Windows 10/11 (64-bit recommended)
- Python 3.14 or a compatible Python version
- VisualBoyAdvance 1.8 beta 3
- A legally obtained Pokémon Emerald ROM

## Installation

### Option 1: Download the application

1. Visit the [Releases](../../releases) page.
2. Download the latest Windows release.
3. Extract the downloaded ZIP archive.
4. Run `EmeraldIVScanner.exe`.

Python installation is not required when using the packaged application.

### Option 2: Run from source

Clone the repository:

```bash
git clone https://github.com/YOUR_USERNAME/EmeraldIVScanner.git
cd EmeraldIVScanner
```

Install the dependencies:

```bash
python -m pip install PySide6 pymem
```

Run the application:

```bash
python main.py
```

## Building from Source

Install PyInstaller:

```bash
python -m pip install pyinstaller
```

Build the Windows application:

```bash
python -m PyInstaller ^
    --noconfirm ^
    --clean ^
    --windowed ^
    --onedir ^
    --name EmeraldIVScanner ^
    --collect-all PySide6 ^
    --add-data "assets;assets" ^
    main.py
```

The packaged application will be generated inside:

```text
dist/EmeraldIVScanner/
```

## Project Structure

```text
EmeraldIVScanner/
│
├── main.py
├── core/
│   ├── gen3_decoder.py
│   ├── memory_reader.py
│   ├── memory_scanner.py
│   └── validator.py
│
├── gui/
│   ├── main_window.py
│   ├── party_view.py
│   └── pokemon_card.py
│
├── assets/
│   ├── pokemon/
│   ├── pc_box/
│   └── sound_effects/
│
├── tests/
│   └── ...
│
├── requirements.txt
├── .gitignore
└── README.md
```

## Preparing Application Data and Assets

The repository includes scripts for generating Pokémon species data and downloading Pokémon Emerald sprites.

These scripts require an internet connection.

### 1. Generate Pokémon species data

```bash
python generate_pokemon_data.py
```

This generates:

```text
core/pokemon_data.py
```

### 2. Download Pokémon Emerald sprites

```bash
python download_emerald_sprites.py
```

This downloads normal and shiny sprites into:

```text
assets/pokemon/
```

Existing valid sprites are skipped, allowing interrupted downloads to be resumed.

### 3. Launch the application

```bash
python main.py
```

**Note:** The generated species database is included in the repository. Running the generator again is optional unless you want to regenerate it from upstream data.

*The structure above illustrates the main application components; individual test files may remain in the project root.*

## Development Roadmap

- [x] Generation III Pokémon data decoder
- [x] VisualBoyAdvance memory reading
- [x] Party Pokémon detection
- [x] Individual Value calculation and display
- [x] Pokémon species data integration
- [x] Emerald normal and shiny sprite integration
- [x] PySide6 graphical interface
- [x] Wild encounter detection
- [x] Wild Pokémon IV analysis
- [ ] Improved validation and memory-reading reliability
- [ ] Application installer and distribution improvements

## Technology Stack

| Technology       | Purpose                       |
| ---------------- | ----------------------------- |
| Python           | Core application language     |
| PySide6          | Desktop graphical interface   |
| PyMem            | Windows process memory access |
| PyInstaller      | Windows executable packaging  |
| VisualBoyAdvance | Game emulator                 |

## Disclaimer

Emerald IV Scanner is an independent fan-made utility. It is not affiliated with, endorsed by, or associated with Nintendo, The Pokémon Company, Game Freak, or VisualBoyAdvance developers.

Pokémon and related characters, names, sprites, and trademarks belong to their respective owners.

This project does not include or distribute Pokémon Emerald ROM files. Users are responsible for obtaining and using game files in accordance with applicable laws.

## License

The original source code of Emerald IV Scanner is distributed under the MIT License. See [LICENSE](LICENSE) for details.

Third-party libraries and assets may be subject to their own respective licenses and terms.
