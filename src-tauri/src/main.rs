// src-tauri/src/main.rs

use rusqlite::Result;
use serde::Serialize;
use tauri::command;

mod db;

#[derive(Serialize)]
struct User {
    id: Option<i64>,
    email: String,
    password: String,
}

#[command]
fn create_user_cmd(email: String, password: String) -> Result<(), String> {
    db::create_user(&email, &password).map_err(|e| e.to_string())
}

#[command]
fn get_user_cmd(email: String) -> Result<Option<User>, String> {
    match db::get_user(&email) {
        Ok(Some(u)) => Ok(Some(User {
            id: u.id,
            email: u.email,
            password: u.password,
        })),
        Ok(None) => Ok(None),
        Err(e) => Err(e.to_string()),
    }
}

#[command]
fn set_current_user_cmd(email: String) -> Result<(), String> {
    db::set_current_user(&email).map_err(|e| e.to_string())
}

#[command]
fn get_current_user_cmd() -> Result<Option<String>, String> {
    db::get_current_user().map_err(|e| e.to_string())
}

#[command]
fn clear_current_user_cmd() -> Result<(), String> {
    db::clear_current_user().map_err(|e| e.to_string())
}

fn main() {
    // Initialize database on startup
    if let Err(e) = db::init_db() {
        eprintln!("❌ Failed to initialize database: {}", e);
    }

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            create_user_cmd,
            get_user_cmd,
            set_current_user_cmd,
            get_current_user_cmd,
            clear_current_user_cmd,
        ])
        .run(tauri::generate_context!())
        .expect("❌ error while running Tauri application");
}
