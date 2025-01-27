#[cfg(target_os = "windows")]
mod windows {
    use base64::Engine;
    use std::fs::File;
    use std::io::Write;
    use tokio::runtime::Runtime;
    use windows::core::Result;
    use windows::core::HSTRING;
    use windows::Foundation::IAsyncOperation;
    use windows::Media::Control::{
        GlobalSystemMediaTransportControlsSessionManager,
        GlobalSystemMediaTransportControlsSessionPlaybackInfo,
    };
    use windows::Storage::Streams::DataReader;
}

#[cfg(target_os = "windows")]
pub fn get_media_info() -> (String, String, String, String, String, String) {
    let rt = Runtime::new().expect("Failed to create Tokio runtime");

    match rt.block_on(async_main()) {
        Ok(info) => info,
        Err(e) => {
            crate::modules::logs::log_message("error", &e.to_string());
            (
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
            )
        }
    }
}

#[cfg(target_os = "windows")]
async fn async_main() -> Result<(String, String, String, String, String, String)> {
    let session_manager_operation: IAsyncOperation<
        GlobalSystemMediaTransportControlsSessionManager,
    > = GlobalSystemMediaTransportControlsSessionManager::RequestAsync()?;
    let session_manager = session_manager_operation.get()?;
    let current_session = session_manager.GetCurrentSession()?;

    // 获取播放信息
    let playback_info: GlobalSystemMediaTransportControlsSessionPlaybackInfo =
        current_session.GetPlaybackInfo()?;
    let playback_status = playback_info.PlaybackStatus()?;
    // 检查播放状态，如果是暂停或其他非播放状态，返回空值
    if playback_status
        != windows::Media::Control::GlobalSystemMediaTransportControlsSessionPlaybackStatus::Playing
    {
        return Ok((
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
        ));
    }

    // 获取媒体属性
    let media_properties_operation = current_session.TryGetMediaPropertiesAsync()?;
    let media_properties = media_properties_operation.get()?;

    let source_app_name_hstring: HSTRING = current_session.SourceAppUserModelId()?.into();
    let title_hstring: HSTRING = media_properties.Title()?.into();
    let artist_hstring: HSTRING = media_properties.Artist()?.into();
    let album_title_hstring: HSTRING = media_properties.AlbumTitle()?.into();
    let album_artist_hstring: HSTRING = media_properties.AlbumArtist()?.into();

    let mut album_thumbnail = "".to_string();

    // 获取缩略图
    let thumbnail_ref = media_properties.Thumbnail()?;
    let thumbnail_stream = thumbnail_ref.OpenReadAsync()?.get()?;
    // 使用 DataReader 读取流中的数据
    let data_reader = DataReader::CreateDataReader(&thumbnail_stream)?;
    let stream_size = thumbnail_stream.Size()?;
    // 将 u64 转换为 u32，处理可能的溢出
    let stream_size_u32: u32 = stream_size.try_into().unwrap_or(0);
    if stream_size_u32 > 0 {
        data_reader.LoadAsync(stream_size_u32)?.get()?;
        // 读取字节数据
        let mut thumbnail_data = vec![0u8; stream_size_u32 as usize];
        data_reader.ReadBytes(&mut thumbnail_data)?;

        // 将缩略图数据编码为 Base64
        album_thumbnail = base64::engine::general_purpose::STANDARD.encode(&thumbnail_data);

        // 保存本地缓存
        let cache_path = crate::libs::cache::get_cache_directory();
        let cache_file_path = cache_path.join("album_thumbnail.png");

        // 写入文件
        let mut file = File::create(&cache_file_path)?;
        file.write_all(&thumbnail_data)?;
    }

    Ok((
        title_hstring.to_string_lossy().to_owned(),
        artist_hstring.to_string_lossy().to_owned(),
        source_app_name_hstring.to_string_lossy().to_owned(),
        album_title_hstring.to_string_lossy().to_owned(),
        album_artist_hstring.to_string_lossy().to_owned(),
        album_thumbnail,
    ))
}
