use parking_lot::Mutex;
use std::path::PathBuf;

use crate::crypto::servicekeys::ServiceKeys;

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct AppConfig {
    /// Number of days to keep a record after soft-deletion
    pub soft_delete_retention_days: u64,
}

impl AppConfig {
    pub fn default() -> Self {
        AppConfig {
            soft_delete_retention_days: 30,
        }
    }
}

pub struct AppState {
    pub app_config: Mutex<AppConfig>,
    pub db_path: Mutex<Option<PathBuf>>,
    pub serivce_keys: Mutex<Option<ServiceKeys>>,
}