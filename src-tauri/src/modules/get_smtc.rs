use tokio::runtime::Runtime;
use windows::core::Result;
use windows::core::HSTRING;
use windows::Foundation::IAsyncOperation;
use windows::Media::Control::GlobalSystemMediaTransportControlsSessionManager;

pub fn get_media_info() -> (String, String, String) {
    let rt = Runtime::new().expect("Failed to create Tokio runtime");
    match rt.block_on(async_main()) {
        Ok(info) => info,
        Err(_) => ("".to_string(), "".to_string(), "".to_string()),
    }
}

async fn async_main() -> Result<(String, String, String)> {
    let session_manager_operation: IAsyncOperation<
        GlobalSystemMediaTransportControlsSessionManager,
    > = GlobalSystemMediaTransportControlsSessionManager::RequestAsync()?;
    let session_manager = session_manager_operation.get()?;
    let current_session = session_manager.GetCurrentSession()?;
    let media_properties_operation = current_session.TryGetMediaPropertiesAsync()?;
    let media_properties = media_properties_operation.get()?;

    // 将String转换为HSTRING
    let source_app_name_hstring: HSTRING = current_session.SourceAppUserModelId()?.into();
    let title_hstring: HSTRING = media_properties.Title()?.into();
    let artist_hstring: HSTRING = media_properties.Artist()?.into();

    // 使用HSTRING类型的变量

    Ok((
        title_hstring.to_string_lossy().to_owned(),
        artist_hstring.to_string_lossy().to_owned(),
        source_app_name_hstring.to_string_lossy().to_owned(),
    ))
}
