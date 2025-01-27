pub fn get_window_info() -> (String, String, String) {
    #[cfg(target_os = "windows")]
    if cfg!(target_os = "windows") {
        return crate::modules::get_processes_win::get_os_windows_info();
    }

    #[cfg(target_os = "macos")]
    if cfg!(target_os = "macos") {
        match crate::modules::get_processes_macos::get_os_macos_info() {
            Ok((process_name, _bundle_id, _path, window_title, icon_base64)) => {
                return (process_name, window_title, icon_base64);
            }
            Err(_) => {
                return (String::new(), String::new(), String::new());
            }
        }
    }

    #[cfg(target_os = "linux")] // todo：linux 平台暂时不支持
    if cfg!(target_os = "linux") {
        return (String::new(), String::new(), String::new());
    } 
        
    (String::new(), String::new(), String::new())
}



pub fn replacer(process_name: &str) -> String {
    let cfg = crate::modules::get_config::load_config();
    for rule in cfg.rules {
        if process_name == rule.match_application {
            return rule.replace.application.clone();
        }
    }
    process_name.to_string()
}