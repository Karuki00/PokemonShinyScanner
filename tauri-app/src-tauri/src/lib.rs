#[path = "decoders/mods.rs"]
pub mod decoders;

#[path = "memory/mods.rs"]
pub mod memory;

#[path = "games/mods.rs"]
pub mod games;

#[path = "scanner/mods.rs"]
pub mod scanner;

use scanner::party_scanner::{
    ScannerState, press_emulator_a, send_emulator_key, send_emulator_turn,
    set_battle_ui_diagnostic, start_scanner, stop_scanner,
};
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        // Keep scanner state available to Tauri commands.
        .manage(ScannerState::default())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Debug)
                        .build(),
                )?;
            }

            Ok(())
        });

    if cfg!(debug_assertions) {
        builder
            .invoke_handler(tauri::generate_handler![
                start_scanner,
                stop_scanner,
                press_emulator_a,
                send_emulator_key,
                send_emulator_turn,
                set_battle_ui_diagnostic,
            ])
            .run(tauri::generate_context!())
            .expect("error while building tauri development application");
    } else {
        builder
            .invoke_handler(tauri::generate_handler![
                start_scanner,
                stop_scanner,
                press_emulator_a,
                send_emulator_key,
                send_emulator_turn,
            ])
            .run(tauri::generate_context!())
            .expect("error while building tauri application");
    }
}
