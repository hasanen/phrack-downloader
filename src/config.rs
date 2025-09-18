// SPDX-License-Identifier: MIT

use crate::{phrack_issue_manager_error::PhrackIssueManagerError, strict_string::PhrackArchiveUrl};
use clap::ValueEnum;
use directories_next::UserDirs;
use enum_iterator::Sequence;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, ValueEnum, Debug, PartialEq, Sequence)]
pub enum ConfigKey {
    DownloadPath,
    PhrackArchiveUrl,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    download_path: PathBuf,
    phrack_archive_url: PhrackArchiveUrl,
}

pub fn load_config() -> Result<Config, PhrackIssueManagerError> {
    if !config_path().exists() {
        std::fs::create_dir_all(config_dir())?;
        let default_config = Config {
            download_path: config_dir().join("issues"),
            phrack_archive_url: PhrackArchiveUrl::new("https://archives.phrack.org"),
        };
        save_config(&default_config)?;
    }
    let content = std::fs::read_to_string(config_path())?;
    let config: Config = toml::from_str(&content)?;

    Ok(config)
}
pub fn save_config(config: &Config) -> Result<(), PhrackIssueManagerError> {
    let toml_str = toml::to_string_pretty(&config)?;
    std::fs::write(config_path(), toml_str)?;

    Ok(())
}

fn user_dir() -> PathBuf {
    let user_dirs = UserDirs::new().unwrap();
    PathBuf::from(user_dirs.home_dir())
}

fn config_dir() -> PathBuf {
    user_dir().join(".config/phrack-downloader")
}

fn config_path() -> PathBuf {
    config_dir().join("config.toml")
}

impl Config {
    pub fn get_as_str(&self, key: &ConfigKey) -> String {
        match key {
            ConfigKey::DownloadPath => self.download_path.display().to_string(),
            ConfigKey::PhrackArchiveUrl => self.phrack_archive_url.as_str().to_string(),
        }
    }
    pub fn download_path(&self) -> &PathBuf {
        &self.download_path
    }
    pub fn phrack_archive_url(&self) -> &PhrackArchiveUrl {
        &self.phrack_archive_url
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
