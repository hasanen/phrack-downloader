use thiserror::Error;

#[derive(Error, Debug)]
pub enum PhrackDownloaderlError {
    #[error("HTTP error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Parse int error: {0}")]
    ParseInt(#[from] std::num::ParseIntError),
    #[error("unknown error")]
    Unknown,
}
