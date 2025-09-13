// SPDX-License-Identifier: MIT
use crate::strict_string::{DownloadPath, PhrackArchiveUrl};
use clap::ValueEnum;
use enum_iterator::Sequence;

#[derive(Clone, ValueEnum, Debug, PartialEq, Sequence)]
pub enum ConfigKey {
    DownloadPath,
    PhrackArchiveUrl,
}

pub struct Config {
    download_path: DownloadPath,
    phrack_archive_url: PhrackArchiveUrl,
}

pub fn load_config() -> Config {
    // Placeholder implementation
    Config {
        download_path: DownloadPath::new("./config/phrack-downloader/issues/"),
        phrack_archive_url: PhrackArchiveUrl::new("https://archives.phrack.org/issues/"),
    }
}

impl Config {
    pub fn get_value(&self, key: &ConfigKey) -> String {
        match key {
            ConfigKey::DownloadPath => self.download_path.to_string(),
            ConfigKey::PhrackArchiveUrl => self.phrack_archive_url.to_string(),
        }
    }

    pub fn set_value(&mut self, key: &ConfigKey, value: &str) {
        match key {
            ConfigKey::DownloadPath => {
                self.download_path = DownloadPath::new(value);
            }
            ConfigKey::PhrackArchiveUrl => {
                self.phrack_archive_url = PhrackArchiveUrl::new(value);
            }
        }
    }
}
