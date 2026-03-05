use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AppEntry {
    pub id: String,
    pub name: String,
    pub bundle_id: String,
    pub path: String,
    pub icon_path: Option<String>,
    pub position: u32,
    pub disabled: bool,
}