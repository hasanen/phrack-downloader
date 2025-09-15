// SPDX-License-Identifier: MIT
use crate::models::article::Article;
use crate::models::phrack_pdf::PhrackPdf;
use crate::phrack_downloader_error::PhrackDownloaderError;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Issue {
    pub issue_number: u32,
    pub articles: Vec<Article>,
    pub phrack_pdf: Option<PhrackPdf>,
}
impl Default for Issue {
    fn default() -> Self {
        Self {
            issue_number: 0,
            articles: Vec::new(),
            phrack_pdf: None,
        }
    }
}
impl From<u32> for Issue {
    fn from(value: u32) -> Self {
        Self {
            issue_number: value,
            ..Default::default()
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
            ..Default::default()
        })
    }
}
impl fmt::Display for Issue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.issue_number)
    }
}
