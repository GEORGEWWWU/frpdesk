#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serde::Serialize;
use std::fs;
use std::io::{BufRead, BufReader};
use std::process::{Child, Command, Stdio};
use std::sync::Mutex;
use std::time::Instant; // 用于计算真实运行时长
use tauri::{AppHandle, Emitter, State};

struct AppState {
    frpc_process: Mutex<Option<Child>>,
    start_time: Mutex<Option<Instant>>, // 记录真实启动时间
}

#[derive(Serialize)]
struct FrpStats {
    uptime: String, // 仅保留运行时长
}

// 前端定时调用的真实数据接口
#[tauri::command]
fn get_frp_stats(state: State<AppState>) -> Result<FrpStats, String> {
    let start_time_guard = state.start_time.lock().unwrap();

    // 1. 利用 Rust 计算真实的运行时长
    let uptime = if let Some(start) = *start_time_guard {
        let elapsed = start.elapsed().as_secs();
        let hours = elapsed / 3600;
        let minutes = (elapsed % 3600) / 60;
        let seconds = elapsed % 60;
        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    } else {
        "00:00:00".to_string()
    };

    Ok(FrpStats { uptime })
}

#[tauri::command]
fn start_frp(
    state: State<AppState>,
    app_handle: AppHandle,
    exec_path: String,
    config_path: String,
) -> Result<String, String> {
    let mut process_guard = state.frpc_process.lock().unwrap();
    if process_guard.is_some() {
        return Ok("服务已经在运行".to_string());
    }

    match Command::new(&exec_path)
        .arg("-c")
        .arg(&config_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(mut child) => {
            // 开辟新线程读取 stdout 并发送给前端
            if let Some(stdout) = child.stdout.take() {
                let app_clone = app_handle.clone();
                std::thread::spawn(move || {
                    let reader = BufReader::new(stdout);
                    for line in reader.lines().flatten() {
                        let _ = app_clone.emit("frpc-log", line);
                    }
                });
            }

            if let Some(stderr) = child.stderr.take() {
                let app_clone = app_handle.clone();
                std::thread::spawn(move || {
                    let reader = BufReader::new(stderr);
                    for line in reader.lines().flatten() {
                        let _ = app_clone.emit("frpc-log", line);
                    }
                });
            }

            *process_guard = Some(child);

            // 记录启动时间
            let mut time_guard = state.start_time.lock().unwrap();
            *time_guard = Some(Instant::now());

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

        // 清空启动时间
        let mut time_guard = state.start_time.lock().unwrap();
        *time_guard = None;

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
            start_time: Mutex::new(None), // 注册新状态
        })
        .invoke_handler(tauri::generate_handler![
            start_frp,
            stop_frp,
            read_config,
            save_config,
            get_frp_stats // 注册新指令
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
