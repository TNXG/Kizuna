use serde_json::Value;
use std::collections::HashMap;

pub fn report(
    endpoint: &str,
    token: &str,
) -> (
    String,
    HashMap<String, Value>,
    String,
    HashMap<String, String>,
    String,
) {
    // 从 get_processes 模块获取当前前台进程名称和窗口名称以及icon_base64
    let (process_name, window_name,icon_base64) = crate::modules::get_processes::get_window_info();

    // 自定义程序名：从配置文件中读取规则，替换进程名
    let process_name = crate::modules::get_processes::replacer(&process_name.replace(".exe", ""));

    // 获取媒体信息
    let (title, artist, source_app_name, album_title, album_artist, album_thumbnail) =
        crate::modules::get_media::get_media_info();

    // 构建媒体更新请求
    let media_update = crate::modules::requests::build_media_update(
        &title,
        &artist,
        &source_app_name,
        &album_title,
        &album_artist,
        &album_thumbnail,
    );
    // 将上一步的媒体信息同程序名构建请求数据
    let mut update_data =
        crate::modules::requests::build_data(&process_name, media_update.clone(), token);

    // 打印日志（为什么要在打印日志的参数里调用report）
    let logdata = crate::modules::logs::log_message(
        "info",
        crate::modules::requests::report(update_data.clone(), endpoint).as_str(),
    );

    // 移除构建数据当中的 key 字段
    update_data.remove("key");
    // 插入窗口名称
    update_data.insert(
        "window_name".to_string(),
        serde_json::Value::String(window_name.trim_end_matches('\u{0000}').to_string()),
    );

    (
        logdata,
        update_data,
        icon_base64,
        media_update,
        album_thumbnail,
    )
}
