use std::env;
use std::fs::File;
use serde_yaml;

mod error;
pub use self::error::ConfigError;


#[derive(Debug, Deserialize)]
pub struct Config {
    pub telegram: TelegramConfig,
    pub watcher: WatcherConfig,
}
#[derive(Debug, Deserialize)]
pub struct TelegramConfig {
    pub token: String,
    pub chat: i64,
}
#[derive(Debug, Deserialize)]
pub struct WatcherConfig {
    pub files: Vec<FileConfig>,
}
#[derive(Debug, Deserialize)]
pub struct FileConfig {
    pub path: String,
    pub chat: Option<i64>,
    pub regex: Option<String>,
}

impl Config {
    pub fn read() -> Result<Config, ConfigError> {
        let filename = match env::args().nth(1) {
            Some(filename) => filename,
            None => "config.yaml".to_string(),
        };
        let file = File::open(&filename)?;
        let config: Config = serde_yaml::from_reader(&file)?;

        Ok(config)
    }
}
