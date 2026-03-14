use crate::login::toggle_launch_at_login;
use crate::state::app_state::AppState;
use tauri::Emitter;
use tauri::Manager;
use tauri::State;
use tauri::{
    menu::{CheckMenuItem, Menu, MenuItem, PredefinedMenuItem},
    tray::TrayIconBuilder,
};
use tauri_plugin_autostart::ManagerExt;

pub fn setup_tray_menu(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    //Check if launch at login is enabled
    let autolaunch_manager = app.autolaunch();
    let enabled = autolaunch_manager.is_enabled().unwrap_or(false);

    // Tray Menu
    let open = MenuItem::with_id(app, "open", "Open Dock Digits", true, None::<&str>)?;
    let login =
        CheckMenuItem::with_id(app, "login", "Launch at Login", true, enabled, None::<&str>)?;
    let refresh = MenuItem::with_id(app, "refresh", "Refresh Dock Apps", true, None::<&str>)?;
    let settings = MenuItem::with_id(app, "settings", "Settings", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;

    let sep1 = PredefinedMenuItem::separator(app)?;
    let sep2 = PredefinedMenuItem::separator(app)?;
    let sep3 = PredefinedMenuItem::separator(app)?;

    let menu = Menu::with_items(
        app,
        &[
            &sep1, &open, &login, &refresh, &sep2, &settings, &sep3, &quit,
        ],
    )?;

    // Manage AppState with the item handle
    app.manage(AppState {
        launch_at_login: std::sync::Mutex::new(enabled),
        login_menu_item: std::sync::Mutex::new(Some(login.clone())),
    });

    // Tray Icon and onClick Launches the App
    let _tray = TrayIconBuilder::with_id("main")
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .on_menu_event(move |app, event| match event.id().as_ref() {
            "open" => {
                let window = app.get_webview_window("main").unwrap();
                window.show().unwrap();
                window.set_focus().unwrap();
            }
            "login" => {
                let state: State<AppState> = app.state();
                toggle_launch_at_login(app.clone(), state);
            }
            "refresh" => {
                let _ = app.emit("dock-apps-refreshed", ());
                println!("Refresh Dock Apps");
            }
            "settings" => {
                let window = app.get_webview_window("main").unwrap();
                window.show().unwrap();
                window.set_focus().unwrap();
                let _ = app.emit("open-settings", ());
            }
            "quit" => {
                app.exit(0);
            }
            _ => {}
        })
        .build(app)?;

    Ok(())
}
