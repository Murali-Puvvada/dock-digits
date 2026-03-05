// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
      .invoke_handler(tauri::generate_handler![
          get_mock_apps
      ])
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
  }

mod app_entry;

use app_entry::AppEntry;

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
