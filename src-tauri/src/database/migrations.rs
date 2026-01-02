use rusqlite::Connection;

const INIT_SQL: &str = include_str!("001_init.sql");

pub fn run_migrations(conn: &Connection) -> rusqlite::Result<()> {
    let version: i32 = conn
        .query_row(
            "SELECT value FROM metadata WHERE key = 'schema_version'",
            [],
            |row| row.get::<_, String>(0).map(|v| v.parse().unwrap_or(0)),
        )
        .unwrap_or(0);

    match version {
        0 => {
            conn.execute_batch(INIT_SQL)?;
        }
        1 => {}
        _ => panic!("Unsupported schema version"),
    }

    Ok(())
}
