#[cfg(target_os = "macos")]
use objc::rc::autoreleasepool;
use objc::runtime::{Class, Object};
use objc::{msg_send, sel, sel_impl};

use core_foundation::{
    array::{CFArrayGetCount, CFArrayGetValueAtIndex},
    base::{CFRelease, TCFType},
    dictionary::{CFDictionaryGetValue, CFDictionaryRef},
    number::{CFNumberGetValue, CFNumberRef},
    string::{CFString, CFStringRef},
};
use core_graphics::window::{
    kCGNullWindowID, kCGWindowListExcludeDesktopElements, kCGWindowListOptionOnScreenOnly,
    CGWindowListCopyWindowInfo,
};
use std::ffi::c_void;

//
// Powered by DeepSeek.ai
//

// 显式链接 macOS 框架
#[link(name = "AppKit", kind = "framework")]
#[link(name = "Foundation", kind = "framework")]
#[link(name = "ApplicationServices", kind = "framework")]
extern "C" {}

#[cfg(target_os = "macos")]
pub fn get_os_macos_info() -> Result<(String, String, String, String, String), String> {
    autoreleasepool(|| {
        unsafe {
            // 获取前台应用
            let ns_workspace_class =
                Class::get("NSWorkspace").ok_or("NSWorkspace class not found")?;
            let shared_workspace: *mut Object = msg_send![ns_workspace_class, sharedWorkspace];
            let front_app: *mut Object = msg_send![shared_workspace, frontmostApplication];
            if front_app.is_null() {
                return Err("Failed to get frontmost application".to_string());
            }

            // 获取应用名称
            let app_name: *mut Object = msg_send![front_app, localizedName];
            let app_name_str: *const i8 = msg_send![app_name, UTF8String];
            let process_name = std::ffi::CStr::from_ptr(app_name_str)
                .to_string_lossy()
                .into_owned();

            // 获取 Bundle ID
            let bundle_id: *mut Object = msg_send![front_app, bundleIdentifier];
            if bundle_id.is_null() {
                return Err("Failed to get bundle identifier".to_string());
            }
            let bundle_id_str: *const i8 = msg_send![bundle_id, UTF8String];
            let bundle_id = std::ffi::CStr::from_ptr(bundle_id_str)
                .to_string_lossy()
                .into_owned();

            // 获取应用路径
            let app_url: *mut Object = msg_send![front_app, bundleURL];
            if app_url.is_null() {
                return Err("Failed to get app URL".to_string());
            }
            let path: *mut Object = msg_send![app_url, path];
            let path_str: *const i8 = msg_send![path, UTF8String];
            let path = std::ffi::CStr::from_ptr(path_str)
                .to_string_lossy()
                .into_owned();

            // 获取窗口标题
            let window_list = CGWindowListCopyWindowInfo(
                kCGWindowListOptionOnScreenOnly | kCGWindowListExcludeDesktopElements,
                kCGNullWindowID,
            );
            let mut window_title = String::from("Unknown");

            if !window_list.is_null() {
                let count = CFArrayGetCount(window_list);
                let k_window_layer = CFString::from_static_string("kCGWindowLayer");
                let k_window_name = CFString::from_static_string("kCGWindowName");
                let k_cfnumber_sint32_type: u32 = 3;

                for i in 0..count {
                    let window_info = CFArrayGetValueAtIndex(window_list, i) as CFDictionaryRef;
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
                            let window_name_ptr = CFDictionaryGetValue(
                                window_info,
                                k_window_name.as_concrete_TypeRef() as *const c_void,
                            ) as CFStringRef;

                            if !window_name_ptr.is_null() {
                                window_title =
                                    CFString::wrap_under_get_rule(window_name_ptr).to_string();
                                break;
                            }
                        }
                    }
                }
                CFRelease(window_list as *mut c_void);
            }

            // 获取应用图标
            let icon: *mut Object = msg_send![front_app, icon];
            if icon.is_null() {
                return Err("Failed to get app icon".to_string());
            }

            let tiff_data: *mut Object = msg_send![icon, TIFFRepresentation];
            let ns_bitmap_image_rep_class =
                Class::get("NSBitmapImageRep").ok_or("NSBitmapImageRep class not found")?;
            let bitmap_rep: *mut Object =
                msg_send![ns_bitmap_image_rep_class, imageRepWithData: tiff_data];

            let ns_png_file_type = 1;
            let properties: *mut Object = msg_send![Class::get("NSDictionary").unwrap(), new];
            let png_data: *mut Object = msg_send![bitmap_rep, representationUsingType: ns_png_file_type properties: properties];

            let length: usize = msg_send![png_data, length];
            let mut buffer = vec![0u8; length];
            let _: () = msg_send![png_data, getBytes: buffer.as_mut_ptr() length: length];

            let file_path = crate::libs::cache::get_cache_directory().join("icon.png");
            std::fs::write(&file_path, &buffer)
                .map_err(|e| format!("Failed to save icon: {}", e))?;

            let icon_base64 = crate::modules::icon_converter::convert_png_to_base64(
                file_path.to_str().ok_or("Failed to convert icon to base64")?,
            );
            // 转为option<string> to string
            let icon_base64 = icon_base64.ok_or("Failed to convert icon to base64")?;

            Ok((process_name, bundle_id, path, window_title, icon_base64))
        }
    })
}
