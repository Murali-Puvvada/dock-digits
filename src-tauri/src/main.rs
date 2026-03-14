// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app_launcher;
mod dock_reader;
mod icon_service;
mod login;
mod models;
mod shortcuts;
mod state;
mod tray_menu;

use crate::login::toggle_launch_at_login;
use crate::models::dock_app::DockApp;
use crate::shortcuts::reload_shortcuts;

use tauri::ActivationPolicy;
use tauri::Emitter;
use tauri::Manager;
use tauri_plugin_autostart::MacosLauncher;

use tauri_plugin_single_instance::init as single_instance;
fn main() {
    tauri::Builder::default()
        // Prevent multiple instances
        .plugin(single_instance(|app, _, _| {
            if let Some(window) = app.get_webview_window("main") {
                window.show().unwrap();
                window.set_focus().unwrap();
            }
        }))
        //Tells OS to register for launch at login if needed
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            None,
        ))
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        // Register Rust functions for the Frontend
        .invoke_handler(tauri::generate_handler![
            get_dock_apps,
            refresh_dock_apps,
            toggle_launch_at_login,
            app_launcher::launch_app,
            set_shortcut_modifiers,
        ])
        // Initial Setup
        .setup(|app| {
            // Hide Dock icon (menu bar utility mode)
            app.set_activation_policy(ActivationPolicy::Accessory);

            // Setup Tray Menu
            tray_menu::setup_tray_menu(app)?;

            shortcuts::register_shortcuts(&app.handle(), None);

            Ok(())
        })
        // Start the engine (ONLY ONCE)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_dock_apps() -> Vec<DockApp> {
    dock_reader::read_dock_apps()
}

#[tauri::command]
fn refresh_dock_apps(app: tauri::AppHandle) {
    let _ = app.emit("dock-apps-refreshed", ());
}

#[tauri::command]
fn set_shortcut_modifiers(app: tauri::AppHandle, modifiers: Vec<String>) -> Result<(), String> {
    reload_shortcuts(&app, modifiers)
}
