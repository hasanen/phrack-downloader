// SPDX-License-Identifier: MIT

use crate::{
    phrack_downloader_error::PhrackDownloaderError,
    strict_string::{DownloadPath, PhrackArchiveUrl},
};
use clap::ValueEnum;
use directories_next::UserDirs;
use enum_iterator::Sequence;
use std::path::PathBuf;

#[derive(Clone, ValueEnum, Debug, PartialEq, Sequence)]
pub enum ConfigKey {
    DownloadPath,
    PhrackArchiveUrl,
}

pub struct Config {
    download_path: PathBuf,
    phrack_archive_url: PhrackArchiveUrl,
}

pub fn load_config() -> Result<Config, PhrackDownloaderError> {
    // Placeholder implementation
    let user_dirs = UserDirs::new().unwrap();
    Ok(Config {
        download_path: PathBuf::from(user_dirs.home_dir())
            .join(".config/phrack-downloader/issues/"),

        phrack_archive_url: PhrackArchiveUrl::new("https://archives.phrack.org"),
    })
}
pub fn save_config(_config: &Config) {
    // Placeholder for saving the config to a file
    println!("Config saved (placeholder)");
}

impl Config {
    pub fn get_value(&self, key: &ConfigKey) -> String {
        match key {
            ConfigKey::DownloadPath => self.download_path.display().to_string(),
            ConfigKey::PhrackArchiveUrl => self.phrack_archive_url.to_string(),
        }
    }

    pub fn set_value(&mut self, key: &ConfigKey, value: &str) {
        match key {
            ConfigKey::DownloadPath => {
                self.download_path = PathBuf::from(value);
            }
            ConfigKey::PhrackArchiveUrl => {
                self.phrack_archive_url = PhrackArchiveUrl::new(value);
            }
        }
    }
}
impl ConfigKey {
    pub fn as_arg(&self) -> String {
        // to_possible_value() always returns Some for ValueEnum
        self.to_possible_value().unwrap().get_name().to_string()
    }
}
