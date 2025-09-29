// SPDX-License-Identifier: MIT
use crate::config;
use crate::models::article::Article;
use crate::models::phrack_pdf::PhrackPdf;
use crate::phrack_issue_manager_error::PhrackIssueManagerError;
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
    type Error = PhrackIssueManagerError;

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

impl Issue {
    pub fn all_issues() -> Result<Vec<Issue>, PhrackIssueManagerError> {
        let config = config::load_config()?;
        let subfolders = config.download_path().read_dir()?;

        let issues: Result<Vec<Issue>, PhrackIssueManagerError> = subfolders
            .map(|entry| {
                let entry = entry?;
                let file_name = entry.file_name();
                let issue_number_str = file_name.to_string_lossy();
                let issue_number = issue_number_str.parse::<u32>()?;
                Ok(Issue::from(issue_number))
            })
            .collect();

        Ok(issues?)
    }
}
