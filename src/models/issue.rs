// SPDX-License-Identifier: MIT
use crate::phrack_downloader_error::PhrackDownloaderError;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Issue {
    pub issue_number: u32,
}
impl From<u32> for Issue {
    fn from(value: u32) -> Self {
        Self {
            issue_number: value,
        }
    }
}
impl TryFrom<String> for Issue {
    type Error = PhrackDownloaderError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let issue_number: u32 = value.parse()?;
        Ok(Self { issue_number })
    }
}
impl fmt::Display for Issue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.issue_number)
    }
}
