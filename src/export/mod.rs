use std::path::PathBuf;

// SPDX-License-Identifier: MIT
use crate::models::issue::Issue;
use crate::phrack_issue_manager_error::PhrackIssueManagerError;

pub enum ExportFormat {
    TXT,
}
pub struct ExportOptions {
    pub format: ExportFormat,
    // Output folder for the exported file
    pub output_folder: PathBuf,
    pub issue: Issue,
}

pub trait Exporter {
    fn export(&self, options: &ExportOptions) -> Result<(), PhrackIssueManagerError>;
}
