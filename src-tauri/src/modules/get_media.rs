pub fn get_media_info() -> (String, String, String, String, String, String) {
    #[cfg(target_os = "windows")]
    if cfg!(target_os = "windows") {
        return crate::modules::get_processes_win::get_os_windows_info();
    }
    
    return (String::new(), String::new(), String::new(), String::new(), String::new(), String::new());
}