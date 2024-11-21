use rusqlite::{params, Connection, Result};

pub fn get_theme(conn: &Connection) -> Result<String> {
    conn.query_row(
        "SELECT value FROM settings WHERE key = ?1",
        params!["theme"],
        |row| row.get(0),
    )
}

pub fn set_theme(conn: &Connection, theme: &str) -> Result<()> {
    conn.execute(
        "UPDATE settings SET value = ?1 WHERE key = ?2",
        params![theme, "theme"],
    )?;
    Ok(())
}