use tauri::AppHandle;
use tauri_plugin_global_shortcut::GlobalShortcutExt;

use crate::app_launcher::launch_app;
use crate::dock_reader::read_dock_apps;

pub fn register_shortcuts(app: &AppHandle) {
    let dock_apps = read_dock_apps();

    for app_entry in dock_apps.into_iter().take(9) {
        let shortcut_string = format!("Option+{}", app_entry.position);

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
