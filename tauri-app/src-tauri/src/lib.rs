#[path = "decoders/mods.rs"]
pub mod decoders;

#[path = "memory/mods.rs"]
pub mod memory;

#[path = "games/mods.rs"]
pub mod games;

#[path = "scanner/mods.rs"]
pub mod scanner;

use scanner::party_scanner::{ScannerState, start_scanner, stop_scanner};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
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
        })
        // Commands callable from the React frontend.
        .invoke_handler(tauri::generate_handler![start_scanner, stop_scanner,])
        .run(tauri::generate_context!())
        .expect("error while building tauri application");
}
