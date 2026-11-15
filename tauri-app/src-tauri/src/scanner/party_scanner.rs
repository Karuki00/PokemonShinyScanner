use std::{
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

use serde::Serialize;
use tauri::{AppHandle, Emitter, State};

use windows_sys::Win32::{
    UI::Input::KeyboardAndMouse::{GetAsyncKeyState, VK_CONTROL},
    UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowThreadProcessId},
};

use crate::{
    decoders::gen3_decoder::{PokemonData, decode_pokemon},
    games::gen3::{
        pokemon_data::{ability_name, nature_name, species_info},
        profile::{
            Gen3Game, Gen3MemoryProfile, POKEMON_SIZE, is_trainer_battle, party_addresses,
            read_battle_type_flags, read_party, read_wild_candidate, verified_profile,
        },
    },
    memory::{
        memory_access::ProcessMemory,
        memory_scanner::resolve_memory_regions,
        process_reader::{find_module, find_process},
    },
};

const PROCESS_NAME: &str = "VisualBoyAdvance.exe";
const POLL_INTERVAL: Duration = Duration::from_millis(500);

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PokemonIvs {
    pub hp: u8,
    pub attack: u8,
    pub defense: u8,
    pub speed: u8,
    pub special_attack: u8,
    pub special_defense: u8,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ScannedPokemon {
    pub party_slot: u8,
    pub pid: u32,
    pub otid: u32,

    pub trainer_id: u16,
    pub secret_id: u16,

    pub species: u16,
    pub species_name: String,
    pub national_dex: u16,

    pub nature_id: u8,
    pub nature_name: String,

    pub ability_number: u8,
    pub ability_name: String,

    pub shiny: bool,
    pub checksum_valid: bool,
    pub ivs: PokemonIvs,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PartyUpdate {
    pub connected: bool,
    pub party_count: u8,
    pub pokemon: Vec<ScannedPokemon>,
}

impl PartyUpdate {
    fn disconnected() -> Self {
        Self {
            connected: false,
            party_count: 0,
            pokemon: Vec::new(),
        }
    }
}

/// Complete result from one memory scan.
struct ScanSnapshot {
    party: PartyUpdate,
    battle_type_flags: u32,
    opponent: Option<ScannedPokemon>,
}

impl ScanSnapshot {
    fn disconnected() -> Self {
        Self {
            party: PartyUpdate::disconnected(),
            battle_type_flags: 0,
            opponent: None,
        }
    }
}

/// Detect Ctrl + R only when VisualBoyAdvance is the foreground application.
fn start_keyboard_listener(
    app: AppHandle,
    running: Arc<AtomicBool>,
) -> Result<JoinHandle<()>, String> {
    thread::Builder::new()
        .name("emerald-keyboard-listener".to_string())
        .spawn(move || {
            let mut target_pid: Option<u32> = None;
            let mut last_pid_refresh = Instant::now();
            let mut previous_combo_down = false;

            while running.load(Ordering::SeqCst) {
                // Refresh the VBA process ID periodically.
                if target_pid.is_none() || last_pid_refresh.elapsed() >= Duration::from_secs(1) {
                    target_pid = find_process(PROCESS_NAME)
                        .ok()
                        .flatten()
                        .map(|process| process.pid);

                    last_pid_refresh = Instant::now();
                }

                // Identify the foreground window's process.
                let foreground_window = unsafe { GetForegroundWindow() };

                let mut foreground_pid = 0u32;

                if !foreground_window.is_null() {
                    unsafe {
                        GetWindowThreadProcessId(foreground_window, &mut foreground_pid);
                    }
                }

                let vba_is_foreground = target_pid == Some(foreground_pid);

                // Check the physical state of Ctrl and R.
                let ctrl_down =
                    unsafe { (GetAsyncKeyState(VK_CONTROL as i32) as u16 & 0x8000) != 0 };

                let r_down = unsafe { (GetAsyncKeyState('R' as i32) as u16 & 0x8000) != 0 };

                let combo_down = vba_is_foreground && ctrl_down && r_down;

                // Rising-edge detection: emit once per key press.
                if combo_down && !previous_combo_down {
                    let _ = app.emit("starter-reset", ());

                    log::info!("Starter reset detected: Ctrl + R");
                }

                previous_combo_down = combo_down;

                thread::sleep(Duration::from_millis(10));
            }
        })
        .map_err(|error| format!("Failed to start keyboard listener: {}", error))
}

/// Controls the background scanner thread.
pub struct ScannerState {
    running: Arc<AtomicBool>,
    worker: Mutex<Option<JoinHandle<()>>>,
    last_opponent: Arc<Mutex<Option<(u32, u32, u16)>>>,
}

impl Default for ScannerState {
    fn default() -> Self {
        Self {
            running: Arc::new(AtomicBool::new(false)),
            worker: Mutex::new(None),
            last_opponent: Arc::new(Mutex::new(None)),
        }
    }
}

impl ScannerState {
    pub fn start(&self, app: AppHandle, game: Gen3Game) -> Result<(), String> {
        let mut worker = self
            .worker
            .lock()
            .map_err(|_| "Scanner worker lock is poisoned.".to_string())?;

        if self.running.load(Ordering::SeqCst) {
            log::debug!("Scanner is already running. Ignoring duplicate start request.");
            return Ok(());
        }

        self.running.store(true, Ordering::SeqCst);
        let profile = verified_profile(game)?;

        let running = Arc::clone(&self.running);
        let last_opponent_state = Arc::clone(&self.last_opponent);
        let previous_opponent = *last_opponent_state
            .lock()
            .map_err(|_| "Scanner opponent state lock is poisoned.".to_string())?;

        let keyboard_running = Arc::clone(&running);
        let keyboard_app = app.clone();

        let keyboard_handle = start_keyboard_listener(keyboard_app, keyboard_running)?;

        let handle = thread::Builder::new()
            .name("emerald-party-scanner".to_string())
            .spawn(move || {
                let mut last_update: Option<PartyUpdate> = None;
                let mut last_error: Option<String> = None;

                // Remember the last wild Pokémon to avoid duplicate encounter events.
                // Preserve the current opponent across Stop/Start so restarting
                // during the same battle does not count the same Pokémon again.
                let mut last_emitted_opponent = previous_opponent;

                while running.load(Ordering::SeqCst) {
                    match scan_once(profile) {
                        Ok(snapshot) => {
                            if !running.load(Ordering::SeqCst) {
                                break;
                            }
                            last_error = None;

                            // Emit party changes.
                            if last_update.as_ref() != Some(&snapshot.party) {
                                log::info!(
                                    "Party scan [{}]: connected={}, memory count={}, decoded slots={}",
                                    profile.game.display_name(),
                                    snapshot.party.connected,
                                    snapshot.party.party_count,
                                    snapshot.party.pokemon.len()
                                );

                                for pokemon in &snapshot.party.pokemon {
                                    log::info!(
                                        "Party slot {}: {} (PID: {:08X}, checksum: {}, shiny: {})",
                                        pokemon.party_slot,
                                        pokemon.species_name,
                                        pokemon.pid,
                                        pokemon.checksum_valid,
                                        pokemon.shiny
                                    );
                                }

                                let _ = app.emit("party-updated", &snapshot.party);
                                last_update = Some(snapshot.party.clone());
                            }

                            // Reset identity tracking if VBA disconnects.
                            if !snapshot.party.connected {
                                last_emitted_opponent = None;
                                if let Ok(mut state) = last_opponent_state.lock() {
                                    *state = None;
                                }
                            } else {
                                let in_battle = snapshot.battle_type_flags != 0
                                    && !is_trainer_battle(snapshot.battle_type_flags, profile);

                                if in_battle {
                                    if let Some(opponent) = snapshot.opponent.as_ref() {
                                        // Identify Pokémon using PID, OTID, and species.
                                        let opponent_identity =
                                            (opponent.pid, opponent.otid, opponent.species);

                                        // Emit only when a different opponent is detected.
                                        if last_emitted_opponent != Some(opponent_identity) {
                                            let _ = app.emit("wild-encounter", opponent);

                                            last_emitted_opponent = Some(opponent_identity);
                                            if let Ok(mut state) = last_opponent_state.lock() {
                                                *state = last_emitted_opponent;
                                            }

                                            log::info!(
                                                "New wild encounter: {} (PID: {:08X})",
                                                opponent.species_name,
                                                opponent.pid
                                            );
                                        }
                                    }
                                } else {
                                    last_emitted_opponent = None;
                                    if let Ok(mut state) = last_opponent_state.lock() {
                                        *state = None;
                                    }
                                }
                            }
                        }

                        Err(error) => {
                            if last_error.as_ref() != Some(&error) {
                                let _ = app.emit("scanner-error", &error);
                                last_error = Some(error);
                            }
                        }
                    }

                    thread::sleep(POLL_INTERVAL);
                }

                // Stop and join the keyboard listener.
                let _ = keyboard_handle.join();
                let _ = app.emit("scanner-stopped", ());
            })
            .map_err(|error| {
                self.running.store(false, Ordering::SeqCst);

                format!("Failed to start scanner thread: {}", error)
            })?;

        *worker = Some(handle);

        Ok(())
    }

    pub fn stop(&self) -> Result<(), String> {
        self.running.store(false, Ordering::SeqCst);

        let handle = self
            .worker
            .lock()
            .map_err(|_| "Scanner worker lock is poisoned.".to_string())?
            .take();

        if let Some(handle) = handle {
            handle
                .join()
                .map_err(|_| "Scanner thread terminated unexpectedly.".to_string())?;
        }

        Ok(())
    }
}

/// Convert decoded Gen III Pokémon data into the frontend structure.
fn to_scanned_pokemon(decoded: PokemonData, party_slot: u8) -> ScannedPokemon {
    let species = decoded.species;
    let species_data = species_info(species);

    ScannedPokemon {
        party_slot,
        pid: decoded.pid,
        otid: decoded.otid,

        trainer_id: decoded.trainer_id,
        secret_id: decoded.secret_id,

        species,
        species_name: species_data
            .map(|info| info.name.to_string())
            .unwrap_or_else(|| format!("Unknown Species (#{})", species)),
        national_dex: species_data.map(|info| info.national_dex).unwrap_or(0),

        nature_id: decoded.nature_id,
        nature_name: nature_name(decoded.nature_id).to_string(),

        ability_number: decoded.ability_number,
        ability_name: ability_name(species, decoded.ability_number).to_string(),

        shiny: decoded.shiny,
        checksum_valid: decoded.checksum_valid,

        ivs: PokemonIvs {
            hp: decoded.ivs.hp,
            attack: decoded.ivs.attack,
            defense: decoded.ivs.defense,
            speed: decoded.ivs.speed,
            special_attack: decoded.ivs.special_attack,
            special_defense: decoded.ivs.special_defense,
        },
    }
}

/// Perform one complete memory scan.
fn scan_once(profile: &Gen3MemoryProfile) -> Result<ScanSnapshot, String> {
    let process = match find_process(PROCESS_NAME)? {
        Some(process) => process,
        None => return Ok(ScanSnapshot::disconnected()),
    };

    let module = find_module(process.pid, PROCESS_NAME)?
        .ok_or_else(|| "VBA executable module was not found.".to_string())?;

    let memory = ProcessMemory::open(process.pid)?;

    let regions = resolve_memory_regions(&memory, &module)?;

    // Read the player's party.
    let party = read_party(&memory, &regions, profile)?;
    let (party_count_address, party_data_address) = party_addresses(&regions, profile)?;

    if matches!(profile.game, Gen3Game::FireRed | Gen3Game::LeafGreen) {
        log::debug!(
            "Party memory [{}]: count @ 0x{:08X} = {}, data @ 0x{:08X} ({} bytes)",
            profile.game.display_name(),
            party_count_address,
            party.count,
            party_data_address,
            party.pokemon_data.len()
        );
    }

    let mut pokemon = Vec::with_capacity(party.count as usize);

    for (slot, bytes) in party.pokemon_data.chunks_exact(POKEMON_SIZE).enumerate() {
        match decode_pokemon(bytes) {
            Ok(decoded) => {
                if matches!(profile.game, Gen3Game::FireRed | Gen3Game::LeafGreen) {
                    log::debug!(
                        "Party slot {} decoded: species={}, PID={:08X}, checksum_valid={}, shiny={}",
                        slot + 1,
                        decoded.species,
                        decoded.pid,
                        decoded.checksum_valid,
                        decoded.shiny
                    );
                }
                pokemon.push(to_scanned_pokemon(decoded, (slot + 1) as u8));
            }
            Err(error) => log::warn!("Skipping invalid party slot {}: {}", slot + 1, error),
        }
    }

    let party_update = PartyUpdate {
        connected: true,
        party_count: party.count,
        pokemon,
    };

    // Read battle state.
    let battle_type_flags = read_battle_type_flags(&memory, &regions, profile)?;

    // Only inspect opponent memory while a battle is active.
    let opponent = if battle_type_flags != 0 && !is_trainer_battle(battle_type_flags, profile) {
        read_wild_candidate(&memory, &regions, profile)?
            .map(|pokemon| to_scanned_pokemon(pokemon, 1))
    } else {
        None
    };

    Ok(ScanSnapshot {
        party: party_update,
        battle_type_flags,
        opponent,
    })
}

/// Tauri command: start continuous party scanning.
#[tauri::command]
pub fn start_scanner(
    app: AppHandle,
    scanner: State<'_, ScannerState>,
    game: String,
) -> Result<(), String> {
    let game = Gen3Game::from_id(&game)?;
    scanner.start(app, game)
}

/// Tauri command: stop continuous party scanning.
#[tauri::command]
pub fn stop_scanner(scanner: State<'_, ScannerState>) -> Result<(), String> {
    scanner.stop()
}
