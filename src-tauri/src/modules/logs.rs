use chrono::Local;
use std::fs::{create_dir_all, read_to_string, OpenOptions};
use std::io::Write;
use std::path::Path;

pub fn log_message(level: &str, message: &str) -> String {
    let now = Local::now();
    let date_str = now.format("%Y-%m-%d").to_string();
    let log_dir = "logs";
    let log_file_path = format!("{}/{}.log", log_dir, date_str);

    // 创建日志目录（如果不存在）
    if !Path::new(log_dir).exists() {
        create_dir_all(log_dir).expect("无法创建日志目录");
    }

    // 打开日志文件（如果不存在则创建）
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_file_path)
        .expect("无法打开日志文件");

    // 写入日志消息
    let log_entry = format!(
        "[{}] [{}] {}\n",
        now.format("%Y-%m-%d %H:%M:%S"),
        level,
        message
    );
    file.write_all(log_entry.as_bytes())
        .expect("无法写入日志文件");

    // 打印日志消息到控制台
    println!("{}", log_entry);

    // 返回日志消息
    log_entry
}

pub fn get_today_log() -> String {
    let now = Local::now();
    let date_str = now.format("%Y-%m-%d").to_string();
    let log_file_path = format!("logs/{}.log", date_str);

    if Path::new(&log_file_path).exists() {
        read_to_string(log_file_path).expect("无法读取日志文件")
    } else {
        String::from("今日无日志记录")
    }
}
