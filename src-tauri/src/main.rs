// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app_entry;
mod dock_reader;
mod models;

use crate::models::dock_app::DockApp;
use app_entry::AppEntry;
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        // 1. Register Rust functions for the Frontend
        .invoke_handler(tauri::generate_handler![get_mock_apps, get_dock_apps])
        // 2. Add Plugins
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(|_, shortcut, event| {
                    if event.state == ShortcutState::Pressed {
                        println!("Shortcut pressed: {:?}", shortcut);
                    }
                })
                .build(),
        )
        // 3. Initial Setup
        .setup(|app| {
            // Register the actual keys
            app.global_shortcut()
                .register("Option+1")
                .expect("Failed to register shortcut");
            Ok(())
        })
        // 4. Start the engine (ONLY ONCE)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_mock_apps() -> Vec<AppEntry> {
    vec![AppEntry {
        id: "1".into(),
        name: "Safari".into(),
        bundle_id: "com.apple.Safari".into(),
        path: "/Applications/Safari.app".into(),
        icon_path: None,
        position: 1,
        disabled: false,
    }]
}

#[tauri::command]
fn get_dock_apps() -> Vec<DockApp> {
    dock_reader::read_dock_apps()
}
