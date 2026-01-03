use rusqlite::{Connection, Result};

use crate::database::migrations::run_migrations;
use crate::error::IterateError;

pub fn open_database(path: std::path::PathBuf) -> Result<Connection, IterateError> {
    let conn = Connection::open(path)?;
    run_migrations(&conn)?;
    let ok: String = conn.query_row("PRAGMA integrity_check;", [], |row| row.get(0))?;

    if ok != "ok" {
        return Err(IterateError::DatabaseIntegrity);
    }

    apply_pragmas(&conn)?;

    Ok(conn)
}

fn apply_pragmas(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        r#"
        PRAGMA journal_mode = WAL;
        PRAGMA synchronous = NORMAL;
        PRAGMA temp_store = MEMORY;
        PRAGMA secure_delete = ON;
        PRAGMA foreign_keys = ON;
        PRAGMA trusted_schema = OFF;
        PRAGMA busy_timeout = 5000;
        "#,
    )?;
    Ok(())
}
