// SPDX-License-Identifier: MIT
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PhrackDownloaderError {
    #[error("HTTP error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Parse int error: {0}")]
    ParseInt(#[from] std::num::ParseIntError),
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("unknown error")]
    Unknown,
}
