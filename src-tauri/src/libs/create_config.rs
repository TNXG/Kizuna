use std::fs;
use std::io::Write;

pub const DEFAULT_CONFIG: &str = r#"
server_config:
  endpoint: "apiurl" # https://api.example.com/api/v2/fn/ps/update
  token: "apikey" # 设置的key
  report_time: 5 # 上报时间间隔，单位秒
rules: # 软件名的替换规则
  - match_application: WeChat
    replace:
      application: 微信
      description: 一个小而美的办公软件
  - match_application: QQ
    replace:
      application: QQ
      description: 一个多功能的通讯软件
  - match_application: Netease Cloud Music
    replace:
      application: 网易云音乐
      description: 一个音乐播放和分享的平台
"#;

pub fn create_config_file() -> std::io::Result<()> {
    use std::path::Path;

    let config_file = if cfg!(dev) {
        Path::new("..").join("config.yml").to_str().unwrap().to_string()
    } else {
        "config.yml".to_string()
    };
    
    // 检查文件是否存在
    if !fs::metadata(&config_file).is_ok() {
        let mut file = fs::File::create(&config_file)?;
        file.write_all(DEFAULT_CONFIG.as_bytes())?;
    }
    
    Ok(())
}
