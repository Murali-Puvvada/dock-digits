use serde::Serialize;

#[derive(Serialize)]
pub struct DockApp {
    pub name: String,
    pub path: String,
    pub position: usize,
}
