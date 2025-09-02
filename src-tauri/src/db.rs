use once_cell::sync::Lazy;
use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Debug)]
pub struct DbUser {
    pub id: Option<i64>,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DbProduct {
    pub id: Option<i64>,
    pub name: String,
    pub price: f64,
    pub description: String,
    pub category: String,
    pub user_email: String,
}

// Keep a global database connection
static DB: Lazy<Mutex<Option<Connection>>> = Lazy::new(|| Mutex::new(None));

pub fn init_db() -> Result<()> {
    let mut db_lock = DB.lock().unwrap();

    if db_lock.is_none() {
        let conn = Connection::open("users.db")?;

        // Users table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                email TEXT UNIQUE NOT NULL,
                password TEXT NOT NULL
            )",
            [],
        )?;

        // Session table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS session (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                email TEXT
            )",
            [],
        )?;

        // Products table with user_email column
        conn.execute(
            "CREATE TABLE IF NOT EXISTS products (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                price REAL NOT NULL,
                description TEXT,
                category TEXT,
                user_email TEXT NOT NULL
            )",
            [],
        )?;

        *db_lock = Some(conn);
    }
    Ok(())
}

//
// --- USERS ---
//
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

//
// --- PRODUCTS ---
//
pub fn add_product(
    name: &str,
    price: f64,
    description: &str,
    category: &str,
    user_email: &str,
) -> Result<()> {
    let db_lock = DB.lock().unwrap();
    let conn = db_lock.as_ref().unwrap();
    conn.execute(
        "INSERT INTO products (name, price, description, category, user_email) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![name, price, description, category, user_email],
    )?;
    Ok(())
}

pub fn get_products() -> Result<Vec<DbProduct>> {
    let db_lock = DB.lock().unwrap();
    let conn = db_lock.as_ref().unwrap();

    let mut stmt =
        conn.prepare("SELECT id, name, price, description, category, user_email FROM products")?;
    let rows = stmt.query_map([], |row| {
        Ok(DbProduct {
            id: row.get(0)?,
            name: row.get(1)?,
            price: row.get(2)?,
            description: row.get(3)?,
            category: row.get(4)?,
            user_email: row.get(5)?,
        })
    })?;

    let mut products = Vec::new();
    for product in rows {
        products.push(product?);
    }
    Ok(products)
}

pub fn get_user_products(user_email: &str) -> Result<Vec<DbProduct>> {
    let db_lock = DB.lock().unwrap();
    let conn = db_lock.as_ref().unwrap();

    let mut stmt = conn.prepare("SELECT id, name, price, description, category, user_email FROM products WHERE user_email = ?1")?;
    let rows = stmt.query_map(params![user_email], |row| {
        Ok(DbProduct {
            id: row.get(0)?,
            name: row.get(1)?,
            price: row.get(2)?,
            description: row.get(3)?,
            category: row.get(4)?,
            user_email: row.get(5)?,
        })
    })?;

    let mut products = Vec::new();
    for product in rows {
        products.push(product?);
    }
    Ok(products)
}

pub fn update_product(
    id: i64,
    name: &str,
    price: f64,
    description: &str,
    category: &str,
    user_email: &str,
) -> Result<()> {
    let db_lock = DB.lock().unwrap();
    let conn = db_lock.as_ref().unwrap();
    conn.execute(
        "UPDATE products SET name = ?1, price = ?2, description = ?3, category = ?4 WHERE id = ?5 AND user_email = ?6",
        params![name, price, description, category, id, user_email],
    )?;
    Ok(())
}

pub fn delete_product(id: i64, user_email: &str) -> Result<()> {
    let db_lock = DB.lock().unwrap();
    let conn = db_lock.as_ref().unwrap();
    conn.execute(
        "DELETE FROM products WHERE id = ?1 AND user_email = ?2",
        params![id, user_email],
    )?;
    Ok(())
}
