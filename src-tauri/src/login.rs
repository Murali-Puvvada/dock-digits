use crate::state::app_state::AppState;
use tauri::{ActivationPolicy, Emitter};
use tauri_plugin_autostart::ManagerExt;

#[cfg(target_os = "macos")]
use objc2_app_kit::{NSApplication, NSImage, NSWorkspace};
#[cfg(target_os = "macos")]
use objc2_foundation::{MainThreadMarker, NSString};
#[cfg(target_os = "macos")]
use objc2::AllocAnyThread;

#[tauri::command]
pub fn toggle_launch_at_login(app: tauri::AppHandle, state: tauri::State<'_, AppState>) -> bool {
    let autolaunch_manager = app.autolaunch();

    // 1. Update Config and Save
    let is_enabled = {
        let mut config = state.config.lock().unwrap();
        config.launch_at_login = !config.launch_at_login;
        let enabled = config.launch_at_login;
        let _ = config.save();
        enabled
    };

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

#[tauri::command]
pub fn set_dock_visibility(
    app: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
) -> Result<bool, String> {
    toggle_dock_icon_logic(&app, &state)
}

#[cfg(target_os = "macos")]
use objc2_foundation::NSData;

#[cfg(target_os = "macos")]
const ICON_BYTES: &[u8] = include_bytes!("../icons/icon.png");

#[cfg(target_os = "macos")]
pub(crate) fn refresh_macos_dock_icon(_app: &tauri::AppHandle) {
    if let Some(mtm) = MainThreadMarker::new() {
        unsafe {
            let ns_app = NSApplication::sharedApplication(mtm);

            // Use embedded icon bytes for maximum reliability
            let data = NSData::with_bytes(ICON_BYTES);
            if let Some(image) = NSImage::initWithData(NSImage::alloc(), &data) {
                ns_app.setApplicationIconImage(Some(&image));
                println!("Refreshed dock icon using embedded bytes");
            } else {
                // Fallback to workspace logic if data loading fails
                let workspace = NSWorkspace::sharedWorkspace();
                if let Ok(current_exe) = std::env::current_exe() {
                    let mut path = current_exe.clone();
                    let path_str_check = path.to_string_lossy();
                    if path_str_check.contains(".app/Contents/MacOS/") {
                        while let Some(parent) = path.parent() {
                            if parent.extension().map(|s| s == "app").unwrap_or(false) {
                                path = parent.to_path_buf();
                                break;
                            }
                            path = parent.to_path_buf();
                        }
                    }
                    let ns_path = NSString::from_str(&path.to_string_lossy());
                    let icon = workspace.iconForFile(&ns_path);
                    ns_app.setApplicationIconImage(Some(&icon));
                }
            }
        }
    }
}

pub fn toggle_dock_icon_logic(
    app: &tauri::AppHandle,
    state: &tauri::State<'_, AppState>,
) -> Result<bool, String> {
    // 1. Update Config and Save
    let show_dock_icon = {
        let mut config = state.config.lock().unwrap();
        config.show_dock_icon = !config.show_dock_icon;
        let show = config.show_dock_icon;
        config.save()?;
        show
    };

    // 2. Apply to App
    if show_dock_icon {
        let _ = app.set_activation_policy(ActivationPolicy::Regular);
        #[cfg(target_os = "macos")]
        refresh_macos_dock_icon(app);
    } else {
        let _ = app.set_activation_policy(ActivationPolicy::Accessory);
    }

    // 3. Update Tray Menu Checkbox
    if let Some(item) = state.show_dock_icon_menu_item.lock().unwrap().as_ref() {
        let _ = item.set_checked(show_dock_icon);
    }

    // 4. Emit event for Frontend sync
    let _ = app.emit("dock-icon-updated", show_dock_icon);

    Ok(show_dock_icon)
}
