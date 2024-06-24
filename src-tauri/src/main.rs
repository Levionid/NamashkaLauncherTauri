// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;
use tauri::{generate_handler, Builder};
use std::fs;

#[tauri::command]
fn open_explorer() {
    if cfg!(target_os = "windows") {
        Command::new("explorer.exe")
            .arg(".")
            .spawn()
            .expect("Failed to open explorer");
    }
}

#[tauri::command]
fn start_minecraft_loader() -> Result<(), String> {
    // Check if minecraftLoader.exe exists
    let loader_exists = fs::metadata("minecraftLoader.exe").is_ok();

    if loader_exists {
        Command::new("minecraftLoader.exe")
            .spawn()
            .map_err(|e| format!("Failed to start Minecraft Loader: {}", e))?;
        Ok(())
    } else {
        Err("minecraft loader отсутствует".to_string())
    }
}

#[tauri::command]
fn check_launcher_options() -> bool {
    fs::metadata("launcherOptions.json").is_ok()
}

fn main() {
    Builder::default()
        .invoke_handler(generate_handler![open_explorer, start_minecraft_loader, check_launcher_options])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
