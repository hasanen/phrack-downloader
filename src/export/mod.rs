// SPDX-License-Identifier: MIT
use crate::phrack_issue_manager_error::PhrackIssueManagerError;

pub trait Exporter {
    fn export(&self) -> Result<(), PhrackIssueManagerError>;
}
