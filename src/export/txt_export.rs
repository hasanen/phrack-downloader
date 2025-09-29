use crate::export::{ExportOptions, Exporter};
use crate::models::issue::Issue;

pub struct TxtExporter;

impl Exporter for TxtExporter {
    fn export(
        &self,
        issue: Issue,
        options: &ExportOptions,
    ) -> Result<(), crate::phrack_issue_manager_error::PhrackIssueManagerError> {
        println!(
            "Exporting issue {} to {}",
            issue.issue_number,
            options.output_folder.display()
        );

        Ok(())
    }
}
