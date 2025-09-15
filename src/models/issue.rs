// SPDX-License-Identifier: MIT
use crate::models::article::Article;
use crate::phrack_downloader_error::PhrackDownloaderError;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Issue {
    pub issue_number: u32,
    pub articles: Vec<Article>,
}
impl From<u32> for Issue {
    fn from(value: u32) -> Self {
        Self {
            issue_number: value,
            articles: Vec::new(),
        }
    }
}
impl Into<Vec<Issue>> for Issue {
    fn into(self) -> Vec<Issue> {
        vec![self]
    }
}
impl TryFrom<String> for Issue {
    type Error = PhrackDownloaderError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self {
            issue_number: value.parse()?,
            articles: Vec::new(),
        })
    }
}
impl fmt::Display for Issue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.issue_number)
    }
}
