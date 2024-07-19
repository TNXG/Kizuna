mod libs;
mod modules;

use libs::cache::create_cache_directory;
use libs::commands::{get_version, open_log_directory, start};
use libs::create_config::create_config_file;
use libs::report::report;

use tauri::{
    CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
};

const VERSION: &str = "0.0.1";

pub fn main() {
    create_cache_directory().expect("Failed to create cache directory");
    create_config_file("config.yml").expect("Failed to create config.yml");
    let quit = CustomMenuItem::new("quit".to_string(), "退出");
    let show = CustomMenuItem::new("show".to_string(), "显示");
    let tray_menu = SystemTrayMenu::new()
        .add_item(show)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);
    let system_tray = SystemTray::new().with_menu(tray_menu);
    tauri::Builder::default()
        .setup(|app| {
            let main_window = app.get_window("main").unwrap();
            main_window.eval("navigator.language = 'zh-CN';").unwrap();
            let main_window_clone = main_window.clone();
            main_window.on_window_event(move |event| match event {
                tauri::WindowEvent::CloseRequested { api, .. } => {
                    api.prevent_close();
                    main_window_clone.hide().unwrap();
                }
                _ => {}
            });
            Ok(())
        })
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                "show" => {
                    let main_window = app.get_window("main").unwrap();
                    main_window.show().unwrap();
                    main_window.set_focus().unwrap();
                }
                _ => {}
            },
            SystemTrayEvent::LeftClick { .. } => {
                let main_window = app.get_window("main").unwrap();
                main_window.show().unwrap();
                main_window.set_focus().unwrap();
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            start,
            open_log_directory,
            get_version
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
