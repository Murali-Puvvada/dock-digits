use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DockApp {
    pub name: String,
    pub path: String,
    pub bundle_id: String,
    pub position: usize,
    pub icon_path: Option<String>,
}
