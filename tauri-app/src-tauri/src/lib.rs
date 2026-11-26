#[path = "decoders/mods.rs"]
pub mod decoders;

#[path = "memory/mods.rs"]
pub mod memory;

#[path = "games/mods.rs"]
pub mod games;

#[path = "scanner/mods.rs"]
pub mod scanner;

use rusqlite::{Connection, params};
use scanner::party_scanner::{
    ScannerState, press_emulator_a, send_emulator_key, send_emulator_turn,
    set_battle_ui_diagnostic, start_scanner, stop_scanner,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs};
use tauri::Manager;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct EvTarget {
    target: HashMap<String, i64>,
}

fn open_ev_database(app: &tauri::AppHandle) -> Result<Connection, String> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|error| error.to_string())?;
    fs::create_dir_all(&app_data_dir).map_err(|error| error.to_string())?;
    let database_path = app_data_dir.join("pokemon-shiny-scanner.sqlite3");
    let connection = Connection::open(database_path).map_err(|error| error.to_string())?;
    connection
        .execute_batch(
            "CREATE TABLE IF NOT EXISTS ev_training_targets (
                pokemon_key TEXT PRIMARY KEY NOT NULL,
                target_json TEXT NOT NULL,
                updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
            );",
        )
        .map_err(|error| error.to_string())?;
    Ok(connection)
}

#[tauri::command]
fn load_ev_targets(app: tauri::AppHandle) -> Result<HashMap<String, EvTarget>, String> {
    let connection = open_ev_database(&app)?;
    let mut statement = connection
        .prepare("SELECT pokemon_key, target_json FROM ev_training_targets")
        .map_err(|error| error.to_string())?;
    let rows = statement
        .query_map([], |row| {
            let key: String = row.get(0)?;
            let target_json: String = row.get(1)?;
            let target = serde_json::from_str(&target_json)
                .map_err(|error| rusqlite::Error::ToSqlConversionFailure(Box::new(error)))?;
            Ok((key, EvTarget { target }))
        })
        .map_err(|error| error.to_string())?;

    rows.collect::<Result<HashMap<_, _>, _>>()
        .map_err(|error| error.to_string())
}

#[tauri::command]
fn save_ev_targets(
    app: tauri::AppHandle,
    targets: HashMap<String, EvTarget>,
) -> Result<(), String> {
    let mut connection = open_ev_database(&app)?;
    let transaction = connection
        .transaction()
        .map_err(|error| error.to_string())?;
    for (key, value) in targets {
        let target_json =
            serde_json::to_string(&value.target).map_err(|error| error.to_string())?;
        transaction
            .execute(
                "INSERT INTO ev_training_targets (pokemon_key, target_json, updated_at)
                 VALUES (?1, ?2, CURRENT_TIMESTAMP)
                 ON CONFLICT(pokemon_key) DO UPDATE SET
                   target_json = excluded.target_json,
                   updated_at = CURRENT_TIMESTAMP",
                params![key, target_json],
            )
            .map_err(|error| error.to_string())?;
    }
    transaction.commit().map_err(|error| error.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        // Keep scanner state available to Tauri commands.
        .manage(ScannerState::default())
        .setup(|_| Ok(()));

    if cfg!(debug_assertions) {
        builder
            .invoke_handler(tauri::generate_handler![
                start_scanner,
                stop_scanner,
                press_emulator_a,
                send_emulator_key,
                send_emulator_turn,
                set_battle_ui_diagnostic,
                load_ev_targets,
                save_ev_targets,
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
                load_ev_targets,
                save_ev_targets,
            ])
            .run(tauri::generate_context!())
            .expect("error while building tauri application");
    }
}
