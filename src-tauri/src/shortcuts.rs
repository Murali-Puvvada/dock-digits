use tauri::AppHandle;
use tauri_plugin_global_shortcut::GlobalShortcutExt;

use crate::app_launcher::launch_app;
use crate::dock_reader::read_dock_apps;

pub fn build_shortcut(modifiers: &Vec<String>, position: u8) -> String {
    let mut parts: Vec<String> = Vec::new();

    for m in modifiers {
        match m.as_str() {
            "command" => parts.push("Cmd".to_string()),
            "option" => parts.push("Option".to_string()),
            "control" => parts.push("Ctrl".to_string()),
            "shift" => parts.push("Shift".to_string()),
            _ => {}
        }
    }

    parts.push(position.to_string());

    parts.join("+")
}

pub fn register_shortcuts(app: &AppHandle, modifiers: Option<&Vec<String>>) {
    let dock_apps = read_dock_apps();
    let default_modifiers = vec!["option".to_string()];
    let mods = modifiers.unwrap_or(&default_modifiers);

    for app_entry in dock_apps.into_iter().take(9) {
        let shortcut_string = build_shortcut(mods, app_entry.position as u8);

        if let Err(err) = app.global_shortcut().on_shortcut(
            shortcut_string.as_str(),
            move |_app, _shortcut, _event| {
                let _ = launch_app(app_entry.bundle_id.clone(), app_entry.path.clone());
            },
        ) {
            eprintln!("Failed to register shortcut {}: {}", shortcut_string, err);
        }
    }
}

pub fn reload_shortcuts(app: &AppHandle, modifiers: Vec<String>) -> Result<(), String> {
    app.global_shortcut()
        .unregister_all()
        .map_err(|_| "Failed to clear shortcuts")?;

    register_shortcuts(app, Some(&modifiers));

    Ok(())
}
