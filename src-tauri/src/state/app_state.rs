use crate::state::config::Config;
use std::sync::Mutex;
use tauri::menu::CheckMenuItem;

pub struct AppState {
    pub login_menu_item: Mutex<Option<CheckMenuItem<tauri::Wry>>>,
    pub show_dock_icon_menu_item: Mutex<Option<CheckMenuItem<tauri::Wry>>>,
    pub config: Mutex<Config>,
}
