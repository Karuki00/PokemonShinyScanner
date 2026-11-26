# Changelog

## Pokemon Shiny Scanner v0.3.4

### Sticky Footer Release

- Keeps the footer visible while scrolling so the app version and updater status remain accessible.

---

## Pokemon Shiny Scanner v0.3.3

### Runtime Cleanup Release

- Removes terminal debug logging from the desktop application.
- Fixes the sticky topbar CSS cascade and removes conflicting topbar overrides.

---

## Pokemon Shiny Scanner v0.3.2

### Updater Feedback Release

- Prevents the updater from remaining stuck in an infinite checking state.
- Shows a retryable failure status and the underlying error on hover.

---

## Pokemon Shiny Scanner v0.3.1

### Updater Improvements

- Periodically checks for updates in the background and when the app regains focus.
- Prompts users when a new release is available.
- Shows clear update-check status and recovers from stalled checks with a timeout.
- Adds a link to the full GitHub release notes in the update dialog.
- Adds EV-training helpers showing Pokémon and EV yields for each stat.
- Saves per-Pokémon EV goals in the app's SQLite database across restarts.
- Keeps the application title bar sticky while scrolling.

---

## Pokemon Shiny Scanner v0.3.0

### Automatic Hunting and PC Storage Release

This release adds automatic wild shiny hunting and live PC storage scanning,
along with party EV tracking and a redesigned interface.

### Features

- Automatic wild hunts for US Pokemon Emerald. The scanner moves in the overworld,
  recognizes battle transitions, runs from non-shiny encounters, and stops on a
  shiny encounter.
- Automatic wild capture flow with post-capture prompts handled and a stop when
  no Pokeballs remain.
- Starter shiny hunting with configurable starter selection and soft-reset
  tracking.
- Live read-only scanning of all 14 PC boxes, with Pokemon details and box
  wallpaper selection.
- EV gain tracking for Pokemon in the party.
- PC storage box wallpaper picker with 24 bundled wallpapers.
- Production builds omit the development-only battle diagnostic controls.
- Updated interface, navigation, responsive layouts, and text rendering.
- More reliable Generation III storage decoding and emulator input handling.

### Requirements

- Windows 10 / Windows 11.
- VisualBoyAdvance 1.8 Beta 3 or a compatible VBA build.
- A legally obtained Generation III Pokemon ROM.

### Compatibility

Targets the US VisualBoyAdvance memory layouts for Pokemon Emerald, Ruby,
Sapphire, FireRed, and LeafGreen. Automatic wild hunting is currently enabled
for the US Emerald profile.

---

## Pokémon Shiny Scanner v0.2.0

### Gen III Region Migration Release

The second public release of Pokémon Shiny Scanner introduces the Tauri + Rust
desktop application and expands scanning beyond Pokémon Emerald.

### Features

- Live Pokémon party monitoring.
- Starter Pokémon scanning and reset tracking.
- Wild encounter detection and analysis.
- IV calculation for party and wild Pokémon.
- Nature, species, and ability identification.
- Shiny detection using Generation III personality and trainer data.
- Encounter and reset counters with persistent hunting sessions.
- Hoenn Scanner support for Pokémon Emerald, Ruby, and Sapphire.
- Kanto Scanner support for Pokémon FireRed and LeafGreen.
- First-launch region selection and Change Region navigation.
- Themed interface panels, menu transitions, and sound effects.
- Offline bundled Pokémon sprites and application assets.
- Signed in-app update support for future releases.

### Requirements

- Windows 10 / Windows 11.
- VisualBoyAdvance 1.8 Beta 3 or a compatible VBA build.
- A legally obtained Generation III Pokémon ROM.

### Installation

1. Download the Windows installer from the release assets.
2. Run the `.msi` installer or the NSIS setup executable.
3. Start Pokémon Emerald, Ruby, Sapphire, FireRed, or LeafGreen in VisualBoyAdvance.
4. Open Pokémon Shiny Scanner.
5. Select the appropriate region and game, then start the scanner.

### Compatibility

Currently supports the US VisualBoyAdvance memory layouts for Pokémon Emerald,
Ruby, Sapphire, FireRed, and LeafGreen.

Non-US, modified, or different ROM revisions may require separate memory
profiles. Support for additional generations, including Game Boy Color titles,
is planned for future releases.

### Disclaimer

This is an unofficial fan-made project and is not affiliated with Nintendo, The
Pokémon Company, or Game Freak.
