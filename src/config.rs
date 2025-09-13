use crate::strict_string::DownloadPath;
use clap::ValueEnum;

#[derive(Clone, ValueEnum, Debug)]
pub enum ConfigKey {
    DownloadPath,
}

pub struct Config {
    download_path: DownloadPath,
}

pub fn load_config() -> Config {
    // Placeholder implementation
    Config {
        download_path: DownloadPath::new("./config/phrack-downloader/issues/"),
    }
}

impl Config {
    pub fn get_value(&self, key: &ConfigKey) -> String {
        match key {
            ConfigKey::DownloadPath => self.download_path.to_string(),
        }
    }

    pub fn set_value(&mut self, key: &ConfigKey, value: &str) {
        match key {
            ConfigKey::DownloadPath => {
                self.download_path = DownloadPath::new(value);
            }
        }
    }
}
