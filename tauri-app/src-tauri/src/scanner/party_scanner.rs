use std::{
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    thread::{self, JoinHandle},
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use enigo::{Direction as EnigoDirection, Enigo, Key as EnigoKey, Keyboard, Settings};
use serde::Serialize;
use tauri::{AppHandle, Emitter, State};

use windows_sys::Win32::{
    UI::Input::KeyboardAndMouse::{
        GetAsyncKeyState, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP, VK_CONTROL,
        VK_DOWN, VK_LEFT, VK_RIGHT, VK_UP,
    },
    UI::WindowsAndMessaging::{
        EnumWindows, GetForegroundWindow, GetWindowThreadProcessId, IsIconic, IsWindowVisible,
        SW_RESTORE, SetForegroundWindow, ShowWindow,
    },
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
const POLL_INTERVAL: Duration = Duration::from_millis(100);
const VBA_A_KEY: u16 = b'Z' as u16;
const VBA_B_KEY: u16 = b'X' as u16;
const EMERALD_GMAIN_OFFSET_IWRAM: usize = 0x22C0;
const EMERALD_TASKS_OFFSET_IWRAM: usize = 0x5E00;
const EMERALD_MAIN_CALLBACK2_OFFSET: usize = 0x04;
const EMERALD_MAIN_HELD_KEYS_RAW_OFFSET: usize = 0x28;
const EMERALD_MAIN_NEW_KEYS_RAW_OFFSET: usize = 0x2A;
const EMERALD_BATTLE_CONTROLLER_FUNCS_OFFSET_IWRAM: usize = 0x5D60;
const EMERALD_ACTIVE_BATTLER_OFFSET_EWRAM: usize = 0x24064;
const EMERALD_BATTLE_CONTROLLER_FLAGS_OFFSET_EWRAM: usize = 0x24068;
const EMERALD_ACTION_SELECTION_CURSOR_OFFSET_EWRAM: usize = 0x244AC;
const EMERALD_BATTLE_COMMUNICATION_OFFSET_EWRAM: usize = 0x24332;
const EMERALD_BATTLE_OUTCOME_OFFSET_EWRAM: usize = 0x2433A;
const EMERALD_BATTLE_SCRIPT_INSTR_OFFSET_EWRAM: usize = 0x24214;
const EMERALD_BATTLE_RESULTS_OFFSET_IWRAM: usize = 0x5D10;
const EMERALD_CAUGHT_SPECIES_OFFSET_BATTLE_RESULTS: usize = 0x28;
const EMERALD_TASK_ACTIVE_OFFSET: usize = 4;
const EMERALD_BATTLE_MAIN_CALLBACK2: u32 = 0x0803_8420;
const EMERALD_OVERWORLD_CALLBACK2: u32 = 0x0808_5E5C;
const EMERALD_NAMING_SCREEN_CALLBACK2: u32 = 0x080E_4F58;
const EMERALD_DEX_TASK_DISPLAY: u32 = 0x080B_FE39;
const EMERALD_DEX_TASK_INPUT: u32 = 0x080C_0089;
const EMERALD_DEX_TASK_EXIT: u32 = 0x080C_012D;
const EMERALD_BATTLE_BUFFER_A_OFFSET_EWRAM: usize = 0x23064;
const EMERALD_BATTLE_BUFFER_SIZE: usize = 0x200;
const EMERALD_GMAIN_IN_BATTLE_OFFSET: usize = 0x439;
// Emerald stores relocated SaveBlock pointers in IWRAM. Resolve these for
// every scan because the save blocks may move when a save is loaded/reset.
const EMERALD_SAVEBLOCK1_PTR_OFFSET_IWRAM: usize = 0x5D8C;
const EMERALD_SAVEBLOCK2_PTR_OFFSET_IWRAM: usize = 0x5D90;
const EMERALD_POKEMON_STORAGE_PTR_OFFSET_IWRAM: usize = 0x5D94;
const FRLG_POKEMON_STORAGE_PTR_OFFSET_IWRAM: usize = 0x5010;
const RS_POKEMON_STORAGE_OFFSET_EWRAM: usize = 0x300A0;
const POKEMON_STORAGE_BOX_COUNT: usize = 14;
const POKEMON_STORAGE_BOX_CAPACITY: usize = 30;
const BOX_POKEMON_SIZE: usize = 80;
// PokemonStorage aligns the BoxPokemon array to four bytes after currentBox.
const POKEMON_STORAGE_BOXES_OFFSET: usize = 4;
const GBA_EWRAM_START: u32 = 0x0200_0000;
const EMERALD_SAVE_BLOCK2_ENCRYPTION_KEY_OFFSET: usize = 0xAC;
const EMERALD_BAG_POKEBALLS_OFFSET: usize = 0x650;
const EMERALD_BAG_POKEBALLS_SLOT_COUNT: usize = 16;
const EMERALD_LAST_BALL_ITEM_ID: u16 = 12;
const EMERALD_BAG_POSITION_SIZE: usize = 28;
const EMERALD_BAG_POCKET_COUNT: usize = 5;
const EMERALD_PLAYER_ACTION_INPUT_HANDLER: u32 = 0x0805_7588;
const EMERALD_PLAYER_WAIT_TEXT_HANDLER: u32 = 0x0805_97B4;
const EMERALD_INTRO_MESSAGE_STRING_ID: u16 = 0;
const EMERALD_GOT_AWAY_SAFELY_STRING_ID: u16 = 223;
const EMERALD_GOTCHA_CAUGHT_MESSAGE_STRING_ID: u16 = 267;
const EMERALD_STARTER_SCREEN_CALLBACK: u32 = 0x0813_41E0;
const EMERALD_STARTER_INPUT_TASK: u32 = 0x0813_425C;
const EMERALD_STARTER_CONFIRM_TASK: u32 = 0x0813_4400;
const EMERALD_STARTER_INIT_TASK: u32 = 0x0813_41FC;
const EMERALD_STARTER_WAIT_TASK: u32 = 0x0813_4340;
const EMERALD_STARTER_ASK_TASK: u32 = 0x0813_4390;
const EMERALD_STARTER_DECLINE_TASK: u32 = 0x0813_44AC;
const EMERALD_STARTER_MOVE_TASK: u32 = 0x0813_4640;
const EMERALD_STARTER_CREATE_LABEL_TASK: u32 = 0x0813_4668;
const EMERALD_TASK_SIZE: usize = 0x28;
const EMERALD_TASK_COUNT: usize = 16;
static EMERALD_DEX_DIAGNOSTIC_LOGGED: AtomicBool = AtomicBool::new(false);
static EMERALD_B_INPUT_DIAGNOSTIC_LOGGED: AtomicBool = AtomicBool::new(false);
static EMERALD_CAPTURE_TRACE_LAST_STAGE: std::sync::atomic::AtomicU8 =
    std::sync::atomic::AtomicU8::new(0);
static EMERALD_CAPTURE_TRACE_COUNT: std::sync::atomic::AtomicU8 =
    std::sync::atomic::AtomicU8::new(0);
static EMERALD_CAPTURE_CONFIRMED_LATCHED: AtomicBool = AtomicBool::new(false);

fn arrow_key(key: u16) -> Option<EnigoKey> {
    match key {
        VK_LEFT => Some(EnigoKey::LeftArrow),
        VK_RIGHT => Some(EnigoKey::RightArrow),
        VK_UP => Some(EnigoKey::UpArrow),
        VK_DOWN => Some(EnigoKey::DownArrow),
        _ => None,
    }
}

struct WindowSearch {
    pid: u32,
    window: windows_sys::Win32::Foundation::HWND,
}

unsafe extern "system" fn find_vba_window(
    window: windows_sys::Win32::Foundation::HWND,
    data: isize,
) -> i32 {
    let search = unsafe { &mut *(data as *mut WindowSearch) };
    let mut window_pid = 0u32;
    unsafe { GetWindowThreadProcessId(window, &mut window_pid) };
    if window_pid == search.pid && unsafe { IsWindowVisible(window) } != 0 {
        search.window = window;
        0
    } else {
        1
    }
}

fn send_vba_key(key: u16, control: bool, game: &str) -> Result<bool, String> {
    send_vba_key_with_turn_tap(key, control, game, false)
}

fn send_vba_key_with_turn_tap(
    key: u16,
    control: bool,
    game: &str,
    turn_tap: bool,
) -> Result<bool, String> {
    let process = find_process(PROCESS_NAME)?
        .ok_or_else(|| "VisualBoyAdvance.exe is not running.".to_string())?;
    let mut search = WindowSearch {
        pid: process.pid,
        window: std::ptr::null_mut(),
    };
    unsafe {
        EnumWindows(
            Some(find_vba_window),
            &mut search as *mut WindowSearch as isize,
        )
    };
    let window = search.window;
    if window.is_null() {
        return Err("Could not find a visible VisualBoyAdvance window.".to_string());
    }

    let ack_reader = if game.eq_ignore_ascii_case("emerald") && !control {
        if let Some(process) = find_process(PROCESS_NAME)? {
            if let Some(module) = find_module(process.pid, PROCESS_NAME)? {
                let memory = ProcessMemory::open(process.pid)?;
                let regions = resolve_memory_regions(&memory, &module)?;
                let key_mask = match key {
                    VBA_A_KEY => 0x0001u16,
                    VBA_B_KEY => 0x0002u16,
                    VK_LEFT => 0x0020u16,
                    VK_RIGHT => 0x0010u16,
                    VK_UP => 0x0040u16,
                    VK_DOWN => 0x0080u16,
                    _ => 0,
                };
                Some((
                    memory,
                    regions.iwram_address
                        + EMERALD_GMAIN_OFFSET_IWRAM
                        + EMERALD_MAIN_HELD_KEYS_RAW_OFFSET,
                    regions.iwram_address
                        + EMERALD_GMAIN_OFFSET_IWRAM
                        + EMERALD_MAIN_NEW_KEYS_RAW_OFFSET,
                    key_mask,
                ))
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    };
    let arrow = arrow_key(key);
    let mut enigo = if arrow.is_some() {
        Some(
            Enigo::new(&Settings::default())
                .map_err(|error| format!("Could not initialize Enigo keyboard input: {error}"))?,
        )
    } else {
        None
    };

    let mut game_acknowledged = false;
    unsafe {
        if GetForegroundWindow() != window {
            // Restore only a minimized VBA window. SW_RESTORE on an already
            // maximized window can return it to the normal size from its INI.
            if IsIconic(window) != 0 {
                ShowWindow(window, SW_RESTORE);
            }
            SetForegroundWindow(window);
            thread::sleep(Duration::from_millis(100));
        }
        if GetForegroundWindow() != window {
            return Err(
                "VisualBoyAdvance did not become the foreground window; no key was sent."
                    .to_string(),
            );
        }

        let control_down = INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VK_CONTROL as u16,
                    wScan: 0,
                    dwFlags: 0,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        };
        let key_down = INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: key,
                    wScan: 0,
                    dwFlags: 0,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        };
        let key_up = INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: key,
                    wScan: 0,
                    dwFlags: KEYEVENTF_KEYUP,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        };
        let send_input = windows_sys::Win32::UI::Input::KeyboardAndMouse::SendInput;
        if control && send_input(1, &control_down, size_of::<INPUT>() as i32) != 1 {
            return Err("Windows did not accept Ctrl for the VBA soft reset.".to_string());
        }
        if let (Some(enigo), Some(arrow)) = (enigo.as_mut(), arrow) {
            enigo
                .key(arrow, EnigoDirection::Press)
                .map_err(|error| format!("Enigo could not press the VBA arrow key: {error}"))?;
        } else if send_input(1, &key_down, size_of::<INPUT>() as i32) != 1 {
            return Err(format!("Windows did not accept the VBA key press ({key})."));
        }
        if let Some((memory, held_keys_address, new_keys_address, key_mask)) = ack_reader.as_ref() {
            let deadline = Instant::now() + Duration::from_millis(300);
            while Instant::now() < deadline {
                let held_bytes = memory.read(*held_keys_address, 2)?;
                let new_bytes = memory.read(*new_keys_address, 2)?;
                let held_keys = u16::from_le_bytes([held_bytes[0], held_bytes[1]]);
                let new_keys = u16::from_le_bytes([new_bytes[0], new_bytes[1]]);
                if held_keys & key_mask != 0 || new_keys & key_mask != 0 {
                    game_acknowledged = true;
                    break;
                }
                thread::sleep(Duration::from_millis(2));
            }
        } else {
            thread::sleep(Duration::from_millis(if turn_tap {
                8
            } else if key == VBA_A_KEY {
                100
            } else {
                80
            }));
        }
        thread::sleep(Duration::from_millis(if turn_tap {
            0
        } else if key == VBA_B_KEY {
            120
        } else if key == VBA_A_KEY {
            25
        } else {
            35
        }));
        if let (Some(enigo), Some(arrow)) = (enigo.as_mut(), arrow) {
            enigo
                .key(arrow, EnigoDirection::Release)
                .map_err(|error| format!("Enigo could not release the VBA arrow key: {error}"))?;
        } else if send_input(1, &key_up, size_of::<INPUT>() as i32) != 1 {
            return Err(format!(
                "Windows did not accept the VBA key release ({key})."
            ));
        }
        if control {
            let control_up = INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: VK_CONTROL as u16,
                        wScan: 0,
                        dwFlags: KEYEVENTF_KEYUP,
                        time: 0,
                        dwExtraInfo: 0,
                    },
                },
            };
            if send_input(1, &control_up, size_of::<INPUT>() as i32) != 1 {
                return Err(
                    "Windows did not accept Ctrl release after the VBA soft reset.".to_string(),
                );
            }
        }
    }

    Ok(game_acknowledged)
}

