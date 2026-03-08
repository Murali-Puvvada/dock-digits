use serde::Serialize;
use std::process::Command;
use tauri::command;

#[derive(Serialize)]
pub struct LaunchResult {
    success: bool,
    message: String,
}

#[command]
pub fn launch_app(
    bundle_id: Option<String>,
    path: Option<String>,
) -> Result<LaunchResult, LaunchResult> {
    // Prefer bundle id if provided
    if let Some(id) = bundle_id {
        let output = Command::new("open").arg("-b").arg(&id).output();

        match output {
            Ok(_) => {
                return Ok(LaunchResult {
                    success: true,
                    message: format!("App launched via bundle id: {}", id),
                });
            }
            Err(e) => {
                return Err(LaunchResult {
                    success: false,
                    message: format!("Failed to launch via bundle id: {}", e),
                });
            }
        }
    }

    // Fallback to path
    if let Some(app_path) = path {
        let output = Command::new("open").arg(&app_path).output();

        match output {
            Ok(_) => {
                return Ok(LaunchResult {
                    success: true,
                    message: format!("App launched via path: {}", app_path),
                });
            }
            Err(e) => {
                return Err(LaunchResult {
                    success: false,
                    message: format!("Failed to launch via path: {}", e),
                });
            }
        }
    }

    Err(LaunchResult {
        success: false,
        message: "Neither bundle_id nor path provided".into(),
    })
}
