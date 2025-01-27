use std::ffi::OsString;
use std::ptr;

// 只在 Windows 平台下导入相关模块
#[cfg(target_os = "windows")]
mod windows {
    use std::os::windows::ffi::OsStringExt;
    use winapi::shared::minwindef::DWORD;
    use winapi::shared::ntdef::HANDLE;
    use winapi::shared::windef::HICON;
    use winapi::um::handleapi::CloseHandle;
    use winapi::um::processthreadsapi::OpenProcess;
    use winapi::um::psapi::GetModuleBaseNameW;
    use winapi::um::winnt::{PROCESS_QUERY_INFORMATION, PROCESS_VM_READ};
    use winapi::um::winuser::{
        GetClassLongPtrW, GetForegroundWindow, GetWindowTextW, GetWindowThreadProcessId,
        SendMessageW, GCLP_HICON, GCLP_HICONSM, ICON_BIG, ICON_SMALL, WM_GETICON,
    };
    use winapi::um::winuser::{LoadIconW, IDI_APPLICATION};
}

#[cfg(target_os = "macos")]
use {
    core_foundation::{
        array::{CFArrayGetCount, CFArrayGetValueAtIndex},
        base::{CFRelease, TCFType},
        dictionary::{CFDictionaryRef, CFDictionaryGetValue},
        string::{CFString, CFStringRef},
        number::{CFNumberGetValue, CFNumberRef},
    },
    core_graphics::window::{CGWindowListCopyWindowInfo, kCGWindowListOptionOnScreenOnly, kCGNullWindowID, kCGWindowListExcludeDesktopElements},
    std::ffi::c_void,
};

pub fn get_window_info() -> (String, String) {
    #[cfg(target_os = "windows")]
    if cfg!(target_os = "windows") {
        unsafe {
            let h_wnd = windows::GetForegroundWindow();
            let mut window_title: [u16; 255] = [0; 255];
            windows::GetWindowTextW(h_wnd, window_title.as_mut_ptr(), 255);

            let mut process_id: windows::DWORD = 0;
            windows::GetWindowThreadProcessId(h_wnd, &mut process_id);

            let process_handle: windows::HANDLE =
                windows::OpenProcess(windows::PROCESS_QUERY_INFORMATION | windows::PROCESS_VM_READ, 0, process_id);
            let mut process_name: [u16; 255] = [0; 255];
            windows::GetModuleBaseNameW(
                process_handle,
                ptr::null_mut(),
                process_name.as_mut_ptr(),
                255,
            );
            windows::CloseHandle(process_handle);

            return (
                OsString::from_wide(&process_name)
                    .to_string_lossy()
                    .into_owned(),
                OsString::from_wide(&window_title)
                    .to_string_lossy()
                    .into_owned(),
            )
        }
    }

    #[cfg(target_os = "macos")]
    if cfg!(target_os = "macos") {
        unsafe {
            // 获取窗口列表
            let window_list = CGWindowListCopyWindowInfo(
                kCGWindowListOptionOnScreenOnly | kCGWindowListExcludeDesktopElements,
                kCGNullWindowID,
            );
            if window_list.is_null() {
                eprintln!("Failed to get window list");
                return ("Unknown".to_string(), "Unknown".to_string());
            }
    
            let count = CFArrayGetCount(window_list);
            if count == 0 {
                eprintln!("No windows found");
                CFRelease(window_list as *mut c_void);
                return ("Unknown".to_string(), "Unknown".to_string());
            }
    
            // 预定义 CFString 键
            let k_window_layer = CFString::from_static_string("kCGWindowLayer");
            let k_window_name = CFString::from_static_string("kCGWindowName");
            let k_window_owner = CFString::from_static_string("kCGWindowOwnerName");
    
            let k_cfnumber_sint32_type: u32 = 3;
    
            // 遍历窗口列表，找到最前台的窗口
            for i in 0..count {
                let window_info = CFArrayGetValueAtIndex(window_list, i) as CFDictionaryRef;
    
                // 检查窗口层级
                let window_layer = CFDictionaryGetValue(
                    window_info,
                    k_window_layer.as_concrete_TypeRef() as *const c_void,
                );
                if !window_layer.is_null() {
                    let mut layer_value: i32 = 0;
                    if CFNumberGetValue(
                        window_layer as CFNumberRef,
                        k_cfnumber_sint32_type,
                        &mut layer_value as *mut _ as *mut c_void,
                    ) && layer_value == 0
                    {
                        // 获取窗口标题
                        let window_name_ptr = CFDictionaryGetValue(
                            window_info,
                            k_window_name.as_concrete_TypeRef() as *const c_void,
                        ) as CFStringRef;
    
                        let window_title = if !window_name_ptr.is_null() {
                            CFString::wrap_under_get_rule(window_name_ptr).to_string()
                        } else {
                            "Unknown".to_string()
                        };
    
                        // 获取应用名称
                        let window_owner_ptr = CFDictionaryGetValue(
                            window_info,
                            k_window_owner.as_concrete_TypeRef() as *const c_void,
                        ) as CFStringRef;
    
                        let process_name = if !window_owner_ptr.is_null() {
                            CFString::wrap_under_get_rule(window_owner_ptr).to_string()
                        } else {
                            "Unknown".to_string()
                        };
    
                        CFRelease(window_list as *mut c_void);
                        return (window_title, process_name);
                    }
                }
            }
    
            // 如果没有找到符合条件的窗口
            CFRelease(window_list as *mut c_void);
            return ("Unknown".to_string(), "Unknown".to_string())
        }
    }

    #[cfg(target_os = "linux")] // todo：linux 平台暂时不支持
    if cfg!(target_os = "linux") {
        (String::new(), String::new())
    } 
        
    (String::new(), String::new())
    
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
