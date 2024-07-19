extern crate image;
extern crate winapi;

use image::{DynamicImage, ImageBuffer, Rgba};
use std::error::Error;
use std::ptr;
use winapi::shared::windef::HICON;
use winapi::um::wingdi::{GetDIBits, GetObjectW, BITMAP, BITMAPINFO, BITMAPINFOHEADER, DIB_RGB_COLORS};
use winapi::um::winuser::{GetDC, GetIconInfo, ICONINFO};

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

pub fn convert_png_to_base64(filename: &str) -> Option<String> {
    use std::fs::File;
    use std::io::Read;
    use base64::engine::general_purpose::STANDARD;
    use base64::Engine; // 导入 Engine trait

    let mut file = File::open(filename).ok()?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).ok()?;
    Some(STANDARD.encode(&buffer))
}
