use tauri::{AppHandle};
use tauri_plugin_global_shortcut::{GlobalShortcutExt};

pub fn register_shortcuts(app: &AppHandle, count: usize) {
    let shortcut_manager = app.global_shortcut();

    for i in 1..=count {
        let shortcut = format!("Option+{}", i);

        if let Err(e) = shortcut_manager.register(shortcut.as_str()) {
            println!("Failed to register {}: {:?}", shortcut, e);
        } else {
            println!("Registered shortcut {}", shortcut);
        }
    }
}

pub fn handle_shortcut<T: std::fmt::Debug>(shortcut: &T) {
    println!("Shortcut pressed: {:?}", shortcut);
}