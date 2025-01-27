#[cfg(target_os = "windows")]
mod windows {
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt;
    use std::ptr;
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

#[cfg(target_os = "windows")]
pub fn get_os_windows_info() -> (String, String, String) {
    unsafe {
        let h_wnd = windows::GetForegroundWindow();
        let mut window_title: [u16; 255] = [0; 255];
        windows::GetWindowTextW(h_wnd, window_title.as_mut_ptr(), 255);

        let mut process_id: windows::DWORD = 0;
        windows::GetWindowThreadProcessId(h_wnd, &mut process_id);

        let process_handle: windows::HANDLE = windows::OpenProcess(
            windows::PROCESS_QUERY_INFORMATION | windows::PROCESS_VM_READ,
            0,
            process_id,
        );
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
            get_window_icon(&window_title),
        );
    }
}

#[cfg(target_os = "windows")]
pub fn get_window_icon(window_title: &str) -> (String) {
    let hicon = get_os_windows_icon(window_title);
    let cache_file = crate::libs::cache::get_cache_directory().join("icon.png");

    // 判断数据是否正常
    if let Err(e) = convert_hicon_to_png(
        hicon,
        &cache_file.to_str().unwrap_or_default(),
    ) {
        eprintln!("Failed to convert icon to PNG: {}", e);
    }

    // 将图标转换为 base64 编码
    let icon_base64 = crate::modules::icon_converter::convert_png_to_base64(
        &cache_file.to_str().unwrap_or_default(),
    );
    return icon_base64;
}

#[cfg(target_os = "windows")]
pub fn get_os_windows_icon(window_title: &str) -> Option<HICON> {
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

#[cfg(target_os = "windows")]
pub fn convert_hicon_to_png(
    hicon_option: Option<HICON>,
    filename: &str,
) -> Result<(), Box<dyn Error>> {
    if let Some(hicon) = hicon_option {
        unsafe {
            // Get icon info
            let mut icon_info: ICONINFO = std::mem::zeroed();
            if GetIconInfo(hicon, &mut icon_info) == 0 {
                return Err("Failed to get icon info".into());
            }
            let hbitmap = icon_info.hbmColor;
            if hbitmap.is_null() {
                return Err("Icon does not have a color bitmap".into());
            }

            // Get bitmap information
            let mut bitmap_info: BITMAPINFO = std::mem::zeroed();
            let mut bitmap = BITMAP {
                bmType: 0,
                bmWidth: 0,
                bmHeight: 0,
                bmWidthBytes: 0,
                bmPlanes: 0,
                bmBitsPixel: 0,
                bmBits: ptr::null_mut(),
            };
            if GetObjectW(
                hbitmap as _,
                std::mem::size_of::<BITMAP>() as _,
                &mut bitmap as *mut _ as _,
            ) == 0
            {
                return Err("Failed to get bitmap object".into());
            }

            // Setup BITMAPINFOHEADER
            bitmap_info.bmiHeader.biSize = std::mem::size_of::<BITMAPINFOHEADER>() as u32;
            bitmap_info.bmiHeader.biWidth = bitmap.bmWidth;
            bitmap_info.bmiHeader.biHeight = -(bitmap.bmHeight); // Negative to get top-down image
            bitmap_info.bmiHeader.biPlanes = 1;
            bitmap_info.bmiHeader.biBitCount = 32;
            bitmap_info.bmiHeader.biCompression = 0; // BI_RGB
            bitmap_info.bmiHeader.biSizeImage = 0;
            bitmap_info.bmiHeader.biClrUsed = 0;
            bitmap_info.bmiHeader.biClrImportant = 0;

            // Allocate buffer and get bitmap bits
            let mut buffer = vec![0u8; (bitmap.bmWidth * bitmap.bmHeight * 4) as usize];
            if GetDIBits(
                GetDC(ptr::null_mut()),
                hbitmap,
                0,
                bitmap.bmHeight as u32,
                buffer.as_mut_ptr() as *mut _,
                &mut bitmap_info,
                DIB_RGB_COLORS,
            ) == 0
            {
                return Err("Failed to get bitmap bits".into());
            }

            // Convert BGRA to RGBA
            for i in 0..bitmap.bmWidth * bitmap.bmHeight {
                let offset = (i * 4) as usize;
                buffer.swap(offset, offset + 2); // Swap blue (B) and red (R)
            }

            // Convert to PNG using image crate
            let img = ImageBuffer::<Rgba<u8>, _>::from_raw(
                bitmap.bmWidth as u32,
                bitmap.bmHeight as u32,
                buffer,
            )
            .ok_or("Failed to create image buffer")?;
            let dynamic_image = DynamicImage::ImageRgba8(img);
            dynamic_image.save(filename)?;
            Ok(())
        }
    } else {
        Err("No HICON provided".into())
    }
}