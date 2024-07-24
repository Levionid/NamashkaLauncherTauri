// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::{Command, Stdio};
use std::thread;
use tauri::{Manager, Window};
use tauri::{generate_handler, Builder};
use std::fs;
use sys_info;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use window_vibrancy::{apply_blur, apply_vibrancy, NSVisualEffectMaterial};
use window_shadows::set_shadow;

#[derive(Serialize, Deserialize)]
struct LauncherOptions {
    options: Options,
    token: String,
}

#[derive(Serialize, Deserialize)]
struct Options {
    jvmArguments: Vec<String>,
    username: String,
}

#[tauri::command]
fn read_launcher_options() -> Result<HashMap<String, String>, String> {
    let file_path = "./launcherOptions.txt";
    let contents = fs::read_to_string(file_path)
        .map_err(|e| format!("Failed to read {}: {}", file_path, e))?;

    let launcher_options: LauncherOptions = serde_json::from_str(&contents)
        .map_err(|e| format!("Failed to parse {}: {}", file_path, e))?;

    let mut options_map = HashMap::new();
    options_map.insert("username".to_string(), launcher_options.options.username);
    options_map.insert("token".to_string(), launcher_options.token);

    // Join the JVM arguments into a single string
    let jvmArguments = format!("{} {}", 
                                       &launcher_options.options.jvmArguments[0][4..launcher_options.options.jvmArguments[0].len()-1],
                                       &launcher_options.options.jvmArguments[1][4..launcher_options.options.jvmArguments[1].len()-1]);
    options_map.insert("jvmArguments".to_string(), jvmArguments);

    Ok(options_map)
}

#[tauri::command]
fn save_launcher_options(username: String, token: String, minJvmArgument: String, maxJvmArgument: String) -> Result<(), String> {
    let file_path = "./launcherOptions.txt";

    let options = Options {
        jvmArguments: vec![format!("-Xms{}M", minJvmArgument), format!("-Xmx{}M", maxJvmArgument)],
        username,
    };

    let launcher_options = LauncherOptions {
        options,
        token,
    };

    let json_data = serde_json::to_string_pretty(&launcher_options)
        .map_err(|e| format!("Failed to serialize data: {}", e))?;

    fs::write(file_path, json_data).map_err(|e| format!("Failed to write to file {}: {}", file_path, e))?;

    Ok(())
}

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
fn start_minecraft_loader(window: Window, mcpack_dir: String) -> Result<(), String> {
    // Check if minecraftLoader.exe exists
    let loader_exists = std::fs::metadata("minecraftLoader.exe").is_ok();

    if loader_exists {
        // Create the mcpack_dir directory if it doesn't exist
        std::fs::create_dir_all(&mcpack_dir).map_err(|e| format!("Failed to create directory {}: {}", mcpack_dir, e))?;

        // Hide the launcher window
        window.hide().map_err(|e| format!("Failed to hide window: {}", e))?;

        // Start the minecraftLoader.exe process in the specified mcpack_dir directory
        let mut child = Command::new("./minecraftLoader.exe")
            .current_dir(format!("./{}", mcpack_dir))
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|e| format!("Failed to start Minecraft Loader: {}", e))?;

        // Monitor the process in a new thread
        let app_handle = window.app_handle();
        thread::spawn(move || {
            let result = child.wait();
            if result.is_ok() {
                // Show the launcher window again after the process has finished
                app_handle
                    .get_window("main")
                    .unwrap()
                    .show()
                    .expect("Failed to show window");
            }
        });

        Ok(())
    } else {
        Err("minecraft loader отсутствует".to_string())
    }
}

#[tauri::command]
fn check_launcher_options() -> bool {
    fs::metadata("./launcherOptions.txt").is_ok()
}

#[tauri::command]
fn get_ram_size() -> Result<u64, String> {
    match sys_info::mem_info() {
        Ok(mem_info) => Ok(mem_info.total / 1024), // Преобразование из килобайтов в мегабайты
        Err(e) => Err(format!("Failed to get RAM size: {}", e)),
    }
}

fn main() {
    Builder::default()
        /*.setup(|app| {
            let window = app.get_window("main").unwrap();
            
            #[cfg(any(windows, target_os = "macos"))]
            set_shadow(&window, true).unwrap();
            
            #[cfg(target_os = "windows")]
            apply_blur(&window, Some((42, 47, 50, 200)))
                .expect("Unsupported platform! 'apply_blur' is only supported on Windows");
            Ok(())
        })*/
        .invoke_handler(generate_handler![open_explorer,
             start_minecraft_loader,
             check_launcher_options,
             read_launcher_options,
             save_launcher_options,
             get_ram_size])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}