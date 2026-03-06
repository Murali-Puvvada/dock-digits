// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod models;
mod app_entry;
mod dock_reader;

use crate::models::dock_app::DockApp;
use app_entry::AppEntry;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_mock_apps, get_dock_apps])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_mock_apps() -> Vec<AppEntry> {
    vec![
        AppEntry {
            id: "1".into(),
            name: "Safari".into(),
            bundle_id: "com.apple.Safari".into(),
            path: "/Applications/Safari.app".into(),
            icon_path: None,
            position: 1,
            disabled: false,
        }
    ]
}

#[tauri::command]
fn get_dock_apps() -> Vec<DockApp> {
    dock_reader::read_dock_apps()
}