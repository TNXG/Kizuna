use serde::{Deserialize, Serialize};
use std::fs;
use std::process::exit;

#[derive(Debug, Deserialize, Serialize)]
pub struct ServerConfig {
    pub endpoint: String,
    pub token: String,
    pub report_time: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Rule {
    pub match_application: String,
    pub replace: Replace,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Replace {
    pub application: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MainConfig {
    pub server_config: ServerConfig,
    pub rules: Vec<Rule>,
}

pub fn load_config() -> MainConfig {
    let workdir = std::env::current_dir().unwrap();
    let config_path = workdir.join("config.yml");
    let data = fs::read_to_string(&config_path).unwrap_or_else(|err| {
        eprintln!("error: {}", err);
        exit(1);
    });
    let config: MainConfig = serde_yaml::from_str(&data).unwrap_or_else(|err| {
        eprintln!("error: {}", err);
        exit(1);
    });
    return config;
}