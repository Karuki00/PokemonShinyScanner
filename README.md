# Pokémon Shiny Scanner

An offline Windows companion app for inspecting Generation III Pokémon data and tracking shiny hunts in VisualBoyAdvance. The project began as a Python/PySide6 application and is migrating to a Tauri + Rust desktop app.

**Current release:** `0.2.0` — Gen III region migration milestone

**Repository:** [Karuki00/PokemonShinyScanner](https://github.com/Karuki00/PokemonShinyScanner)

## Supported games

| Region | Game | Generation | Status |
|---|---|---:|---|
| Hoenn | Pokémon Emerald | III | Supported |
| Hoenn | Pokémon Ruby | III | Supported and live validated |
| Hoenn | Pokémon Sapphire | III | Supported and live validated |
| Kanto | Pokémon FireRed | III | Supported and live validated |
| Kanto | Pokémon LeafGreen | III | Implemented with the shared US Kanto profile; validate against your ROM revision |
| — | Pokémon Gold / Silver / Crystal | II | Planned |

The first launch presents a region picker for Hoenn or Kanto. The active region can be changed later with **Change Region** in the sidebar. The scanner remembers the selected game locally, but all Pokémon data, sprites, and sound effects are bundled into the application so the Tauri build works offline.

Compatibility depends on the ROM language/revision and emulator memory layout. The current Gen III profiles target the US VisualBoyAdvance layouts. Non-US or modified ROMs may require a separate profile.

## Features

- Live party monitoring with species, nature, ability, shiny status, PID/OTID, checksum, and IVs.
- Wild encounter detection and encounter counters.
- Starter soft-reset tracking with `Ctrl + R` while VisualBoyAdvance is focused.
- Hoenn and Kanto region/game selection.
- Theme-aware Pokémon summary panels with bundled sprites and sound effects.
- Offline-first Tauri + Rust desktop runtime; no runtime download is required.
- Legacy Python/PySide6 implementation retained for reference during migration.

## Recommended release installation

Download the Windows installer from the GitHub Release assets. The installer contains the frontend, Pokémon sprites, shiny sprites, PC box backgrounds, and sound effects. No Python, Node.js, or internet connection is required for the released Tauri application.

Use a ROM legally obtained for your own use and start it in VisualBoyAdvance before starting the scanner.

## Development setup

### Tauri + Rust application

Requirements:

- Windows 10 or Windows 11
- Rust toolchain compatible with the `rust-version` in `tauri-app/src-tauri/Cargo.toml`
- Node.js and pnpm
- VisualBoyAdvance 1.8 Beta 3 or a compatible VBA build

```powershell
cd tauri-app
pnpm install
pnpm tauri dev
```

Checks used before release:

```powershell
pnpm build
cargo test --manifest-path src-tauri/Cargo.toml
```

The Tauri frontend imports assets from `tauri-app/src/assets`, so those files must remain tracked in Git. Do not commit `node_modules`, `dist`, `target`, or local logs.

### Legacy Python application

```powershell
python -m venv .venv
.venv\Scripts\activate
pip install -r requirements.txt
python main.py
```

The Python sprite/data generation scripts are retained for migration support. Normal Tauri users do not need to run them.

## Project structure

```text
PokemonShinyScanner/
├── tauri-app/
│   ├── src/                 React UI and bundled assets
│   └── src-tauri/           Rust scanner, decoders, and game profiles
├── core/                    Legacy Python memory/decoder modules
├── gui/                     Legacy PySide6 UI
├── assets/                  Legacy Python UI assets
├── tools/                   Migration/data-generation tools
├── LICENSE
└── README.md
```

## Roadmap

- [x] Gen III Pokémon decoding, IVs, natures, abilities, and shiny detection.
- [x] Tauri + Rust migration foundation.
- [x] Hoenn region picker and Emerald/Ruby/Sapphire profiles.
- [x] Kanto region picker and FireRed profile.
- [x] LeafGreen profile using the shared US Kanto layout.
- [ ] Validate LeafGreen with a live party and wild encounter.
- [ ] Add ROM revision/language profile detection.
- [ ] Add Generation I/II Game Boy Color scanner support.
- [ ] Add automated Windows release builds and checksums.

## Assets and attribution

Sprites are bundled for offline use and were sourced through the project’s sprite download tooling from the [PokeAPI sprites repository](https://github.com/PokeAPI/sprites). Pokémon names, characters, game data, and related assets remain the property of their respective owners. This is an unofficial fan-made tool and is not affiliated with Nintendo, The Pokémon Company, or Game Freak.

The application source code is distributed under the MIT License. See [LICENSE](LICENSE).
