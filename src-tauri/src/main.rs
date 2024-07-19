// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod modules;

use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::thread;
use std::time::Duration;

use modules::get_config::load_config;
use modules::get_processes;
use modules::get_smtc::get_media_info;
use modules::icon_converter;
use modules::logs;
use modules::requests;

use tauri::{Manager, Wry};

const VERSION: &str = "0.0.4";

pub fn main() {
    if !fs::metadata(".\\cache\\").is_ok() {
        fs::create_dir(".\\cache.\\").expect("Failed to create assets directory");
    }

    let config_file = "config.yml";
    if !fs::metadata(config_file).is_ok() {
        let mut file = fs::File::create(config_file).expect("Failed to create config.yml");
        file.write_all(DEFAULT_CONFIG.as_bytes())
            .expect("Failed to write to config.yml");
    }

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            start,
            open_log_directory,
            get_version
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn start(app_handle: tauri::AppHandle<Wry>) {
    let config = load_config();
    let endpoint = config.server_config.endpoint.to_string();
    let token = config.server_config.token.to_string();
    let report_time = Duration::from_secs(config.server_config.report_time as u64);
    thread::spawn(move || {
        let endpoint = endpoint.to_owned();
        let token = token.to_owned();
        loop {
            thread::sleep(report_time);
            let (logdata, data, icon_base64, media_update) = report(&endpoint, &token);
            let home_event_data = serde_json::json!({
                "data": data,
                "icon": icon_base64,
            });
            app_handle.emit_all("home-event", home_event_data).unwrap();
            app_handle.emit_all("log-event", logdata).unwrap();
        }
    });
}
#[tauri::command]
fn open_log_directory() {
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
            .arg(".\\logs\\")
            .spawn()
            .unwrap();
    }
}

#[tauri::command]
fn get_version() -> String {
    return VERSION.to_string();
}

fn report(
    endpoint: &str,
    token: &str,
) -> (
    String,
    HashMap<String, Value>,
    Option<String>,
    HashMap<String, String>,
) {
    let (process_name, window_name) = get_processes::get_window_info();

    let process_name = get_processes::replacer(&process_name.replace(".exe", ""));
    let icon = get_processes::get_window_icon(&window_name);
    let (title, artist, source_app_name) = get_media_info();
    let media_update = requests::build_media_update(&title, &artist, &source_app_name);
    let mut update_data = requests::build_data(&process_name, media_update.clone(), token);
    let logdata = logs::log_message(
        "info",
        requests::report(update_data.clone(), endpoint).as_str(),
    );
    update_data.remove("key");
    // 添加 window_name
    update_data.insert(
        "window_name".to_string(),
        serde_json::Value::String(window_name.trim_end_matches('\u{0000}').to_string()),
    );
    let _ = icon_converter::convert_hicon_to_png(icon, "cache/icon.png");
    let icon_base64 = icon_converter::convert_png_to_base64("cache/icon.png");
    (logdata, update_data, icon_base64, media_update)
}

const DEFAULT_CONFIG: &str = r#"
ServerConfig:
  Endpoint: "apiurl" # https://api.example.com/api/v2/fn/ps/update
  Token: "apikey" # 设置的key
  ReportTime: "10" # 上报时间间隔，单位秒
Rules: # 软件名的替换规则（
  - MatchApplication: WeChat
    Replace:
      Application: 微信
      Description: 一个小而美的办公软件
  - MatchApplication: QQ
    Replace:
      Application: QQ
      Description: 一个多功能的通讯软件
  - MatchApplication: Netease Cloud Music
    Replace:
      Application: 网易云音乐
      Description: 一个音乐播放和分享的平台

"#;
