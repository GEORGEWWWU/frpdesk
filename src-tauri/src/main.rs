#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::fs;
use std::io::{BufRead, BufReader}; // 新增：用于读取缓冲日志
use std::process::{Child, Command, Stdio}; // 新增：引入 Stdio 处理管道
use std::sync::Mutex;
use tauri::{AppHandle, Emitter, State}; // 新增：引入 AppHandle 和 Emitter 用于发送事件

struct AppState {
    frpc_process: Mutex<Option<Child>>,
}

#[tauri::command]
fn start_frp(
    state: State<AppState>,
    app_handle: AppHandle, // 新增注入 AppHandle
    exec_path: String,
    config_path: String,
) -> Result<String, String> {
    let mut process_guard = state.frpc_process.lock().unwrap();
    if process_guard.is_some() {
        return Ok("服务已经在运行".to_string());
    }

    // 配置管道以捕获输出
    match Command::new(&exec_path)
        .arg("-c")
        .arg(&config_path)
        .stdout(Stdio::piped()) // 捕获标准输出
        .stderr(Stdio::piped()) // 捕获标准错误
        .spawn()
    {
        Ok(mut child) => {
            // 开辟新线程读取 stdout 并发送给前端
            if let Some(stdout) = child.stdout.take() {
                let app_clone = app_handle.clone();
                std::thread::spawn(move || {
                    let reader = BufReader::new(stdout);
                    for line in reader.lines().flatten() {
                        let _ = app_clone.emit("frpc-log", line); // 触发事件
                    }
                });
            }

            // 开辟新线程读取 stderr 并发送给前端
            if let Some(stderr) = child.stderr.take() {
                let app_clone = app_handle.clone();
                std::thread::spawn(move || {
                    let reader = BufReader::new(stderr);
                    for line in reader.lines().flatten() {
                        let _ = app_clone.emit("frpc-log", line); // 触发事件
                    }
                });
            }

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
        .plugin(tauri_plugin_dialog::init())
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
