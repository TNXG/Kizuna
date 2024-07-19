use crate::modules::get_config;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::ptr;
use winapi::shared::minwindef::DWORD;
use winapi::shared::ntdef::HANDLE;
use winapi::shared::windef::HICON;
use winapi::um::handleapi::CloseHandle;
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::psapi::GetModuleBaseNameW;
use winapi::um::winnt::PROCESS_QUERY_INFORMATION;
use winapi::um::winnt::PROCESS_VM_READ;
use winapi::um::winuser::{
    GetClassLongPtrW, GetForegroundWindow, GetWindowTextW, GetWindowThreadProcessId, SendMessageW,
    GCLP_HICON, GCLP_HICONSM, ICON_BIG, ICON_SMALL, WM_GETICON,
};
use winapi::um::winuser::{LoadIconW, IDI_APPLICATION};

pub fn get_window_info() -> (String, String) {
    unsafe {
        let h_wnd = GetForegroundWindow();
        let mut window_title: [u16; 255] = [0; 255];
        GetWindowTextW(h_wnd, window_title.as_mut_ptr(), 255);

        let mut process_id: DWORD = 0;
        GetWindowThreadProcessId(h_wnd, &mut process_id);

        let process_handle: HANDLE =
            OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, 0, process_id);
        let mut process_name: [u16; 255] = [0; 255];
        GetModuleBaseNameW(
            process_handle,
            ptr::null_mut(),
            process_name.as_mut_ptr(),
            255,
        );
        CloseHandle(process_handle);

        (
            OsString::from_wide(&process_name)
                .to_string_lossy()
                .into_owned(),
            OsString::from_wide(&window_title)
                .to_string_lossy()
                .into_owned(),
        )
    }
}

pub fn replacer(process_name: &str) -> String {
    let cfg = get_config::load_config();
    for rule in cfg.rules {
        if process_name == rule.match_application {
            return rule.replace.application.clone();
        }
    }
    process_name.to_string()
}

pub fn get_window_icon(window_title: &str) -> Option<HICON> {
    unsafe {
        let h_wnd = GetForegroundWindow();
        let mut current_window_title: [u16; 255] = [0; 255];
        GetWindowTextW(h_wnd, current_window_title.as_mut_ptr(), 255);
        let current_window_title = OsString::from_wide(&current_window_title)
            .to_string_lossy()
            .into_owned();

        if current_window_title == window_title {
            let h_icon = SendMessageW(h_wnd, WM_GETICON, ICON_BIG as usize, 0) as HICON;
            if h_icon.is_null() || h_icon == LoadIconW(ptr::null_mut(), IDI_APPLICATION) {
                let h_icon = SendMessageW(h_wnd, WM_GETICON, ICON_SMALL as usize, 0) as HICON;
                if h_icon.is_null() || h_icon == LoadIconW(ptr::null_mut(), IDI_APPLICATION) {
                    let h_icon = GetClassLongPtrW(h_wnd, GCLP_HICON) as HICON;
                    if h_icon.is_null() || h_icon == LoadIconW(ptr::null_mut(), IDI_APPLICATION) {
                        let h_icon = GetClassLongPtrW(h_wnd, GCLP_HICONSM) as HICON;
                        if h_icon.is_null() || h_icon == LoadIconW(ptr::null_mut(), IDI_APPLICATION)
                        {
                            let h_icon = LoadIconW(ptr::null_mut(), IDI_APPLICATION);
                            return Some(h_icon);
                        }
                        return Some(h_icon);
                    }
                    return Some(h_icon);
                }
                return Some(h_icon);
            }
            return Some(h_icon);
        }
    }
    None
}
