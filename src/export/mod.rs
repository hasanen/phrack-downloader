use clap::ValueEnum;
use enum_iterator::Sequence;
use std::path::PathBuf;

pub mod txt_export;

// SPDX-License-Identifier: MIT
use crate::models::issue::Issue;
use crate::phrack_issue_manager_error::PhrackIssueManagerError;

#[derive(ValueEnum, Clone, Debug, Sequence)]
#[value(rename_all = "lowercase")]
pub enum ExportFormat {
    #[value(alias = "text")]
    TXT,
}
pub struct ExportOptions {
    // Output folder for the exported file
    pub output_folder: PathBuf,
}

pub trait Exporter {
    fn export_all(&self, options: &ExportOptions) -> Result<(), PhrackIssueManagerError> {
        Issue::all_issues()?
            .into_iter()
            .try_for_each(|issue| self.export(issue, options))
    }
    fn export(&self, issue: Issue, options: &ExportOptions) -> Result<(), PhrackIssueManagerError>;
}
