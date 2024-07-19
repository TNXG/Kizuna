use serde_json::Value;
use std::collections::HashMap;

pub fn report(
    endpoint: &str,
    token: &str,
) -> (
    String,
    HashMap<String, Value>,
    Option<String>,
    HashMap<String, String>,
) {
    let (process_name, window_name) = crate::modules::get_processes::get_window_info();

    let process_name = crate::modules::get_processes::replacer(&process_name.replace(".exe", ""));
    let icon = crate::modules::get_processes::get_window_icon(&window_name);
    let (title, artist, source_app_name) = crate::modules::get_smtc::get_media_info();
    let media_update =
        crate::modules::requests::build_media_update(&title, &artist, &source_app_name);
    let mut update_data =
        crate::modules::requests::build_data(&process_name, media_update.clone(), token);
    let logdata = crate::modules::logs::log_message(
        "info",
        crate::modules::requests::report(update_data.clone(), endpoint).as_str(),
    );
    update_data.remove("key");
    update_data.insert(
        "window_name".to_string(),
        serde_json::Value::String(window_name.trim_end_matches('\u{0000}').to_string()),
    );
    if let Err(e) = crate::modules::icon_converter::convert_hicon_to_png(icon, "cache/icon.png") {
        eprintln!("Failed to convert icon to PNG: {}", e);
    }
    let icon_base64 = crate::modules::icon_converter::convert_png_to_base64("cache/icon.png");
    (logdata, update_data, icon_base64, media_update)
}
