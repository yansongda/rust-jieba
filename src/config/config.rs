use std::env;

use dotenv::dotenv;

pub struct Config {
    pub app: AppConfig,
    pub logger: LoggerConfig,
    pub jieba: JiebaConfig,
}

pub struct AppConfig {
    pub name: String,
    pub listen: String,
    pub port: u16,
}

pub struct LoggerConfig {
    pub name: String,
    pub level: String,
}

#[derive(Copy)]
pub struct JiebaConfig {
    pub fixed: String,
}

impl Config {
    pub fn init() -> Self {
        dotenv().ok();

        Config {
            app: AppConfig::default(),
            logger: LoggerConfig::default(),
            jieba: JiebaConfig::default(),
        }
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            name: env::var("APP_NAME").unwrap_or_else(|_| String::from("jieba")),
            listen: env::var("APP_LISTEN").unwrap_or_else(|_| String::from("0.0.0.0")),
            port: env::var("APP_PORT").map_or_else(|_| 8080, |v| v.parse().unwrap()),
        }
    }
}

impl Default for LoggerConfig {
    fn default() -> Self {
        LoggerConfig {
            name: env::var("LOGGER_NAME").unwrap_or_else(|_| String::from("jieba")),
            level: env::var("LOGGER_LEVEL").unwrap_or_else(|_| String::from("info")),
        }
    }
}

impl Default for JiebaConfig {
    fn default() -> Self {
        JiebaConfig {
            fixed: env::var("JIEBA_FIXED").unwrap_or_else(|_| String::from("")),
        }
    }
}
