use serde::*;
use std::collections::HashMap;
use tokio::sync::Mutex;

lazy_static::lazy_static! {
    pub static ref SETTINGS_FILE: Mutex<String> = {

        let main_separator = std::path::MAIN_SEPARATOR;

        let mut result = std::env::var("HOME").unwrap();

        if !result.ends_with(main_separator){
            result.push(main_separator);
        }

        result.push_str(".my-commander");

        Mutex::new(result)
    };
}

#[derive(Default, Serialize, Deserialize)]
pub struct PanelPersistenceState {
    pub show_hidden_files: bool,
}

#[derive(Default, Serialize, Deserialize)]
pub struct PersistenceState {
    pub selected_paths: HashMap<String, bool>,
    pub left_panel: PanelPersistenceState,
    pub right_panel: PanelPersistenceState,
}

impl PersistenceState {
    pub async fn load() -> Self {
        let settings_file = SETTINGS_FILE.lock().await;

        let content = tokio::fs::read_to_string(settings_file.as_str())
            .await
            .unwrap_or_default();

        if content.is_empty() {
            return PersistenceState::default();
        }

        match serde_yaml::from_str(&content) {
            Ok(state) => state,
            Err(e) => {
                eprintln!("Error loading settings: {}", e);
                PersistenceState::default()
            }
        }
    }

    pub async fn save(&self) -> Result<(), String> {
        let settings_file = SETTINGS_FILE.lock().await;

        let result = serde_yaml::to_string(self).unwrap();

        let _ = tokio::fs::write(settings_file.as_str(), result.as_bytes()).await;

        Ok(())
    }
}
