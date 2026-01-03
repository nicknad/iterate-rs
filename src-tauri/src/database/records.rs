use crate::error::IterateError;
use chrono::Utc;
use rusqlite::{Connection, Row, params};
use tracing::error;
use uuid::Uuid;

pub struct RecordRow {
    pub id: Uuid,
    pub encrypted_content: Vec<u8>,
    pub sentiment_score: Option<f32>,
    pub is_summarized: bool,
    pub is_summary_record: bool,
    pub is_archived: bool,
    pub is_deleted: bool,
    pub created_at_utc: i64,
    pub last_modified_at_utc: i64,
    pub deleted_at_utc: Option<i64>,
}

pub struct RecordRepository<'a> {
    conn: &'a Connection,
}

impl<'a> RecordRepository<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn insert(
        &self,
        id: &Uuid,
        content: &[u8],
        sentiment: Option<f32>,
    ) -> Result<(), IterateError> {
        let now = Utc::now().timestamp();

        self.conn.execute(
            "INSERT INTO record (
                record_id, encrypted_content, sentiment_score, 
                is_summarized, is_summary_record, is_archived, is_deleted,
                created_at_utc, last_modified_at_utc
            ) VALUES (?, ?, ?, 0, 0, 0, 0, ?, ?)",
            params![id.as_bytes(), content, sentiment, now, now],
        )?;

        Ok(())
    }

    pub fn update(&self, id: &Uuid, content: &[u8]) -> Result<(), IterateError> {
        let now = Utc::now().timestamp();

        self.conn.execute(
            "UPDATE record SET 
                encrypted_content = ?, 
                last_modified_at_utc = ? 
            WHERE record_id = ?",
            params![content, now, id.as_bytes()],
        )?;

        Ok(())
    }

    pub fn get_record(&self, id: Uuid) -> Result<RecordRow, IterateError> {
        self.conn
            .query_row(
                "SELECT * FROM record WHERE record_id = ?",
                params![id.as_bytes()],
                |row| {
                    Ok(RecordRow {
                        id,
                        encrypted_content: row.get("encrypted_content")?,
                        sentiment_score: row.get("sentiment_score")?,
                        is_summarized: row.get::<_, i32>("is_summarized")? != 0,
                        is_summary_record: row.get::<_, i32>("is_summary_record")? != 0,
                        is_archived: row.get::<_, i32>("is_archived")? != 0,
                        is_deleted: row.get::<_, i32>("is_deleted")? != 0,
                        created_at_utc: row.get("created_at_utc")?,
                        last_modified_at_utc: row.get("last_modified_at_utc")?,
                        deleted_at_utc: row.get("deleted_at_utc")?,
                    })
                },
            )
            .map_err(|e| {
                error!("DB entry missing: {}", e);
                return IterateError::RecordNotFound;
            })
    }

    pub fn fetch_latest(&self, limit: usize) -> Result<Vec<RecordRow>, IterateError> {
        let mut stmt = self.conn.prepare(
            "SELECT * FROM record 
             WHERE is_deleted = 0 
             ORDER BY created_at_utc DESC 
             LIMIT ?",
        )?;
        let safe_limit = std::cmp::min(limit, 1000) as i64;
        let rows = stmt.query_map([safe_limit], |row| {
            Ok(RecordRow {
                id: Uuid::from_slice(&row.get::<_, Vec<u8>>("record_id")?).unwrap_or_default(),
                encrypted_content: row.get("encrypted_content")?,
                sentiment_score: row.get("sentiment_score")?,
                is_summarized: row.get::<_, i32>("is_summarized")? != 0,
                is_summary_record: row.get::<_, i32>("is_summary_record")? != 0,
                is_archived: row.get::<_, i32>("is_archived")? != 0,
                is_deleted: row.get::<_, i32>("is_deleted")? != 0,
                created_at_utc: row.get("created_at_utc")?,
                last_modified_at_utc: row.get("last_modified_at_utc")?,
                deleted_at_utc: row.get("deleted_at_utc")?,
            })
        })?;

        let mut results = Vec::with_capacity(limit);
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }
}
