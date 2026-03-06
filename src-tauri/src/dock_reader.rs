use plist::Value;
use std::path::PathBuf;

pub fn read_dock_apps() -> Vec<(String, String)> {
    let home = std::env::var("HOME").expect("Could not find HOME directory");
    let path = PathBuf::from(home).join("Library/Preferences/com.apple.dock.plist");

    // Reading the binary plist
    let plist = Value::from_file(&path).expect("Failed to read Dock plist");

    let mut apps = Vec::new();

    if let Some(dict) = plist.as_dictionary() {
        if let Some(persistent_apps) = dict.get("persistent-apps").and_then(|v| v.as_array()) {
            for item in persistent_apps {
                if let Some(tile_data) = item.as_dictionary().and_then(|i| i.get("tile-data")).and_then(|t| t.as_dictionary()) {
                    
                    // 1. Get the Label (The name displayed in the Dock)
                    let label = tile_data.get("file-label")
                        .and_then(|l| l.as_string())
                        .unwrap_or("Unknown")
                        .to_string();

                    // 2. Get the URL/Path
                    let url = tile_data.get("file-data")
                        .and_then(|fd| fd.as_dictionary())
                        .and_then(|fd_dict| fd_dict.get("_CFURLString"))
                        .and_then(|url_str| url_str.as_string())
                        .unwrap_or("")
                        .to_string();

                    if !url.is_empty() {
                        apps.push((label, url));
                    }
                }
            }
        }
    }

    apps
}