/// Send one key to VisualBoyAdvance. Key names use VBA's default bindings.
#[tauri::command]
pub fn send_emulator_key(
    key: String,
    game: String,
    allow_starter_selection_a: bool,
) -> Result<String, String> {
    let (virtual_key, control, label) = match key.as_str() {
        "a" => (VBA_A_KEY, false, "A"),
        "b" => (VBA_B_KEY, false, "B"),
        "left" => (VK_LEFT, false, "Left arrow"),
        "right" => (VK_RIGHT, false, "Right arrow"),
        "up" => (VK_UP, false, "Up arrow"),
        "down" => (VK_DOWN, false, "Down arrow"),
        "soft_reset" => (b'R' as u16, true, "Ctrl+R soft reset"),
        _ => return Err("Unsupported VBA diagnostic key.".to_string()),
    };
    if key == "a"
        && game.eq_ignore_ascii_case("emerald")
        && !allow_starter_selection_a
        && emerald_starter_screen_active()?
    {
        return Ok("Skipped A: Emerald is already in the starter chooser.".to_string());
    }
    let acknowledged = send_vba_key(virtual_key, control, &game)?;
    if key == "b"
        && game.eq_ignore_ascii_case("emerald")
        && !EMERALD_B_INPUT_DIAGNOSTIC_LOGGED.swap(true, Ordering::Relaxed)
    {
        log::warn!(
            "Wild capture diagnostic: sent VBA B binding X; gMain key-memory acknowledgment={acknowledged}."
        );
    }
    if game.eq_ignore_ascii_case("emerald") && !control {
        Ok(format!(
            "Sent {label}; {}",
            if acknowledged {
                "Emerald acknowledged the input in gMain held/new key memory."
            } else {
                "no Emerald input acknowledgment observed; retrying may be needed."
            }
        ))
    } else {
        Ok(format!("Sent {label} to VisualBoyAdvance."))
    }
}

#[tauri::command]
pub fn send_emulator_turn(key: String, game: String) -> Result<String, String> {
    let key = key.to_ascii_lowercase();
    let virtual_key = match key.as_str() {
        "left" => VK_LEFT,
        "right" => VK_RIGHT,
        "up" => VK_UP,
        "down" => VK_DOWN,
        _ => return Err("Overworld turn input only accepts a direction arrow.".to_string()),
    };
    let acknowledged = send_vba_key_with_turn_tap(virtual_key, false, &game, true)?;
    if acknowledged {
        Ok("One-tick turn input acknowledged by Pokémon Emerald.".to_string())
    } else {
        Ok("One-tick turn input sent, but Emerald did not acknowledge it.".to_string())
    }
}

/// Backward-compatible one-press diagnostic command.
#[tauri::command]
pub fn press_emulator_a() -> Result<String, String> {
    send_vba_key(VBA_A_KEY, false, "")?;
    Ok("Sent one A-button press to VisualBoyAdvance using its default Z key binding.".to_string())
}

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
pub struct PokemonEvs {
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

    pub shiny_value: u16,
    pub ability_number: u8,
    pub ability_name: String,

