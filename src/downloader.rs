// SPDX-License-Identifier: MIT
use crate::config::Config;
use crate::phrack_downloader_error::PhrackDownloaderlError;
use regex::Regex;
use reqwest;
use scraper::{Html, Selector};

pub struct Downloader {
    config: Config,
}

impl Downloader {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn download_all_issues(&self, refresh: bool) -> Result<(), PhrackDownloaderlError> {
        println!("Checking available issues");
        let url = self
            .config
            .get_value(&crate::config::ConfigKey::PhrackArchiveUrl);
        let document = self.fetch_html(&url).unwrap();
        let issues = self.parse_issues(&document)?;

        println!("Found {} issues", issues.len());
        for issue_number in issues {
            self.download_issue(issue_number.parse()?, refresh)
        }
        Ok(())
    }

    pub fn download_issue(&self, issue_number: u32, refresh: bool) {
        // Placeholder for downloading a specific issue
        println!("Downloading issue number: {}", issue_number);
    }

    #[tokio::main]
    async fn fetch_html(&self, url: &str) -> Result<Html, PhrackDownloaderlError> {
        let body = reqwest::get(url).await?.text().await?;
        let document = Html::parse_document(&body);

        Ok(document)
    }

    fn parse_issues(&self, document: &Html) -> Result<Vec<String>, PhrackDownloaderlError> {
        let selector = Selector::parse("a").unwrap();
        let mut issues = Vec::new();
        let re = Regex::new(r"/issues/(\d+)/").unwrap();

        for element in document.select(&selector) {
            if let Some(href) = element.value().attr("href") {
                if let Some(captures) = re.captures(href) {
                    issues.push(captures[1].to_string());
                }
            }
        }

        Ok(issues)
    }
}
