# Pokémon Shiny Scanner — Tauri application

This directory contains the current desktop application. The React frontend and Rust backend are bundled by Tauri into an offline Windows application.

## Development

```powershell
pnpm install
pnpm tauri dev
```

## Verification

```powershell
pnpm build
cargo test --manifest-path src-tauri/Cargo.toml
```

The scanner currently supports US VisualBoyAdvance memory profiles for Pokémon Emerald, Ruby, Sapphire, FireRed, and LeafGreen. The profile implementation is in `src-tauri/src/games/gen3/profile.rs`.

Assets under `src/assets` are imported into the frontend bundle and must remain available for offline builds. Do not replace them with runtime GitHub downloads.