    pub shiny: bool,
    pub checksum_valid: bool,
    pub ivs: PokemonIvs,
    pub evs: PokemonEvs,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PcBoxSnapshot {
    pub index: u8,
    pub occupied_count: u8,
    pub invalid_slots: Vec<u8>,
    pub slots: Vec<Option<ScannedPokemon>>,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PcStorageUpdate {
    pub connected: bool,
    pub supported: bool,
    pub available: bool,
    pub current_box: Option<u8>,
    pub total_pokemon: u16,
    pub boxes: Vec<PcBoxSnapshot>,
    pub message: String,
}

impl PcStorageUpdate {
    fn disconnected() -> Self {
        Self {
            connected: false,
            supported: true,
            available: false,
            current_box: None,
            total_pokemon: 0,
            boxes: Vec::new(),
            message: "Start the scanner to read PC storage.".to_string(),
        }
    }

    fn unavailable(connected: bool, supported: bool, message: String) -> Self {
        Self {
            connected,
            supported,
            available: false,
            current_box: None,
            total_pokemon: 0,
            boxes: Vec::new(),
            message,
        }
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PartyUpdate {
    pub connected: bool,
    pub party_count: u8,
    pub pokemon: Vec<ScannedPokemon>,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GameStateUpdate {
    pub connected: bool,
    pub state: String,
    pub detail: String,
    pub party_count: u8,
    pub battle_type_flags: u32,
    pub emerald_callback2: u32,
    pub starter_selection_ready: bool,
    pub starter_confirmation_ready: bool,
    pub emerald_starter_selection: Option<u8>,
    pub battle_action_menu_ready: bool,
    pub battle_action_menu_cursor: Option<u8>,
    pub wild_appeared_text_ready: bool,
    pub wild_escaped_text_ready: bool,
    pub battle_text_ready: bool,
    pub emerald_in_battle: Option<bool>,
    pub emerald_pokeball_count: Option<u16>,
    pub emerald_bag_menu_ready: bool,
    pub emerald_bag_pocket: Option<u8>,
    pub emerald_bag_cursor: Option<u16>,
    pub emerald_nickname_prompt_ready: bool,
    pub emerald_nickname_prompt_cursor: Option<u8>,
    pub emerald_dex_registration_ready: bool,
    pub emerald_catch_confirmed: bool,
    pub emerald_overworld_ready: bool,
    pub emerald_battle_outcome: Option<u8>,
    pub emerald_battle_communication: Option<Vec<u8>>,
    pub emerald_battle_text_string_id: Option<u16>,
    pub emerald_caught_species: Option<u16>,
    pub emerald_wild_species: Option<u16>,
    pub emerald_dex_task_id: Option<u8>,
    pub emerald_dex_task_function: Option<u32>,
    pub emerald_dex_task_active: Option<bool>,
    pub emerald_dex_page_input_ready: bool,
    pub emerald_naming_screen_active: bool,
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
    pc_storage: PcStorageUpdate,
    pc_storage_address: Option<usize>,
    pc_storage_diagnostic: Option<String>,
    battle_type_flags: u32,
    opponent: Option<ScannedPokemon>,
    game_state: GameStateUpdate,
    battle_ui_diagnostic: Option<BattleUiDiagnosticSample>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BattleUiDiagnosticSample {
    pub timestamp_unix_ms: u128,
    pub game_profile: String,
    pub address_basis: String,
    pub ewram_process_base: Option<String>,
    pub iwram_process_base: Option<String>,
    pub battle_type_flags_address: Option<String>,
    pub battle_type_flags: Option<u32>,
    pub active_battler_address: Option<String>,
    pub active_battler: Option<u8>,
    pub controller_function_address: Option<String>,
    pub controller_function: Option<u32>,
    pub battler_controller_functions: Vec<u32>,
    pub controller_exec_flags_address: Option<String>,
    pub controller_exec_flags: Option<u32>,
    pub action_cursor_address: Option<String>,
    pub action_cursor: Option<u8>,
    pub current_menu_candidate: bool,
    pub candidate_state: String,
    pub emerald_in_battle_address: Option<String>,
    pub emerald_in_battle: Option<bool>,
    pub battle_buffer_command_address: Option<String>,
    pub battle_buffer_command: Option<u8>,
    pub battle_buffer_string_id: Option<u16>,
    pub battler_buffer_commands: Vec<u8>,
    pub battler_buffer_string_ids: Vec<u16>,
    pub wild_appeared_text_candidate: bool,
    pub wild_escaped_text_candidate: bool,
    pub emerald_callback2: Option<u32>,
    pub opponent_species: Option<u16>,
    pub opponent_pid: Option<u32>,
    pub opponent_shiny: Option<bool>,
    pub rom_fingerprint_verified: bool,
}

impl BattleUiDiagnosticSample {
    fn transition_key(&self) -> String {
        [
            format!("{:?}", self.battle_type_flags),
            format!("{:?}", self.emerald_callback2),
            format!("{:?}", self.emerald_in_battle),
            format!("{:?}", self.active_battler),
            format!("{:?}", self.controller_function),
            format!("{:?}", self.battler_controller_functions),
            format!("{:?}", self.controller_exec_flags),
            format!("{:?}", self.action_cursor),
            format!("{}", self.current_menu_candidate),
            format!("{:?}", self.battle_buffer_command),
            format!("{:?}", self.battle_buffer_string_id),
            format!("{:?}", self.battler_buffer_commands),
            format!("{:?}", self.battler_buffer_string_ids),
            format!("{}", self.wild_appeared_text_candidate),
            format!("{}", self.wild_escaped_text_candidate),
            format!("{:?}", self.opponent_species),
            format!("{:?}", self.opponent_pid),
            format!("{:?}", self.opponent_shiny),
        ]
        .join("|")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EmeraldBattleUiState {
    Overworld,
    WildIntroduction,
    WildEscapeText,
    CommandMenuReady,
    BattleTransition,
}

fn classify_emerald_battle_ui(
    in_battle: bool,
    menu_ready: bool,
    wild_appeared_text_candidate: bool,
    wild_escaped_text_candidate: bool,
) -> EmeraldBattleUiState {
    if !in_battle {
        EmeraldBattleUiState::Overworld
    } else if menu_ready {
        EmeraldBattleUiState::CommandMenuReady
    } else if wild_escaped_text_candidate {
        EmeraldBattleUiState::WildEscapeText
    } else if wild_appeared_text_candidate {
        EmeraldBattleUiState::WildIntroduction
    } else {
        EmeraldBattleUiState::BattleTransition
    }
}

#[derive(Default)]
struct BattleMenuDebouncer {
    ready_samples: u8,
    stable_ready: bool,
}

impl BattleMenuDebouncer {
    fn update(&mut self, raw_ready: bool) -> bool {
        if raw_ready {
            self.ready_samples = self.ready_samples.saturating_add(1);
            if self.ready_samples >= 2 {
                self.stable_ready = true;
            }
        } else {
            self.ready_samples = 0;
            self.stable_ready = false;
        }
        self.stable_ready
    }
}

impl ScanSnapshot {
    fn disconnected() -> Self {
        Self {
            party: PartyUpdate::disconnected(),
            pc_storage: PcStorageUpdate::disconnected(),
            pc_storage_address: None,
            pc_storage_diagnostic: None,
            battle_type_flags: 0,
            opponent: None,
            game_state: GameStateUpdate {
                connected: false,
                state: "disconnected".to_string(),
                detail: "VisualBoyAdvance is not detected.".to_string(),
                party_count: 0,
                battle_type_flags: 0,
                emerald_callback2: 0,
                starter_selection_ready: false,
                starter_confirmation_ready: false,
                emerald_starter_selection: None,
                battle_action_menu_ready: false,
                battle_action_menu_cursor: None,
                wild_appeared_text_ready: false,
                wild_escaped_text_ready: false,
                battle_text_ready: false,
                emerald_in_battle: None,
                emerald_pokeball_count: None,
                emerald_bag_menu_ready: false,
                emerald_bag_pocket: None,
                emerald_bag_cursor: None,
                emerald_nickname_prompt_ready: false,
                emerald_nickname_prompt_cursor: None,
                emerald_dex_registration_ready: false,
                emerald_catch_confirmed: false,
                emerald_overworld_ready: false,
                emerald_battle_outcome: None,
                emerald_battle_communication: None,
                emerald_battle_text_string_id: None,
                emerald_caught_species: None,
                emerald_wild_species: None,
                emerald_dex_task_id: None,
                emerald_dex_task_function: None,
                emerald_dex_task_active: None,
                emerald_dex_page_input_ready: false,
                emerald_naming_screen_active: false,
            },
            battle_ui_diagnostic: None,
        }
    }
}

fn classify_game_state(
    party_count: u8,
    first_party_national_dex: Option<u16>,
    battle_type_flags: u32,
    has_wild_opponent: bool,
    starter_selection_ready: bool,
    profile: &Gen3MemoryProfile,
) -> (&'static str, &'static str) {
    if first_party_national_dex == Some(252) {
        ("treecko_detected", "Treecko is present in party slot 1.")
    } else if starter_selection_ready {
        (
            "starter_selection",
            "Emerald starter chooser is active and accepts input.",
        )
    } else if battle_type_flags != 0 && is_trainer_battle(battle_type_flags, profile) {
        ("trainer_battle", "Battle flags indicate a trainer battle.")
    } else if battle_type_flags != 0 && has_wild_opponent {
        (
            "wild_battle",
            "Battle flags and opponent data indicate a wild battle.",
        )
    } else if party_count > 0 {
        (
            "party_present",
            "Party data is readable; the intro/menu screen is not identified yet.",
        )
    } else {
        (
            "unknown",
            "VBA is connected; no verified intro, dialogue, or starter-menu signal is available yet.",
        )
    }
}

#[derive(Debug, Clone, Default)]
struct EmeraldRuntimeState {
    callback2: u32,
    starter_selection_ready: bool,
    starter_confirmation_ready: bool,
    starter_selection: Option<u8>,
    battle_action_menu_ready: bool,
    battle_action_menu_cursor: Option<u8>,
    active_battler: Option<u8>,
    controller_function: Option<u32>,
    controller_exec_flags: Option<u32>,
    action_cursor: Option<u8>,
    in_battle: bool,
    battle_buffer_address: Option<usize>,
    battle_buffer_command: Option<u8>,
    battle_buffer_string_id: Option<u16>,
    wild_escaped_text_ready: bool,
    battle_text_ready: bool,
    battler_controller_functions: Vec<u32>,
    battler_buffer_commands: Vec<u8>,
    battler_buffer_string_ids: Vec<u16>,
}

fn read_u32_at(memory: &ProcessMemory, address: usize) -> Result<u32, String> {
    let bytes = memory.read(address, 4)?;
    Ok(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
}

fn resolve_pc_storage_address(
    memory: &ProcessMemory,
    regions: &crate::memory::memory_scanner::MemoryRegions,
    game: Gen3Game,
) -> Result<usize, String> {
    let storage_offset = match game {
        Gen3Game::Ruby | Gen3Game::Sapphire => RS_POKEMON_STORAGE_OFFSET_EWRAM,
        Gen3Game::Emerald => {
            let pointer = read_u32_at(
                memory,
                regions.iwram_address + EMERALD_POKEMON_STORAGE_PTR_OFFSET_IWRAM,
            )?;
            pointer
                .checked_sub(GBA_EWRAM_START)
                .map(|offset| offset as usize)
                .ok_or_else(|| format!("Emerald storage pointer is invalid: 0x{pointer:08X}"))?
        }
        Gen3Game::FireRed | Gen3Game::LeafGreen => {
            let pointer = read_u32_at(
                memory,
                regions.iwram_address + FRLG_POKEMON_STORAGE_PTR_OFFSET_IWRAM,
            )?;
            pointer
                .checked_sub(GBA_EWRAM_START)
                .map(|offset| offset as usize)
                .ok_or_else(|| {
                    format!("FireRed/LeafGreen storage pointer is invalid: 0x{pointer:08X}")
                })?
        }
    };

    let storage_size = POKEMON_STORAGE_BOXES_OFFSET
        + POKEMON_STORAGE_BOX_COUNT * POKEMON_STORAGE_BOX_CAPACITY * BOX_POKEMON_SIZE;
    if storage_offset & 3 != 0 || storage_offset + storage_size > 0x40000 {
        return Err(format!(
            "PC storage address is outside EWRAM: offset=0x{storage_offset:X}, size=0x{storage_size:X}"
        ));
    }

    Ok(regions.ewram_address + storage_offset)
}

fn read_pc_storage(
    memory: &ProcessMemory,
    regions: &crate::memory::memory_scanner::MemoryRegions,
    game: Gen3Game,
) -> Result<(PcStorageUpdate, usize), String> {
    let storage_address = resolve_pc_storage_address(memory, regions, game)?;
    let storage_size = POKEMON_STORAGE_BOXES_OFFSET
        + POKEMON_STORAGE_BOX_COUNT * POKEMON_STORAGE_BOX_CAPACITY * BOX_POKEMON_SIZE;
    let storage = memory.read(storage_address, storage_size)?;
    if storage.len() != storage_size {
        return Err(format!(
            "Short PC storage read at 0x{storage_address:X}: got {} of {storage_size} bytes",
            storage.len()
        ));
    }

    let current_box = storage[0];
    if current_box as usize >= POKEMON_STORAGE_BOX_COUNT {
        return Err(format!(
            "PC storage current box is out of range: {current_box}"
        ));
    }

    let mut total_pokemon = 0u16;
    let mut boxes = Vec::with_capacity(POKEMON_STORAGE_BOX_COUNT);
    for box_index in 0..POKEMON_STORAGE_BOX_COUNT {
        let mut slots = Vec::with_capacity(POKEMON_STORAGE_BOX_CAPACITY);
        let mut invalid_slots = Vec::new();
        let mut occupied_count = 0u8;
        for slot_index in 0..POKEMON_STORAGE_BOX_CAPACITY {
            let offset = POKEMON_STORAGE_BOXES_OFFSET
                + (box_index * POKEMON_STORAGE_BOX_CAPACITY + slot_index) * BOX_POKEMON_SIZE;
            let raw = &storage[offset..offset + BOX_POKEMON_SIZE];
            let has_species = raw[19] & 0x02 != 0;
            if !has_species {
                slots.push(None);
                continue;
            }
            occupied_count += 1;

            match decode_pokemon(raw) {
                Ok(decoded) => {
                    let mut pokemon = to_scanned_pokemon(decoded, (slot_index + 1) as u8);
                    if pokemon.species == 0 && raw[19] & 0x04 != 0 {
                        pokemon.species_name = "Egg".to_string();
                    }
                    total_pokemon += 1;
                    slots.push(Some(pokemon));
                }
                Err(_) => {
                    invalid_slots.push((slot_index + 1) as u8);
                    slots.push(None);
                }
            }
        }
        boxes.push(PcBoxSnapshot {
            index: box_index as u8,
            occupied_count,
            invalid_slots,
            slots,
        });
    }

    Ok((
        PcStorageUpdate {
            connected: true,
            supported: true,
            available: true,
            current_box: Some(current_box),
            total_pokemon,
            boxes,
            message: "Live PC storage memory read successfully.".to_string(),
        },
        storage_address,
    ))
}

fn read_emerald_pokeball_count(
    memory: &ProcessMemory,
    regions: &crate::memory::memory_scanner::MemoryRegions,
) -> Result<Option<u16>, String> {
    let save1_ptr = read_u32_at(
        memory,
        regions.iwram_address + EMERALD_SAVEBLOCK1_PTR_OFFSET_IWRAM,
    )?;
    let save2_ptr = read_u32_at(
        memory,
        regions.iwram_address + EMERALD_SAVEBLOCK2_PTR_OFFSET_IWRAM,
    )?;
    let save1_offset = save1_ptr.checked_sub(0x0200_0000).map(|v| v as usize);
    let save2_offset = save2_ptr.checked_sub(0x0200_0000).map(|v| v as usize);
    let (Some(save1_offset), Some(save2_offset)) = (save1_offset, save2_offset) else {
        return Ok(None);
    };
    // EWRAM is 256 KiB. Reject null, stale, or malformed save pointers.
    if save1_offset + EMERALD_BAG_POKEBALLS_OFFSET + EMERALD_BAG_POKEBALLS_SLOT_COUNT * 4 > 0x40000
        || save2_offset + EMERALD_SAVE_BLOCK2_ENCRYPTION_KEY_OFFSET + 4 > 0x40000
        || save1_offset & 3 != 0
        || save2_offset & 3 != 0
    {
        return Ok(None);
    }

    let encryption_key = memory.read(
        regions.ewram_address + save2_offset + EMERALD_SAVE_BLOCK2_ENCRYPTION_KEY_OFFSET,
        4,
    )?;
    if encryption_key.len() != 4 {
        return Ok(None);
    }
    let key = u32::from_le_bytes([
        encryption_key[0],
        encryption_key[1],
        encryption_key[2],
        encryption_key[3],
    ]) as u16;
    let slots = memory.read(
        regions.ewram_address + save1_offset + EMERALD_BAG_POKEBALLS_OFFSET,
        EMERALD_BAG_POKEBALLS_SLOT_COUNT * 4,
    )?;
    if slots.len() != EMERALD_BAG_POKEBALLS_SLOT_COUNT * 4 {
        return Ok(None);
    }

    let mut total = 0u32;
    for slot in slots.chunks_exact(4) {
        let item_id = u16::from_le_bytes([slot[0], slot[1]]);
        // Emerald ball items occupy IDs 1 through 12 (Master through
        // Premier Ball). Count every usable ball, not only the basic Ball.
        if (1..=EMERALD_LAST_BALL_ITEM_ID).contains(&item_id) {
            let encrypted_quantity = u16::from_le_bytes([slot[2], slot[3]]);
            total += (encrypted_quantity ^ key) as u32;
        }
    }
    Ok(Some(total.min(u16::MAX as u32) as u16))
}

/// Locate Emerald's `gBagPosition` while the Bag UI is active. It is an
/// EWRAM global, so scan for its validated struct signature instead of
/// assuming a fixed EWRAM address that may differ between ROM builds.
fn read_emerald_bag_position(
    memory: &ProcessMemory,
    regions: &crate::memory::memory_scanner::MemoryRegions,
) -> Result<Option<(u8, u16)>, String> {
    let ewram = memory.read(regions.ewram_address, 0x40000)?;
    if ewram.len() != 0x40000 {
        return Ok(None);
    }
    let mut match_value = None;
    let mut match_count = 0usize;
    for offset in (0..=ewram.len() - EMERALD_BAG_POSITION_SIZE).step_by(4) {
        let callback = u32::from_le_bytes([
            ewram[offset],
            ewram[offset + 1],
            ewram[offset + 2],
            ewram[offset + 3],
        ]);
        let location = ewram[offset + 4];
        let pocket = ewram[offset + 5];
        if !(0x0800_0000..0x0A00_0000).contains(&callback)
            || location != 1 // ITEMMENULOCATION_BATTLE
            || pocket as usize >= EMERALD_BAG_POCKET_COUNT
        {
            continue;
        }

        let cursor_start = offset + 8;
        let scroll_start = cursor_start + EMERALD_BAG_POCKET_COUNT * 2;
        let arrays_are_plausible = (0..EMERALD_BAG_POCKET_COUNT).all(|index| {
            let cursor = u16::from_le_bytes([
                ewram[cursor_start + index * 2],
                ewram[cursor_start + index * 2 + 1],
            ]);
            let scroll = u16::from_le_bytes([
                ewram[scroll_start + index * 2],
                ewram[scroll_start + index * 2 + 1],
            ]);
            cursor <= 64 && scroll <= 64
        });
        if !arrays_are_plausible {
            continue;
        }

        let cursor = u16::from_le_bytes([
            ewram[cursor_start + pocket as usize * 2],
            ewram[cursor_start + pocket as usize * 2 + 1],
        ]);
        match_value = Some((pocket, cursor));
        match_count += 1;
    }

    Ok((match_count == 1).then_some(match_value).flatten())
}

fn find_action_battler(controller_functions: &[u32]) -> Option<usize> {
    controller_functions
        .iter()
        .position(|function| *function == EMERALD_PLAYER_ACTION_INPUT_HANDLER)
}

fn find_intro_text_battler(
    controller_functions: &[u32],
    buffer_commands: &[u8],
    buffer_string_ids: &[u16],
) -> Option<usize> {
    controller_functions
        .iter()
        .enumerate()
        .find(|(battler, function)| {
            **function == EMERALD_PLAYER_WAIT_TEXT_HANDLER
                && buffer_commands.get(*battler) == Some(&16) // CONTROLLER_PRINTSTRING
                && buffer_string_ids.get(*battler) == Some(&EMERALD_INTRO_MESSAGE_STRING_ID)
        })
        .map(|(battler, _)| battler)
}

fn find_wild_escape_text_battler(
    controller_functions: &[u32],
    buffer_commands: &[u8],
    buffer_string_ids: &[u16],
) -> Option<usize> {
    controller_functions
        .iter()
        .enumerate()
        .find(|(battler, function)| {
            **function == EMERALD_PLAYER_WAIT_TEXT_HANDLER
                && buffer_commands.get(*battler) == Some(&16)
                && buffer_string_ids.get(*battler) == Some(&EMERALD_GOT_AWAY_SAFELY_STRING_ID)
        })
        .map(|(battler, _)| battler)
}

fn find_battle_text_battler(controller_functions: &[u32], buffer_commands: &[u8]) -> Option<usize> {
    controller_functions
        .iter()
        .enumerate()
        .find(|(battler, function)| {
            **function == EMERALD_PLAYER_WAIT_TEXT_HANDLER
                && buffer_commands.get(*battler) == Some(&16)
        })
        .map(|(battler, _)| battler)
}

/// Read the US Emerald main callback and active task functions from VBA's IWRAM.
fn read_emerald_runtime_state(
    memory: &ProcessMemory,
    regions: &crate::memory::memory_scanner::MemoryRegions,
) -> Result<EmeraldRuntimeState, String> {
    let main_address = regions.iwram_address + EMERALD_GMAIN_OFFSET_IWRAM;
    let callback2 = read_u32_at(memory, main_address + EMERALD_MAIN_CALLBACK2_OFFSET)? & !1;
    let main_state = memory.read(main_address + EMERALD_GMAIN_IN_BATTLE_OFFSET, 1)?[0];
    let in_battle = main_state & 0x02 != 0;
    let active_battler = memory.read(
        regions.ewram_address + EMERALD_ACTIVE_BATTLER_OFFSET_EWRAM,
        1,
    )?[0];
    let controller_flags = read_u32_at(
        memory,
        regions.ewram_address + EMERALD_BATTLE_CONTROLLER_FLAGS_OFFSET_EWRAM,
    )?;
    // gActiveBattler is reused as a loop variable by Emerald and can point one
    // past the real battlers between frames. Locate player input by scanning
    // all controller callbacks instead of indexing with gActiveBattler.
    let battler_controller_functions = (0..4)
        .map(|battler| {
            read_u32_at(
                memory,
                regions.iwram_address + EMERALD_BATTLE_CONTROLLER_FUNCS_OFFSET_IWRAM + battler * 4,
            )
            .map(|function| function & !1)
        })
        .collect::<Result<Vec<_>, _>>()?;
    let battler_buffer_addresses: Vec<usize> = (0..4)
        .map(|battler| {
            regions.ewram_address
                + EMERALD_BATTLE_BUFFER_A_OFFSET_EWRAM
                + battler * EMERALD_BATTLE_BUFFER_SIZE
        })
        .collect();
    let mut battler_buffer_commands = Vec::with_capacity(4);
    let mut battler_buffer_string_ids = Vec::with_capacity(4);
    for address in &battler_buffer_addresses {
        battler_buffer_commands.push(memory.read(*address, 1)?[0]);
        let string_id_bytes = memory.read(*address + 2, 2)?;
        battler_buffer_string_ids
            .push(u16::from_le_bytes([string_id_bytes[0], string_id_bytes[1]]));
    }

    let action_battler = find_action_battler(&battler_controller_functions);
    let intro_text_battler = find_intro_text_battler(
        &battler_controller_functions,
        &battler_buffer_commands,
        &battler_buffer_string_ids,
    );
    let wild_escape_text_battler = find_wild_escape_text_battler(
        &battler_controller_functions,
        &battler_buffer_commands,
        &battler_buffer_string_ids,
    );
    let battle_text_battler =
        find_battle_text_battler(&battler_controller_functions, &battler_buffer_commands);
    let selected_battler = action_battler
        .or(intro_text_battler)
        .or(wild_escape_text_battler)
        .or(battle_text_battler)
        .or_else(|| (active_battler < 4).then_some(active_battler as usize));
    let action_cursor = action_battler
        .map(|battler| {
            memory.read(
                regions.ewram_address + EMERALD_ACTION_SELECTION_CURSOR_OFFSET_EWRAM + battler,
                1,
            )
        })
        .transpose()?
        .map(|bytes| bytes[0]);
    let battle_action_menu_ready =
        action_battler.is_some() && action_cursor.is_some_and(|cursor| cursor <= 3);
    let battle_action_menu_cursor = battle_action_menu_ready.then_some(action_cursor).flatten();
    let selected_index = selected_battler;
    let controller_function = selected_index.map(|index| battler_controller_functions[index]);
    let battle_buffer_address = selected_index.map(|index| battler_buffer_addresses[index]);
    let battle_buffer_command = selected_index.map(|index| battler_buffer_commands[index]);
    let battle_buffer_string_id = selected_index.map(|index| battler_buffer_string_ids[index]);
    let mut starter_selection_ready = false;
    let mut starter_confirmation_ready = false;
    let mut starter_selection = None;

    let starter_task_functions = [
        EMERALD_STARTER_INIT_TASK,
        EMERALD_STARTER_INPUT_TASK,
        EMERALD_STARTER_WAIT_TASK,
        EMERALD_STARTER_ASK_TASK,
        EMERALD_STARTER_CONFIRM_TASK,
        EMERALD_STARTER_DECLINE_TASK,
        EMERALD_STARTER_MOVE_TASK,
        EMERALD_STARTER_CREATE_LABEL_TASK,
    ];

    for task_index in 0..EMERALD_TASK_COUNT {
        let task_address =
            regions.iwram_address + EMERALD_TASKS_OFFSET_IWRAM + task_index * EMERALD_TASK_SIZE;
        let task_header = memory.read(task_address, 8)?;
        let function = u32::from_le_bytes([
            task_header[0],
            task_header[1],
            task_header[2],
            task_header[3],
        ]) & !1;
        let active = task_header[4] != 0;
        if !active {
            continue;
        }

        if starter_task_functions
            .iter()
            .any(|candidate| function == (candidate & !1))
        {
            let selection_bytes = memory.read(task_address + 8, 2)?;
            let selection = u16::from_le_bytes([selection_bytes[0], selection_bytes[1]]);
            if selection <= 2 {
                starter_selection = Some(selection as u8);
            }
        }

        if function == (EMERALD_STARTER_INPUT_TASK & !1) {
            starter_selection_ready = true;
        } else if function == (EMERALD_STARTER_CONFIRM_TASK & !1) {
            starter_confirmation_ready = true;
        }
    }

    Ok(EmeraldRuntimeState {
        callback2,
        starter_selection_ready: callback2 == EMERALD_STARTER_SCREEN_CALLBACK
            && starter_selection_ready,
        starter_confirmation_ready: callback2 == EMERALD_STARTER_SCREEN_CALLBACK
            && starter_confirmation_ready,
        starter_selection: if callback2 == EMERALD_STARTER_SCREEN_CALLBACK {
            starter_selection
        } else {
            None
        },
        battle_action_menu_ready,
        battle_action_menu_cursor,
        active_battler: selected_index.map(|battler| battler as u8),
        controller_function,
        controller_exec_flags: Some(controller_flags),
        action_cursor,
        in_battle,
        battle_buffer_address,
        battle_buffer_command,
        battle_buffer_string_id,
        wild_escaped_text_ready: wild_escape_text_battler.is_some(),
        battle_text_ready: battle_text_battler.is_some(),
        battler_controller_functions,
        battler_buffer_commands,
        battler_buffer_string_ids,
    })
}

/// Check the live VBA task state immediately before sending an intro A press.
fn emerald_starter_screen_active() -> Result<bool, String> {
    let Some(process) = find_process(PROCESS_NAME)? else {
        return Ok(false);
    };
    let Some(module) = find_module(process.pid, PROCESS_NAME)? else {
        return Ok(false);
    };
    let memory = ProcessMemory::open(process.pid)?;
    let regions = resolve_memory_regions(&memory, &module)?;
    let runtime = read_emerald_runtime_state(&memory, &regions)?;
    Ok(runtime.starter_selection_ready || runtime.starter_confirmation_ready)
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
    battle_ui_diagnostic_enabled: Arc<AtomicBool>,
}

impl Default for ScannerState {
    fn default() -> Self {
        Self {
            running: Arc::new(AtomicBool::new(false)),
            worker: Mutex::new(None),
            last_opponent: Arc::new(Mutex::new(None)),
            battle_ui_diagnostic_enabled: Arc::new(AtomicBool::new(false)),
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
        let diagnostic_enabled = Arc::clone(&self.battle_ui_diagnostic_enabled);
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
                let mut last_pc_storage: Option<PcStorageUpdate> = None;
                let mut last_pc_storage_diagnostic: Option<String> = None;
                let mut last_game_state: Option<GameStateUpdate> = None;
                let mut last_error: Option<String> = None;
                let mut last_diagnostic_key = None;
                let mut menu_debouncer = BattleMenuDebouncer::default();

                // Remember the last wild Pokémon to avoid duplicate encounter events.
                // Preserve the current opponent across Stop/Start so restarting
                // during the same battle does not count the same Pokémon again.
                let mut last_emitted_opponent = previous_opponent;

                while running.load(Ordering::SeqCst) {
                    match scan_once(profile) {
                        Ok(mut snapshot) => {
                            if !running.load(Ordering::SeqCst) {
                                break;
                            }
                            last_error = None;

                            let raw_menu_ready = snapshot.game_state.battle_action_menu_ready;
                            snapshot.game_state.battle_action_menu_ready =
                                menu_debouncer.update(raw_menu_ready);
                            if !snapshot.game_state.battle_action_menu_ready {
                                snapshot.game_state.battle_action_menu_cursor = None;
                            }

                            if diagnostic_enabled.load(Ordering::SeqCst) {
                                if let Some(sample) = snapshot.battle_ui_diagnostic.as_ref() {
                                    let key = sample.transition_key();
                                    if last_diagnostic_key.as_ref() != Some(&key) {
                                        let _ = app.emit("battle-ui-diagnostic-sample", sample);
                                        last_diagnostic_key = Some(key);
                                    }
                                }
                            } else {
                                last_diagnostic_key = None;
                            }

                            if last_game_state.as_ref() != Some(&snapshot.game_state) {
                                let _ = app.emit("game-state-updated", &snapshot.game_state);
                                last_game_state = Some(snapshot.game_state.clone());
                            }

                            if snapshot.pc_storage_diagnostic != last_pc_storage_diagnostic {
                                if let Some(diagnostic) = snapshot.pc_storage_diagnostic.as_ref() {
                                    log::warn!(
                                        "PC storage memory diagnostic [{}]: {}",
                                        profile.game.display_name(),
                                        diagnostic
                                    );
                                }
                                last_pc_storage_diagnostic =
                                    snapshot.pc_storage_diagnostic.clone();
                            }

                            if last_pc_storage.as_ref() != Some(&snapshot.pc_storage) {
                                let box_counts = snapshot
                                    .pc_storage
                                    .boxes
                                    .iter()
                                    .map(|box_data| {
                                        format!(
                                            "{:02}:{}{}{}",
                                            box_data.index + 1,
                                            box_data.occupied_count,
                                            if box_data.invalid_slots.is_empty() {
                                                String::new()
                                            } else {
                                                format!("+{} invalid", box_data.invalid_slots.len())
                                            },
                                            if snapshot.pc_storage.current_box == Some(box_data.index) {
                                                "*"
                                            } else {
                                                ""
                                            }
                                        )
                                    })
                                    .collect::<Vec<_>>()
                                    .join(", ");
                                log::info!(
                                    "PC storage scan [{}]: connected={}, available={}, address={}, box_array_offset=0x{:X}, current_box={}, decoded={}/420, boxes=[{}]",
                                    profile.game.display_name(),
                                    snapshot.pc_storage.connected,
                                    snapshot.pc_storage.available,
                                    snapshot
                                        .pc_storage_address
                                        .map(|address| format!("0x{address:08X}"))
                                        .unwrap_or_else(|| "unresolved".to_string()),
                                    POKEMON_STORAGE_BOXES_OFFSET,
                                    snapshot
                                        .pc_storage
                                        .current_box
                                        .map(|index| (index + 1).to_string())
                                        .unwrap_or_else(|| "unknown".to_string()),
                                    snapshot.pc_storage.total_pokemon,
                                    box_counts
                                );
                                for box_data in &snapshot.pc_storage.boxes {
                                    let contents = box_data
                                        .slots
                                        .iter()
                                        .enumerate()
                                        .filter_map(|(slot, pokemon)| {
                                            pokemon.as_ref().map(|pokemon| {
                                                format!(
                                                    "{}:{}{}(SV {})",
                                                    slot + 1,
                                                    pokemon.species_name,
                                                    if pokemon.shiny { "*" } else { "" },
                                                    pokemon.shiny_value
                                                )
                                            })
                                        })
                                        .collect::<Vec<_>>()
                                        .join(", ");
                                    log::info!(
                                        "PC storage box {:02}: {}/30 decoded, invalid_slots={:?}, contents=[{}]",
                                        box_data.index + 1,
                                        box_data.occupied_count,
                                        box_data.invalid_slots,
                                        contents
                                    );
                                }
                                let _ = app.emit("pc-storage-updated", &snapshot.pc_storage);
                                last_pc_storage = Some(snapshot.pc_storage.clone());
                            }

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
                                let in_battle = snapshot
                                    .game_state
                                    .emerald_in_battle
                                    .unwrap_or(snapshot.battle_type_flags != 0)
                                    && snapshot.battle_type_flags != 0
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
        self.battle_ui_diagnostic_enabled
            .store(false, Ordering::SeqCst);

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

        shiny_value: decoded.shiny_value,
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
        evs: PokemonEvs {
            hp: decoded.evs.hp,
            attack: decoded.evs.attack,
            defense: decoded.evs.defense,
            speed: decoded.evs.speed,
            special_attack: decoded.evs.special_attack,
            special_defense: decoded.evs.special_defense,
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

    let (pc_storage, pc_storage_address, pc_storage_diagnostic) = if party.count == 0 {
        (
            PcStorageUpdate::unavailable(
                true,
                true,
                "Waiting for a loaded game save before scanning PC storage.".to_string(),
            ),
            None,
            Some("Party count is zero; deferred storage scan until a save is loaded.".to_string()),
        )
    } else {
        match read_pc_storage(&memory, &regions, profile.game) {
            Ok((storage, address)) => (storage, Some(address), None),
            Err(error) => (
                PcStorageUpdate::unavailable(
                    true,
                    true,
                    "PC storage could not be read. Check the terminal diagnostic.".to_string(),
                ),
                None,
                Some(error),
            ),
        }
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

    let emerald_runtime = if profile.game == Gen3Game::Emerald {
        read_emerald_runtime_state(&memory, &regions)?
    } else {
        EmeraldRuntimeState::default()
    };
    let emerald_pokeball_count = if profile.game == Gen3Game::Emerald {
        read_emerald_pokeball_count(&memory, &regions)?
    } else {
        None
    };
    let emerald_bag_position = if profile.game == Gen3Game::Emerald
        && emerald_runtime.in_battle
        && !emerald_runtime.battle_action_menu_ready
        && !emerald_runtime.battle_text_ready
    {
        read_emerald_bag_position(&memory, &regions)?
    } else {
        None
    };
    let (
        emerald_nickname_prompt_ready,
        emerald_nickname_prompt_cursor,
        emerald_dex_registration_ready,
        emerald_catch_confirmed,
        emerald_battle_outcome,
        emerald_battle_communication,
        emerald_battle_text_string_id,
        emerald_caught_species,
        emerald_wild_species,
        emerald_dex_task_id,
        emerald_dex_task_function,
        emerald_dex_task_active,
        emerald_dex_page_input_ready,
        emerald_naming_screen_active,
    ) = if profile.game == Gen3Game::Emerald {
        let communication = memory.read(
            regions.ewram_address + EMERALD_BATTLE_COMMUNICATION_OFFSET_EWRAM,
            8,
        )?;
        let outcome = memory.read(
            regions.ewram_address + EMERALD_BATTLE_OUTCOME_OFFSET_EWRAM,
            1,
        )?[0];
        let caught_species_bytes = memory.read(
            regions.iwram_address
                + EMERALD_BATTLE_RESULTS_OFFSET_IWRAM
                + EMERALD_CAUGHT_SPECIES_OFFSET_BATTLE_RESULTS,
            2,
        )?;
        let caught_species = u16::from_le_bytes([caught_species_bytes[0], caught_species_bytes[1]]);
        let wild_species = opponent.as_ref().map(|pokemon| pokemon.species);
        // caughtMonSpecies can remain populated after the capture script ends.
        // Require the matching opponent species and an active post-capture
        // communication state so a later encounter with the same species is
        // not mistaken for another successful catch.
        let caught_opponent_matches = caught_species != 0
            && wild_species.is_some_and(|species| species == caught_species)
            && matches!(communication[0], 1 | 2);
        let gotcha_text = emerald_runtime.battle_text_ready
            && emerald_runtime.battle_buffer_command == Some(16)
            && emerald_runtime.battle_buffer_string_id
                == Some(EMERALD_GOTCHA_CAUGHT_MESSAGE_STRING_ID);
        let catch_confirmed_now = (outcome == 7 || caught_opponent_matches || gotcha_text)
            && emerald_runtime.in_battle
            && battle_type_flags != 0
            && wild_species.is_some();
        let overworld_ready =
            emerald_runtime.callback2 == EMERALD_OVERWORLD_CALLBACK2 && !emerald_runtime.in_battle;
        if overworld_ready {
            EMERALD_CAPTURE_CONFIRMED_LATCHED.store(false, Ordering::Relaxed);
        } else if catch_confirmed_now {
            EMERALD_CAPTURE_CONFIRMED_LATCHED.store(true, Ordering::Relaxed);
        }
        let catch_confirmed =
            catch_confirmed_now || EMERALD_CAPTURE_CONFIRMED_LATCHED.load(Ordering::Relaxed);
        let task_id = communication[1];
        let task_entry = if task_id < EMERALD_TASK_COUNT as u8 {
            let address = regions.iwram_address
                + EMERALD_TASKS_OFFSET_IWRAM
                + task_id as usize * EMERALD_TASK_SIZE;
            Some(memory.read(address, EMERALD_TASK_SIZE)?)
        } else {
            None
        };
        let task_function = task_entry
            .as_ref()
            .map(|entry| u32::from_le_bytes([entry[0], entry[1], entry[2], entry[3]]));
        let task_active = task_entry
            .as_ref()
            .map(|entry| entry[EMERALD_TASK_ACTIVE_OFFSET] != 0);
        let known_dex_task = task_function.is_some_and(|function| {
            matches!(
                function,
                EMERALD_DEX_TASK_DISPLAY | EMERALD_DEX_TASK_INPUT | EMERALD_DEX_TASK_EXIT
            )
        });
        let dex_task_active = communication[0] == 2
            && emerald_runtime.callback2 == EMERALD_BATTLE_MAIN_CALLBACK2
            && known_dex_task
            && task_active == Some(true);
        let dex_page_input_ready = dex_task_active && task_function == Some(EMERALD_DEX_TASK_INPUT);
        let nickname_ready = catch_confirmed
            && emerald_runtime.callback2 == EMERALD_BATTLE_MAIN_CALLBACK2
            && communication[0] == 1
            && task_id <= 1;
        let naming_screen_active = emerald_runtime.callback2 == EMERALD_NAMING_SCREEN_CALLBACK2;
        let dex_registration_ready = dex_task_active;
        let dex_task_id = (communication[0] == 2).then_some(task_id);
        let stage = if naming_screen_active {
            5
        } else if nickname_ready {
            4
        } else if communication[0] == 4 && catch_confirmed {
            6
        } else if dex_page_input_ready {
            2
        } else if dex_task_active {
            1
        } else if communication[0] == 2 && catch_confirmed {
            3
        } else if catch_confirmed {
            1
        } else if overworld_ready && EMERALD_CAPTURE_TRACE_LAST_STAGE.load(Ordering::Relaxed) != 0 {
            7
        } else {
            0
        };
        let previous_stage = EMERALD_CAPTURE_TRACE_LAST_STAGE.swap(stage, Ordering::Relaxed);
        if stage != 0 && stage != previous_stage {
            let sample_number = EMERALD_CAPTURE_TRACE_COUNT.fetch_add(1, Ordering::Relaxed);
            if sample_number < 24 {
                let script_pointer = read_u32_at(
                    &memory,
                    regions.ewram_address + EMERALD_BATTLE_SCRIPT_INSTR_OFFSET_EWRAM,
                )?;
                let keys = memory.read(
                    regions.iwram_address
                        + EMERALD_GMAIN_OFFSET_IWRAM
                        + EMERALD_MAIN_HELD_KEYS_RAW_OFFSET,
                    4,
                )?;
                let held_keys = u16::from_le_bytes([keys[0], keys[1]]);
                let new_keys = u16::from_le_bytes([keys[2], keys[3]]);
                let all_tasks = memory.read(
                    regions.iwram_address + EMERALD_TASKS_OFFSET_IWRAM,
                    EMERALD_TASK_COUNT * EMERALD_TASK_SIZE,
                )?;
                let active_tasks: Vec<(usize, u32, Vec<u8>)> = all_tasks
                    .chunks_exact(EMERALD_TASK_SIZE)
                    .enumerate()
                    .filter(|(_, entry)| entry[EMERALD_TASK_ACTIVE_OFFSET] != 0)
                    .map(|(id, entry)| {
                        (
                            id,
                            u32::from_le_bytes([entry[0], entry[1], entry[2], entry[3]]),
                            entry[5..13].to_vec(),
                        )
                    })
                    .collect();
                let party_species: Vec<u16> = party_update
                    .pokemon
                    .iter()
                    .map(|pokemon| pokemon.species)
                    .collect();
                let stage_name = match stage {
                    1 => "caught_or_dex_loading",
                    2 => "pokedex_page_input_ready",
                    3 => "pokedex_task_finished",
                    4 => "nickname_decision",
                    5 => "nickname_editor_unsafe",
                    6 => "nickname_declined",
                    7 => "overworld_confirmed",
                    _ => "unknown",
                };
                log::warn!(
                    "Emerald post-capture transition {}/24: stage={stage_name}, outcome={outcome}, comm={communication:?}, caught_species={caught_species}, wild_species={wild_species:?}, caught_opponent_match={caught_opponent_matches}, gotcha_text={gotcha_text}, battle_text_id={:?}, buffer_command={:?}, controller_function={:?}, catch_confirmed={catch_confirmed}, dex_task_id={dex_task_id:?}, dex_task_function={task_function:?}, dex_task_active={task_active:?}, dex_input_ready={dex_page_input_ready}, nickname_prompt={nickname_ready}, naming_screen={naming_screen_active}, callback2=0x{:08X}, in_battle={}, battle_flags=0x{:08X}, battle_script=0x{:08X}, held_keys=0x{:04X}, new_keys=0x{:04X}, party_count={}, party_species={party_species:?}, active_tasks={active_tasks:?}",
                    sample_number + 1,
                    emerald_runtime.battle_buffer_string_id,
                    emerald_runtime.battle_buffer_command,
                    emerald_runtime.controller_function,
                    emerald_runtime.callback2,
                    emerald_runtime.in_battle,
                    battle_type_flags,
                    script_pointer,
                    held_keys,
                    new_keys,
                    party_update.party_count,
                );
            }
        }
        if !EMERALD_DEX_DIAGNOSTIC_LOGGED.swap(true, Ordering::Relaxed) {
            log::warn!(
                "Emerald one-time memory snapshot: outcome={outcome}, comm={communication:?}, caught_species={caught_species}, wild_species={wild_species:?}, caught_opponent_match={caught_opponent_matches}, dex_task_id={dex_task_id:?}, dex_task_function={task_function:?}, dex_task_active={task_active:?}, callback2=0x{:08X}, in_battle={}, battle_flags=0x{:08X}",
                emerald_runtime.callback2,
                emerald_runtime.in_battle,
                battle_type_flags,
            );
        }
        (
            nickname_ready,
            nickname_ready.then_some(task_id),
            dex_registration_ready,
            catch_confirmed,
            Some(outcome),
            Some(communication),
            emerald_runtime.battle_buffer_string_id,
            Some(caught_species),
            wild_species,
            dex_task_id,
            task_function,
            task_active,
            dex_page_input_ready,
            naming_screen_active,
        )
    } else {
        (
            false, None, false, false, None, None, None, None, None, None, None, None, false, false,
        )
    };
    let wild_appeared_text_ready = profile.game == Gen3Game::Emerald
        && emerald_runtime.in_battle
        && battle_type_flags != 0
        && !is_trainer_battle(battle_type_flags, profile)
        && opponent.is_some()
        && emerald_runtime.controller_function == Some(EMERALD_PLAYER_WAIT_TEXT_HANDLER)
        && emerald_runtime.battle_buffer_command == Some(16)
        && emerald_runtime.battle_buffer_string_id == Some(EMERALD_INTRO_MESSAGE_STRING_ID);
    let wild_escaped_text_ready = profile.game == Gen3Game::Emerald
        && emerald_runtime.in_battle
        && battle_type_flags != 0
        && !is_trainer_battle(battle_type_flags, profile)
        && emerald_runtime.wild_escaped_text_ready;
    let battle_text_ready = profile.game == Gen3Game::Emerald
        && emerald_runtime.in_battle
        && emerald_runtime.battle_text_ready;

    // Report only states supported by verified memory signals. Intro, dialogue,
    // and starter-menu screens remain unknown until their retail-ROM signals
    // have been identified and validated.
    let first_party_national_dex = party_update
        .pokemon
        .first()
        .map(|pokemon| pokemon.national_dex);
    let battle_ui_state = classify_emerald_battle_ui(
        emerald_runtime.in_battle,
        emerald_runtime.battle_action_menu_ready,
        wild_appeared_text_ready,
        wild_escaped_text_ready,
    );
    let (state, detail) = if battle_ui_state == EmeraldBattleUiState::CommandMenuReady {
        let cursor_name = match emerald_runtime.battle_action_menu_cursor {
            Some(0) => "Fight",
            Some(1) => "Bag",
            Some(2) => "Pokémon",
            Some(3) => "Run",
            _ => "unknown option",
        };
        (
            "battle_action_menu".to_string(),
            format!("Emerald battle action menu is active; cursor is on {cursor_name}."),
        )
    } else if battle_ui_state == EmeraldBattleUiState::WildIntroduction {
        (
            "wild_appeared_text".to_string(),
            "Wild intro text candidate is active in the player battle text handler.".to_string(),
        )
    } else if battle_ui_state == EmeraldBattleUiState::WildEscapeText {
        (
            "wild_escape_text".to_string(),
            "Emerald is displaying the wild battle escape confirmation text.".to_string(),
        )
    } else if profile.game == Gen3Game::Emerald
        && battle_ui_state == EmeraldBattleUiState::Overworld
        && !emerald_runtime.starter_selection_ready
    {
        (
            "overworld".to_string(),
            "Emerald gMain.inBattle is clear; game is outside an active battle.".to_string(),
        )
    } else {
        let (state, detail) = classify_game_state(
            party_update.party_count,
            first_party_national_dex,
            battle_type_flags,
            opponent.is_some(),
            emerald_runtime.starter_selection_ready,
            profile,
        );
        (state.to_string(), detail.to_string())
    };

    let game_state = GameStateUpdate {
        connected: true,
        state,
        detail,
        party_count: party_update.party_count,
        battle_type_flags,
        emerald_callback2: emerald_runtime.callback2,
        starter_selection_ready: emerald_runtime.starter_selection_ready,
        starter_confirmation_ready: emerald_runtime.starter_confirmation_ready,
        emerald_starter_selection: emerald_runtime.starter_selection,
        battle_action_menu_ready: emerald_runtime.battle_action_menu_ready,
        battle_action_menu_cursor: emerald_runtime.battle_action_menu_cursor,
        wild_appeared_text_ready,
        wild_escaped_text_ready,
        battle_text_ready,
        emerald_in_battle: (profile.game == Gen3Game::Emerald).then_some(emerald_runtime.in_battle),
        emerald_pokeball_count,
        emerald_bag_menu_ready: emerald_bag_position.is_some(),
        emerald_bag_pocket: emerald_bag_position.map(|(pocket, _)| pocket),
        emerald_bag_cursor: emerald_bag_position.map(|(_, cursor)| cursor),
        emerald_nickname_prompt_ready,
        emerald_nickname_prompt_cursor,
        emerald_dex_registration_ready,
        emerald_catch_confirmed,
        emerald_overworld_ready: profile.game == Gen3Game::Emerald
            && emerald_runtime.callback2 == EMERALD_OVERWORLD_CALLBACK2
            && !emerald_runtime.in_battle,
        emerald_battle_outcome,
        emerald_battle_communication,
        emerald_battle_text_string_id,
        emerald_caught_species,
        emerald_wild_species,
        emerald_dex_task_id,
        emerald_dex_task_function,
        emerald_dex_task_active,
        emerald_dex_page_input_ready,
        emerald_naming_screen_active,
    };

    let battle_ui_diagnostic = if profile.game == Gen3Game::Emerald {
        let controller_address = emerald_runtime.active_battler.map(|battler| {
            regions.iwram_address
                + EMERALD_BATTLE_CONTROLLER_FUNCS_OFFSET_IWRAM
                + battler as usize * 4
        });
        let current_menu_candidate = emerald_runtime.battle_action_menu_ready;
        let candidate_state = if current_menu_candidate {
            "command_menu_candidate"
        } else if wild_appeared_text_ready {
            "wild_appeared_text_candidate"
        } else if wild_escaped_text_ready {
            "wild_escape_text_candidate"
        } else if !emerald_runtime.in_battle {
            "overworld_candidate"
        } else if opponent.is_some() {
            "wild_opponent_battle_candidate"
        } else if battle_type_flags == 0 {
            "battle_flags_zero_candidate"
        } else {
            "battle_flag_or_transition_candidate"
        };
        Some(BattleUiDiagnosticSample {
            timestamp_unix_ms: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis(),
            game_profile: "US Emerald layout selected in app; ROM fingerprint not checked".to_string(),
            address_basis: "VBA process addresses = resolved region base + Emerald memory offset; candidate values require live validation".to_string(),
            ewram_process_base: Some(format!("0x{:X}", regions.ewram_address)),
            iwram_process_base: Some(format!("0x{:X}", regions.iwram_address)),
            battle_type_flags_address: Some(format!("0x{:X}", regions.ewram_address + profile.battle_type_flags_offset)),
            battle_type_flags: Some(battle_type_flags),
            active_battler_address: Some(format!("0x{:X}", regions.ewram_address + EMERALD_ACTIVE_BATTLER_OFFSET_EWRAM)),
            active_battler: emerald_runtime.active_battler,
            controller_function_address: controller_address.map(|address| format!("0x{:X}", address)),
            controller_function: emerald_runtime.controller_function,
            battler_controller_functions: emerald_runtime.battler_controller_functions.clone(),
            controller_exec_flags_address: Some(format!("0x{:X}", regions.ewram_address + EMERALD_BATTLE_CONTROLLER_FLAGS_OFFSET_EWRAM)),
            controller_exec_flags: emerald_runtime.controller_exec_flags,
            action_cursor_address: emerald_runtime.active_battler.map(|battler| format!(
                "0x{:X}", regions.ewram_address + EMERALD_ACTION_SELECTION_CURSOR_OFFSET_EWRAM + battler as usize
            )),
            action_cursor: emerald_runtime.action_cursor,
            current_menu_candidate,
            candidate_state: candidate_state.to_string(),
            emerald_in_battle_address: Some(format!(
                "0x{:X}",
                regions.iwram_address
                    + EMERALD_GMAIN_OFFSET_IWRAM
                    + EMERALD_GMAIN_IN_BATTLE_OFFSET
            )),
            emerald_in_battle: Some(emerald_runtime.in_battle),
            battle_buffer_command_address: emerald_runtime
                .battle_buffer_address
                .map(|address| format!("0x{:X}", address)),
            battle_buffer_command: emerald_runtime.battle_buffer_command,
            battle_buffer_string_id: emerald_runtime.battle_buffer_string_id,
            battler_buffer_commands: emerald_runtime.battler_buffer_commands.clone(),
            battler_buffer_string_ids: emerald_runtime.battler_buffer_string_ids.clone(),
            wild_appeared_text_candidate: wild_appeared_text_ready,
            wild_escaped_text_candidate: wild_escaped_text_ready,
            emerald_callback2: Some(emerald_runtime.callback2),
            opponent_species: opponent.as_ref().map(|pokemon| pokemon.species),
            opponent_pid: opponent.as_ref().map(|pokemon| pokemon.pid),
            opponent_shiny: opponent.as_ref().map(|pokemon| pokemon.shiny),
            rom_fingerprint_verified: false,
        })
    } else {
        None
    };

    Ok(ScanSnapshot {
        party: party_update,
        pc_storage,
        pc_storage_address,
        pc_storage_diagnostic,
        battle_type_flags,
        opponent,
        game_state,
        battle_ui_diagnostic,
    })
}

#[cfg(test)]
mod game_state_tests {
    use super::{
        BattleMenuDebouncer, EmeraldBattleUiState, classify_emerald_battle_ui, classify_game_state,
        find_action_battler, find_intro_text_battler, find_wild_escape_text_battler,
    };
    use crate::games::gen3::profile::EMERALD_PROFILE;

    #[test]
    fn identifies_treecko_in_the_first_party_slot() {
        assert_eq!(
            classify_game_state(1, Some(252), 0, false, false, &EMERALD_PROFILE).0,
            "treecko_detected"
        );
    }

    #[test]
    fn identifies_wild_battle_only_when_opponent_data_is_valid() {
        assert_eq!(
            classify_game_state(0, None, 1, true, false, &EMERALD_PROFILE).0,
            "wild_battle"
        );
        assert_eq!(
            classify_game_state(0, None, 1, false, false, &EMERALD_PROFILE).0,
            "unknown"
        );
    }

    #[test]
    fn identifies_trainer_battle_from_profile_flags() {
        assert_eq!(
            classify_game_state(
                0,
                None,
                EMERALD_PROFILE.trainer_battle_mask,
                false,
                false,
                &EMERALD_PROFILE
            )
            .0,
            "trainer_battle"
        );
    }

    #[test]
    fn does_not_guess_intro_or_starter_menu_from_empty_party() {
        assert_eq!(
            classify_game_state(0, None, 0, false, false, &EMERALD_PROFILE).0,
            "unknown"
        );
    }

    #[test]
    fn recognizes_the_emerald_starter_selection_state() {
        assert_eq!(
            classify_game_state(0, None, 0, false, true, &EMERALD_PROFILE).0,
            "starter_selection"
        );
    }

    #[test]
    fn battle_ui_states_prioritize_overworld_and_action_menu_signals() {
        assert_eq!(
            classify_emerald_battle_ui(false, false, false, false),
            EmeraldBattleUiState::Overworld
        );
        assert_eq!(
            classify_emerald_battle_ui(true, true, true, false),
            EmeraldBattleUiState::CommandMenuReady
        );
        assert_eq!(
            classify_emerald_battle_ui(true, false, true, false),
            EmeraldBattleUiState::WildIntroduction
        );
        assert_eq!(
            classify_emerald_battle_ui(true, false, false, true),
            EmeraldBattleUiState::WildEscapeText
        );
        assert_eq!(
            classify_emerald_battle_ui(true, false, false, true),
            EmeraldBattleUiState::WildEscapeText
        );
        assert_eq!(
            classify_emerald_battle_ui(true, false, false, false),
            EmeraldBattleUiState::BattleTransition
        );
    }

    #[test]
    fn menu_ready_requires_two_samples_and_clears_immediately() {
        let mut debouncer = BattleMenuDebouncer::default();
        assert!(!debouncer.update(true));
        assert!(debouncer.update(true));
        assert!(!debouncer.update(false));
        assert!(!debouncer.update(true));
    }

    #[test]
    fn finds_menu_battler_by_controller_function_not_gactivebattler() {
        let functions = [0x0805_7588, 0x0805_7458, 0x0805_7458, 0x0805_7458];
        assert_eq!(find_action_battler(&functions), Some(0));
        assert_eq!(find_action_battler(&[0x0805_7458, 0x0805_7458]), None);
    }

    #[test]
    fn finds_intro_text_by_the_matching_battler_buffer() {
        let functions = [0x0805_7458, 0x0805_97B4, 0x0805_7458, 0x0805_7458];
        let commands = [0, 16, 0, 0];
        let string_ids = [0, 0, 0, 0];
        assert_eq!(
            find_intro_text_battler(&functions, &commands, &string_ids),
            Some(1)
        );
        assert_eq!(
            find_intro_text_battler(&functions, &[0, 0, 0, 0], &string_ids),
            None
        );
        assert_eq!(
            find_intro_text_battler(&functions, &commands, &[0, 223, 0, 0]),
            None
        );
    }

    #[test]
    fn recognizes_got_away_safely_from_its_live_text_handler_and_string_id() {
        let functions = [0x0805_97B4, 0x0805_7458, 0x0805_7458, 0x0805_7458];
        let commands = [16, 0, 0, 0];
        let ids = [223, 0, 0, 0];
        assert_eq!(
            find_wild_escape_text_battler(&functions, &commands, &ids),
            Some(0)
        );
        assert_eq!(
            find_wild_escape_text_battler(&functions, &commands, &[0, 0, 0, 0]),
            None
        );
    }
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

#[tauri::command]
pub fn set_battle_ui_diagnostic(
    enabled: bool,
    scanner: State<'_, ScannerState>,
) -> Result<(), String> {
    if enabled && !scanner.running.load(Ordering::SeqCst) {
        return Err("Start the scanner before enabling battle UI diagnostics.".to_string());
    }
    scanner
        .battle_ui_diagnostic_enabled
        .store(enabled, Ordering::SeqCst);
    Ok(())
}
