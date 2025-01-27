use chrono::Local;
use std::fs::{create_dir_all, OpenOptions};
use std::io::{BufWriter, Write};
use std::sync::Mutex;
use std::path::Path;
use once_cell::sync::Lazy;

static LOG_WRITER: Lazy<Mutex<BufWriter<std::fs::File>>> = Lazy::new(|| {
    let now = Local::now();
    let date_str = now.format("%Y-%m-%d").to_string();
    let log_dir = if cfg!(dev) { "../logs" } else { "/logs" };

    if !Path::new(log_dir).exists() {
        create_dir_all(log_dir).expect("无法创建日志目录");
    }

    let log_file_path = format!("{}/{}.log", log_dir, date_str);

    let file = OpenOptions::new()
        .create(true)
        .append(true) // 确保每次写入时不会覆盖已有内容
        .open(log_file_path)
        .expect("无法打开日志文件");

    Mutex::new(BufWriter::new(file))
});

pub fn log_message(level: &str, message: &str) -> String {
    let now = Local::now();
    let log_entry = format!(
        "[{}] [{}] {}\n",
        now.format("%Y-%m-%d %H:%M:%S"),
        level,
        message
    );

    {
        let mut writer = LOG_WRITER.lock().unwrap(); // 获取锁，确保线程安全
        writer.write_all(log_entry.as_bytes()).expect("无法写入日志文件"); // 写入日志
        writer.flush().expect("无法刷新日志内容"); // 刷新缓冲区
    }

    println!("{}", log_entry); // 控制台输出日志
    log_entry
}

pub fn get_today_log() -> String {
    let now = Local::now();
    let date_str = now.format("%Y-%m-%d").to_string();
    let log_dir = if cfg!(dev) { "../logs" } else { "/logs" };
    let log_file_path = format!("{}/{}.log", log_dir, date_str);

    if Path::new(&log_file_path).exists() {
        std::fs::read_to_string(log_file_path).expect("无法读取日志文件")
    } else {
        String::from("今日无日志记录")
    }
}
