#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(deprecated)] // 临时处理generate_context报warning的问题 issues/12382

mod libs;
mod modules;

use libs::cache::create_cache_directory;
use libs::commands::{get_config, get_version, open_log_directory, save_config, start};
use libs::create_config::create_config_file;

use tauri::{
    Manager, 
    menu::{Menu, MenuItem},
    tray::{TrayIconBuilder},
    WindowEvent,
};

const VERSION: &str = "0.0.2";

pub fn main() {
    // 初始化缓存目录和配置文件
    create_cache_directory().expect("Failed to create cache directory");
    create_config_file().expect("Failed to create config.yml");

    tauri::Builder::default()
        .setup(|app| {
            let main_window = app.get_webview_window("main").unwrap();
            let main_window_clone = main_window.clone();
            main_window.on_window_event(move |event| match event {
                WindowEvent::CloseRequested { api, .. } => {
                    api.prevent_close(); // 阻止关闭事件
                    main_window_clone.hide().unwrap(); // 隐藏窗口
                }
                _ => {}
            });
            
            // 创建托盘菜单
            let tray_menu = Menu::with_items(app, &[
                &MenuItem::with_id(app, "show", "显示窗口", true, None::<&str>)?,
                &MenuItem::with_id(app, "hide", "隐藏窗口", true, None::<&str>)?,
                &MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?,
            ])?;
            // 添加托盘图标及其事件逻辑
            TrayIconBuilder::new()
            .icon(app.default_window_icon().unwrap().clone())
            .menu(&tray_menu)
            .tooltip("KizunaBaka")
            .on_menu_event(|app, event| match event.id.0.as_str() {
                "show" => {
                    let main_window = app.get_webview_window("main").unwrap();
                    main_window.show().unwrap();
                    main_window.set_focus().unwrap();
                }
                "hide" => {
                    let main_window = app.get_webview_window("main").unwrap();
                    main_window.hide().unwrap();
                }
                "quit" => {
                    app.exit(0); // 退出应用程序
                }
                _ => {}
            })
            .build(app)?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            start,
            open_log_directory,
            get_version,
            save_config,
            get_config
        ])
        .run(tauri::generate_context!())
        .expect("Error while running Tauri application");
}