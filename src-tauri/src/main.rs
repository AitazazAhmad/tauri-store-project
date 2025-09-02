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

#[derive(Serialize)]
struct Product {
    id: Option<i64>,
    name: String,
    price: f64,
    description: String,
    category: String,
}

//
// --- USER COMMANDS ---
//
#[command]
fn create_user(email: String, password: String) -> Result<(), String> {
    db::create_user(&email, &password).map_err(|e| e.to_string())
}

#[command]
fn get_user(email: String) -> Result<Option<User>, String> {
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
fn set_current_user(email: String) -> Result<(), String> {
    db::set_current_user(&email).map_err(|e| e.to_string())
}

#[command]
fn get_current_user() -> Result<Option<String>, String> {
    db::get_current_user().map_err(|e| e.to_string())
}

#[command]
fn clear_current_user() -> Result<(), String> {
    db::clear_current_user().map_err(|e| e.to_string())
}

//
// --- PRODUCT COMMANDS ---
//
#[command]
fn add_product(
    name: String,
    price: f64,
    description: String,
    category: String,
) -> Result<(), String> {
    db::add_product(&name, price, &description, &category).map_err(|e| e.to_string())
}

#[command]
fn get_products() -> Result<Vec<Product>, String> {
    match db::get_products() {
        Ok(list) => Ok(list
            .into_iter()
            .map(|p| Product {
                id: p.id,
                name: p.name,
                price: p.price,
                description: p.description,
                category: p.category,
            })
            .collect()),
        Err(e) => Err(e.to_string()),
    }
}

#[command]
fn update_product(
    id: i64,
    name: String,
    price: f64,
    description: String,
    category: String,
) -> Result<(), String> {
    db::update_product(id, &name, price, &description, &category).map_err(|e| e.to_string())
}

#[command]
fn delete_product(id: i64) -> Result<(), String> {
    db::delete_product(id).map_err(|e| e.to_string())
}

//
// --- MAIN ---
//
fn main() {
    // Initialize database on startup
    if let Err(e) = db::init_db() {
        eprintln!("❌ Failed to initialize database: {}", e);
    }

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            // User/session commands
            create_user,
            get_user,
            set_current_user,
            get_current_user,
            clear_current_user,
            // Product commands
            add_product,
            get_products,
            update_product,
            delete_product,
        ])
        .run(tauri::generate_context!())
        .expect("❌ error while running Tauri application");
}
