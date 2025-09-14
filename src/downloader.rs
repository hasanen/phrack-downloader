// SPDX-License-Identifier: MIT
use crate::config::Config;

pub struct Downloader {
    config: Config,
}

impl Downloader {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn download_issue(&self, issue_number: u32) {
        // Placeholder for downloading a specific issue
        println!("Downloading issue number: {}", issue_number);
    }

    pub fn download_all_issues(&self) {
        // Placeholder for downloading all issues
        println!("Downloading all issues...");
    }
}
