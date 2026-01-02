use crate::error::IterateError;
use chrono::{Duration, Utc};
use rusqlite::{Connection, params};
use tracing::{error, info};

pub fn purge_old_deleted_records(
    conn: &Connection,
    retention_days: u64,
) -> Result<usize, IterateError> {
    let cutoff_time = Utc::now()
        .checked_sub_signed(Duration::days(retention_days as i64))
        .ok_or_else(|| IterateError::Internal("Time calculation overflow".into()))?
        .timestamp();

    let deleted_count = conn.execute(
        "DELETE FROM record 
             WHERE is_deleted = 1 
             AND deleted_at_utc < ?",
        params![cutoff_time],
    )?;

    if deleted_count > 0 {
        info!(
            "Maintenance: Purged {} old soft-deleted records.",
            deleted_count
        );
    }

    Ok(deleted_count)
}
