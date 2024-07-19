use tauri::{AppHandle, Manager, Wry};

#[tauri::command]
pub fn start(app_handle: AppHandle<Wry>) {
    let config = crate::modules::get_config::load_config();
    let endpoint = config.server_config.endpoint.clone();
    let token = config.server_config.token.clone();
    let report_time = std::time::Duration::from_secs(config.server_config.report_time as u64);

    std::thread::spawn(move || loop {
        std::thread::sleep(report_time);
        let (logdata, data, icon_base64, _media_update) = crate::report(&endpoint, &token);
        let home_event_data = serde_json::json!({
            "data": data,
            "icon": icon_base64,
        });
        app_handle
            .emit_all("home-event", home_event_data)
            .unwrap_or_else(|e| {
                eprintln!("Failed to emit home-event: {}", e);
            });
        app_handle
            .emit_all("log-event", logdata)
            .unwrap_or_else(|e| {
                eprintln!("Failed to emit log-event: {}", e);
            });
    });
}

#[tauri::command]
pub fn open_log_directory() {
    #[cfg(target_os = "windows")]
    {
        if let Err(e) = std::process::Command::new("explorer")
            .arg(".\\logs\\")
            .spawn()
        {
            eprintln!("Failed to open log directory: {}", e);
        }
    }
}

#[tauri::command]
pub fn get_version() -> String {
    crate::VERSION.to_string()
}
