use rand::rngs::OsRng;
use serde::Serialize;
use tauri_plugin_dialog::DialogExt;
use tracing::error;
use zeroize::Zeroize;

use crate::{
    database::open_database,
    services::{databasecleaner::purge_old_deleted_records, gatekeeper::{initialize_key_store, verify_password}},
    state::AppState,
};

#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "message")]
pub enum JournalOpeningError {
    Cancelled,
    InternalError(String),
    InvalidPassword,
    InvalidState,
}

#[tauri::command]
pub async fn create_journal(
    app: tauri::AppHandle,
    password: Vec<u8>,
    state: tauri::State<'_, AppState>,
) -> Result<(), JournalOpeningError> {
    let mut password = password;
    tracing::debug!("{}", password.len());
    let path = app
        .dialog()
        .file()
        .set_file_name("my_journal.db")
        .blocking_save_file()
        .ok_or(JournalOpeningError::Cancelled)?
        .into_path()
        .map_err(|_| JournalOpeningError::InternalError("Invalid Path".to_string()))?;
    tracing::debug!("Path");

    let mut conn = open_database(path.clone()).map_err(|e| {
        error!("open_database failed: {}", e);
        return JournalOpeningError::InternalError("The database didn't open.".to_string());
    })?;

    if let Err(e) = initialize_key_store(&mut OsRng, &mut conn, &password) {
        match e {
            crate::error::IterateError::SerializationFailed(error) => {
                return Err(JournalOpeningError::InternalError(error.to_string()));
            }
            crate::error::IterateError::SystemRngFailure => {
                return Err(JournalOpeningError::InternalError(
                    "Catastrophic system error".to_string(),
                ));
            }
            crate::error::IterateError::KeyDerivationFailed => {
                return Err(JournalOpeningError::InternalError(
                    "Crypto error".to_string(),
                ));
            }
            crate::error::IterateError::HkdfExpansionFailed => {
                return Err(JournalOpeningError::InternalError(
                    "Crypto error".to_string(),
                ));
            }
            crate::error::IterateError::Database(error) => {
                return Err(JournalOpeningError::InternalError(error.to_string()));
            }
            _ => todo!(),
        }
    }

    password.zeroize();
    let mut db_path_handle = state.db_path.lock();
    *db_path_handle = Some(path);

    Ok(())
}

#[tauri::command]
pub async fn open_journal(
    app: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
) -> Result<(), JournalOpeningError> {
    let file_path = app
        .dialog()
        .file()
        .add_filter("Journal Database", &["db", "sqlite", "iterate"])
        .blocking_pick_file();

    let path = match file_path {
        Some(p) => p
            .into_path()
            .map_err(|_| JournalOpeningError::InternalError("Invalid path".to_string()))?,
        None => return Err(JournalOpeningError::Cancelled), // Explicitly tell frontend it was cancelled
    };

    let conn = open_database(path.clone()).map_err(|e| {
        error!("open_database failed: {}", e);
        return JournalOpeningError::InternalError("The database didn't open.".to_string());
    })?;
    conn.execute("PRAGMA integrity_check;", [])
        .map_err(|_| JournalOpeningError::InternalError("DB Integrity failure".to_string()))?;

    let mut db_path_handle = state.db_path.lock();
    *db_path_handle = Some(path);

    let app_conf =  state.app_config.lock();
    if let Err(e) = purge_old_deleted_records(&conn, app_conf.soft_delete_retention_days) {
        error!("DB purge failed: {}", e);
    }

    Ok(())
}

#[tauri::command]
pub async fn unlock_journal(
    password: Vec<u8>,
    state: tauri::State<'_, AppState>,
) -> Result<(), JournalOpeningError> {
    let mut password = password;
    let db_path = {
        let guard = state.db_path.lock();
        guard
            .as_ref()
            .cloned()
            .ok_or(JournalOpeningError::InvalidState)?
    };

    let mut conn = open_database(db_path).map_err(|e| {
        error!("open_database failed: {}", e);
        return JournalOpeningError::InternalError("The database didn't open.".to_string());
    })?;

    let service_keys = verify_password(&mut conn, &password).map_err(|e| {
        error!("{}", e);
        return JournalOpeningError::InvalidPassword;
    })?;

    let mut master_key_handle = state.serivce_keys.lock();
    *master_key_handle = Some(service_keys);
    password.zeroize();

    Ok(())
}
