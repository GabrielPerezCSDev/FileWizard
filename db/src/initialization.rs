use rusqlite::{Connection, Result};
use std::path::Path;
use utils::get_base_path;

pub fn initialize_database() -> rusqlite::Result<()> {
    let db_path = get_base_path().join("db").join("file_wizard.db");

    // Ensure the `db/` directory exists
    let db_dir = db_path.parent().unwrap();
    if !db_dir.exists() {
        std::fs::create_dir_all(db_dir); // Create the directory if it doesn't exist
    }

    if db_path.exists() {
        println!("Database already exists. Skipping initialization.");
        return Ok(());
    }

    let conn = rusqlite::Connection::open(&db_path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS settings (
            id INTEGER PRIMARY KEY,
            key TEXT NOT NULL UNIQUE,
            value TEXT NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "INSERT OR IGNORE INTO settings (key, value) VALUES (?1, ?2)",
        rusqlite::params!["theme", "light"],
    )?;

    println!("Database initialized at: {}", db_path.display());
    Ok(())
}