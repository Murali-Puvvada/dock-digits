// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app_launcher;
mod dock_reader;
mod icon_service;
mod models;
mod shortcuts;

use crate::models::dock_app::DockApp;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        // 1. Register Rust functions for the Frontend
        .invoke_handler(tauri::generate_handler![
            get_dock_apps,
            app_launcher::launch_app,
        ])
        // 2. Initial Setup
        .setup(|app| {
            shortcuts::register_shortcuts(&app.handle());

            Ok(())
        })
        // 3. Start the engine (ONLY ONCE)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_dock_apps() -> Vec<DockApp> {
    dock_reader::read_dock_apps()
}
