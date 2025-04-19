use rusqlite::{params, Connection};
use std::sync::{Arc, Mutex};
pub fn create_tables(conn: &Arc<Mutex<Connection>>) -> Result<(), rusqlite::Error> {
    let db = conn.lock().expect("DB lock failed");
    db.execute(
        "CREATE TABLE IF NOT EXISTS links (
            id INTEGER PRIMARY KEY,
            url TEXT NOT NULL UNIQUE,
            status_code INTEGER,
            has_form TEXT
        )",
        params![],
    )?;
    Ok(())
}
