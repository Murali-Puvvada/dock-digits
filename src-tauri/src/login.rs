use crate::state::app_state::AppState;
use tauri::Emitter;
use tauri_plugin_autostart::ManagerExt;

#[tauri::command]
pub fn toggle_launch_at_login(app: tauri::AppHandle, state: tauri::State<'_, AppState>) -> bool {
    let autolaunch_manager = app.autolaunch();
    let mut enabled_lock = state.launch_at_login.lock().unwrap();
    
    // 1. Rust state updated
    *enabled_lock = !*enabled_lock;
    let is_enabled = *enabled_lock;

    // 2. macOS Launch Agent updated
    if is_enabled {
        let _ = autolaunch_manager.enable();
    } else {
        let _ = autolaunch_manager.disable();
    }

    // 3. Menu checkmark updated
    if let Some(item) = state.login_menu_item.lock().unwrap().as_ref() {
        let _ = item.set_checked(is_enabled);
    }

    // 4. Event emitted (Frontend toggle updated via listener)
    let _ = app.emit("launch-at-login-updated", is_enabled);

    println!("Launch at login toggled: {}", is_enabled);
    is_enabled
}
