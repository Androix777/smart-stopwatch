// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use active_win_pos_rs::get_active_window;
use serde::Serialize;
use std::path::PathBuf;
use tauri::command;

#[derive(Serialize)]
struct WindowPosition {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

#[derive(Serialize)]
struct ActiveWindow {
    title: String,
    #[serde(with = "pathbuf_serde")]
    process_path: PathBuf,
    app_name: String,
    window_id: String,
    process_id: u64,
    position: WindowPosition,
}

mod pathbuf_serde {
    use serde::{self, Serializer};
    use std::path::PathBuf;

    pub fn serialize<S>(path_buf: &PathBuf, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(path_buf.to_str().unwrap_or(""))
    }
}

#[command]
fn get_window() -> Result<ActiveWindow, String> {
    match get_active_window() {
        Ok(active_window) => {
            let window = ActiveWindow {
                title: active_window.title,
                process_path: active_window.process_path,
                app_name: active_window.app_name,
                window_id: active_window.window_id,
                process_id: active_window.process_id,
                position: WindowPosition {
                    x: active_window.position.x,
                    y: active_window.position.y,
                    width: active_window.position.width,
                    height: active_window.position.height,
                },
            };
            Ok(window)
        },
        Err(_) => {
            Err("Error occurred while getting the active window.".to_string())
        }
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_window])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}