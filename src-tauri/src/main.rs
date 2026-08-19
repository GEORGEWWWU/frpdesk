#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::fs;
use std::process::{Child, Command};
use std::sync::Mutex;
use tauri::State;

// 用于全局存储 frpc 进程状态
struct AppState {
    frpc_process: Mutex<Option<Child>>,
}

#[tauri::command]
fn start_frp(
    state: State<AppState>,
    exec_path: String,
    config_path: String,
) -> Result<String, String> {
    let mut process_guard = state.frpc_process.lock().unwrap();
    if process_guard.is_some() {
        return Ok("服务已经在运行".to_string());
    }

    // 对应批处理命令: frpc -c frpc.toml[cite: 4]
    match Command::new(&exec_path).arg("-c").arg(&config_path).spawn() {
        Ok(child) => {
            *process_guard = Some(child);
            Ok("FRP 已启动".to_string())
        }
        Err(e) => Err(format!("启动失败: {}", e)),
    }
}

#[tauri::command]
fn stop_frp(state: State<AppState>) -> Result<String, String> {
    let mut process_guard = state.frpc_process.lock().unwrap();
    if let Some(mut child) = process_guard.take() {
        let _ = child.kill();
        let _ = child.wait();
        Ok("FRP 已停止".to_string())
    } else {
        Ok("服务未运行".to_string())
    }
}

#[tauri::command]
fn read_config(path: String) -> Result<String, String> {
    fs::read_to_string(path).map_err(|e| e.to_string())
}

#[tauri::command]
fn save_config(path: String, content: String) -> Result<(), String> {
    fs::write(path, content).map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .manage(AppState {
            frpc_process: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![
            start_frp,
            stop_frp,
            read_config,
            save_config
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
