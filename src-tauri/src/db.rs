// src-tauri/src/db.rs

use once_cell::sync::Lazy;
use rusqlite::{params, Connection, Result};
use std::sync::Mutex;

#[derive(Debug)]
pub struct DbUser {
    pub id: Option<i64>,
    pub email: String,
    pub password: String,
}

// Keep a global database connection
static DB: Lazy<Mutex<Option<Connection>>> = Lazy::new(|| Mutex::new(None));

pub fn init_db() -> Result<()> {
    let mut db_lock = DB.lock().unwrap();

    if db_lock.is_none() {
        let conn = Connection::open("users.db")?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                email TEXT UNIQUE NOT NULL,
                password TEXT NOT NULL
            )",
            [],
        )?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS session (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                email TEXT
            )",
            [],
        )?;
        *db_lock = Some(conn);
    }
    Ok(())
}

pub fn create_user(email: &str, password: &str) -> Result<()> {
    let db_lock = DB.lock().unwrap();
    let conn = db_lock.as_ref().unwrap();
    conn.execute(
        "INSERT INTO users (email, password) VALUES (?1, ?2)",
        params![email, password],
    )?;
    Ok(())
}

pub fn get_user(email: &str) -> Result<Option<DbUser>> {
    let db_lock = DB.lock().unwrap();
    let conn = db_lock.as_ref().unwrap();

    let mut stmt = conn.prepare("SELECT id, email, password FROM users WHERE email = ?1")?;
    let mut rows = stmt.query(params![email])?;

    if let Some(row) = rows.next()? {
        Ok(Some(DbUser {
            id: row.get(0)?,
            email: row.get(1)?,
            password: row.get(2)?,
        }))
    } else {
        Ok(None)
    }
}

pub fn set_current_user(email: &str) -> Result<()> {
    let db_lock = DB.lock().unwrap();
    let conn = db_lock.as_ref().unwrap();

    conn.execute("DELETE FROM session", [])?;
    conn.execute("INSERT INTO session (email) VALUES (?1)", params![email])?;
    Ok(())
}

pub fn get_current_user() -> Result<Option<String>> {
    let db_lock = DB.lock().unwrap();
    let conn = db_lock.as_ref().unwrap();

    let mut stmt = conn.prepare("SELECT email FROM session LIMIT 1")?;
    let mut rows = stmt.query([])?;

    if let Some(row) = rows.next()? {
        Ok(Some(row.get(0)?))
    } else {
        Ok(None)
    }
}

pub fn clear_current_user() -> Result<()> {
    let db_lock = DB.lock().unwrap();
    let conn = db_lock.as_ref().unwrap();
    conn.execute("DELETE FROM session", [])?;
    Ok(())
}
