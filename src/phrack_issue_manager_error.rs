// SPDX-License-Identifier: MIT
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PhrackIssueManagerError {
    #[error("HTTP error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Parse int error: {0}")]
    ParseInt(#[from] std::num::ParseIntError),
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("TOML error: {0}")]
    Toml(#[from] toml::ser::Error),
    #[error("TOML deserialization error: {0}")]
    TomlDe(#[from] toml::de::Error),
}
