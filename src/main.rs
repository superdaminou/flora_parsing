// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::str::FromStr;

use dotenv::dotenv;
use error::ParsingError;
use log::info;
use parser::{parse, Mode};
mod error;
mod parser;

fn main() {
    dotenv().ok();
    env_logger::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![execute])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn execute(content: &str, mode: &str) -> Result<String, ParsingError> {
    info!("Start parsing");
    Mode::from_str(mode)
        .map_err(|e|ParsingError::DefaultError("Could not parse mode".to_string()))
        .and_then(|mode| parse(content, mode))
}
