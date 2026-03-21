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

use crate::login::{set_dock_visibility, toggle_launch_at_login};
use crate::models::dock_app::DockApp;
use crate::shortcuts::reload_shortcuts;

use tauri::ActivationPolicy;
use tauri::Emitter;
use tauri::Manager;
use tauri_plugin_autostart::{MacosLauncher, ManagerExt};

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
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        // Register Rust functions for the Frontend
        .invoke_handler(tauri::generate_handler![
            get_dock_apps,
            refresh_dock_apps,
            toggle_launch_at_login,
            app_launcher::launch_app,
            set_shortcut_modifiers,
            get_config,
            set_dock_visibility,
        ])
        // Initial Setup
        .setup(|app| {
            // Setup Tray Menu (This also initializes AppState with Config)
            tray_menu::setup_tray_menu(app)?;

            let (show_dock_icon, launch_at_login, modifiers) = {
                let state: tauri::State<crate::state::app_state::AppState> = app.state();
                let config = state.config.lock().unwrap();
                (config.show_dock_icon, config.launch_at_login, config.modifiers.clone())
            };

            // Sync Launch at Login with OS
            let autolaunch_manager = app.autolaunch();
            if launch_at_login {
                let _ = autolaunch_manager.enable();
            } else {
                let _ = autolaunch_manager.disable();
            }

            // Apply Dock Icon Visibility (menu bar utility mode or regular)
            if show_dock_icon {
                let _ = app.set_activation_policy(ActivationPolicy::Regular);
                #[cfg(target_os = "macos")]
                crate::login::refresh_macos_dock_icon(app.handle());
            } else {
                let _ = app.set_activation_policy(ActivationPolicy::Accessory);
            }

            shortcuts::register_shortcuts(app.handle(), Some(&modifiers));

            // Show window on manual launch (not autostart)
            let args: Vec<String> = std::env::args().collect();
            if !args.contains(&"--autostart".to_string()) {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }

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
fn set_shortcut_modifiers(
    app: tauri::AppHandle,
    state: tauri::State<crate::state::app_state::AppState>,
    modifiers: Vec<String>,
) -> Result<(), String> {
    {
        let mut config = state.config.lock().unwrap();
        config.modifiers = modifiers.clone();
        config.save()?;
    }
    reload_shortcuts(&app, modifiers)
}

#[tauri::command]
fn get_config(
    state: tauri::State<crate::state::app_state::AppState>,
) -> crate::state::config::Config {
    state.config.lock().unwrap().clone()
}
