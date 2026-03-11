use std::sync::Mutex;
use tauri::menu::CheckMenuItem;

pub struct AppState {
    pub launch_at_login: Mutex<bool>,
    pub login_menu_item: Mutex<Option<CheckMenuItem<tauri::Wry>>>,
}
