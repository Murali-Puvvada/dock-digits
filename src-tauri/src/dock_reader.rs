use plist::Value;
use std::path::PathBuf;
use urlencoding;

use crate::icon_service::get_icon_for_app;
use crate::models::dock_app::DockApp;

pub fn clean_dock_path(raw_url: &str) -> String {
    let path = raw_url.replace("file://", "");

    urlencoding::decode(&path)
        .map(|s| s.into_owned())
        .unwrap_or(path)
}

pub fn read_dock_apps() -> Vec<DockApp> {
    let home = std::env::var("HOME").expect("Could not find HOME directory");
    let path = PathBuf::from(home).join("Library/Preferences/com.apple.dock.plist");

    // Reading the binary plist
    let plist = Value::from_file(&path).expect("Failed to read Dock plist");

    let mut apps = Vec::new();
    let mut position = 1;

    if let Some(dict) = plist.as_dictionary() {
        if let Some(persistent_apps) = dict.get("persistent-apps").and_then(|v| v.as_array()) {
            for item in persistent_apps {
                // Stop after the first 9 apps
                if position > 9 {
                    break;
                }
                if let Some(tile_data) = item
                    .as_dictionary()
                    .and_then(|i| i.get("tile-data"))
                    .and_then(|t| t.as_dictionary())
                {
                    // 1. Get the Label (The name displayed in the Dock)
                    let label = tile_data
                        .get("file-label")
                        .and_then(|l| l.as_string())
                        .unwrap_or("Unknown")
                        .to_string();

                    let bundle_id = tile_data
                        .get("bundle-identifier")
                        .and_then(|l| l.as_string())
                        .unwrap_or("Unknown")
                        .to_string();

                    let raw_url = tile_data
                        .get("file-data")
                        .and_then(|fd| fd.as_dictionary())
                        .and_then(|fd_dict| fd_dict.get("_CFURLString"))
                        .and_then(|url_str| url_str.as_string())
                        .unwrap_or("");

                    let url = clean_dock_path(raw_url);

                    let icon_path = get_icon_for_app(&url);

                    if !url.is_empty() {
                        apps.push(DockApp {
                            name: label,
                            path: url,
                            bundle_id,
                            position,
                            icon_path,
                        });

                        position += 1;
                    }
                }
            }
        }
    }

    apps
}
