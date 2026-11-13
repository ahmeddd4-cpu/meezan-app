use crate::models::AppData;

#[cfg(not(target_arch = "wasm32"))]
pub fn load_data() -> AppData {
    if let Some(mut dir) = dirs::data_local_dir() {
        dir.push("meezan_app");
        dir.push("meezan_data.json");
        if let Ok(content) = std::fs::read_to_string(dir) {
            if let Ok(parsed) = serde_json::from_str::<AppData>(&content) {
                return parsed;
            }
        }
    }
    AppData::default()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn save_data(data: &AppData) {
    if let Some(mut dir) = dirs::data_local_dir() {
        dir.push("meezan_app");
        let _ = std::fs::create_dir_all(&dir);
        dir.push("meezan_data.json");
        if let Ok(json) = serde_json::to_string_pretty(data) {
            let _ = std::fs::write(dir, json);
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub fn load_data() -> AppData {
    use gloo_storage::{LocalStorage, Storage};
    LocalStorage::get("meezan_rust_data").unwrap_or_default()
}

#[cfg(target_arch = "wasm32")]
pub fn save_data(data: &AppData) {
    use gloo_storage::{LocalStorage, Storage};
    let _ = LocalStorage::set("meezan_rust_data", data);
}
