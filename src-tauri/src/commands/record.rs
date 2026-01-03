use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use tracing::error;
use uuid::Uuid;

use crate::{
    crypto::aead::encrypt,
    database::{
        open_database,
        RecordRepository,
    },
    state::AppState,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct JournalEntry {
    id: Option<Uuid>,
    text: String,
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "message")]
pub enum SaveRecordError {
    InvalidState,
    InternalError(String),
    EncryptionFailure,
    DatabaseFailure(String),
}

#[tauri::command]
pub async fn save_journal_entry(
    mut journal_entry: JournalEntry,
    state: tauri::State<'_, AppState>,
) -> Result<JournalEntry, SaveRecordError> {
    let db_path = {
        let guard = state.db_path.lock();
        guard
            .as_ref()
            .cloned()
            .ok_or(SaveRecordError::InvalidState)?
    };

    let conn = open_database(db_path)
        .map_err(|e| SaveRecordError::DatabaseFailure(format!("Failed to open DB: {}", e)))?;

    let keys_guard = state.serivce_keys.lock();
    let keys = keys_guard.as_ref().ok_or(SaveRecordError::InvalidState)?;

    let (record_id, is_new) = match journal_entry.id {
        Some(existing_id) => (existing_id, false),
        None => {
            let new_id = Uuid::now_v7();
            journal_entry.id = Some(new_id);
            (new_id, true)
        }
    };

    let envelope = encrypt(
        &mut OsRng,
        &keys.content,
        journal_entry.text.as_bytes(),
        record_id.as_bytes(),
    )
    .map_err(|_| SaveRecordError::EncryptionFailure)?;

    let encrypted_blob = envelope.to_blob().map_err(|e| {
        error!("Postcard-Serialization failed: {:?}", e);

        return SaveRecordError::InternalError("Unexpected error occured".to_string());
    })?;

    let record_repository = RecordRepository::new(&conn);

    if is_new {
        record_repository.insert(&record_id, &encrypted_blob, None)
            .map_err(|e| {
                error!("SQL Insert failed: {:?}", e);
                return SaveRecordError::DatabaseFailure("Record was not saved".to_string());
            })?;
    } else {
        record_repository.update(&record_id, &encrypted_blob).map_err(|e| {
            error!("SQL Update failed: {:?}", e);
            return SaveRecordError::DatabaseFailure("Record was not saved".to_string());
        })?;
    }

    Ok(journal_entry)
}
