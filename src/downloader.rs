// SPDX-License-Identifier: MIT
use crate::config::Config;
use crate::models::article::Article;
use crate::models::issue::Issue;
use crate::phrack::html_parser::{parse_articles, parse_issues};
use crate::phrack_downloader_error::PhrackDownloaderError;
use futures::stream::{self, StreamExt};
use reqwest;
use scraper::Html;
use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
    time::Duration,
};

pub struct Downloader {
    config: Config,
}

impl Downloader {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub async fn download_all_issues(&self, refresh: bool) -> Result<(), PhrackDownloaderError> {
        println!("Checking available issues");
        let issues_url = self.issues_url();
        let document = self.fetch_html(&issues_url).await?;
        let issues = parse_issues(&document)?;

        println!("Found {} issues", issues.len());
        for issue in issues {
            self.download_issue(&issue, refresh).await?
        }
        Ok(())
    }

    pub async fn download_issue(
        &self,
        issue: &Issue,
        refresh: bool,
    ) -> Result<(), PhrackDownloaderError> {
        let download_path = self
            .config
            .download_path()
            .join(issue.issue_number.to_string());

        println!("Downloading issue {} to {}", issue, download_path.display());
        if download_path.exists() && !refresh {
            println!(
                "Issue {} already downloaded at {}, skipping (use --refresh to re-download)",
                issue,
                download_path.display()
            );
            return Ok(());
        } else {
            if refresh && download_path.exists() {
                fs::remove_dir_all(&download_path)?;
            }
            fs::create_dir_all(&download_path)?;
        }

        let issue_articles_html = self.fetch_html(&self.issue_url(issue)).await?;
        let issue_articles = parse_articles(&issue_articles_html, issue)?;

        self.download_articles(&issue_articles).await?;

        Ok(())
    }

    async fn fetch_html(&self, url: &str) -> Result<Html, PhrackDownloaderError> {
        let body = self.fetch_url(url).await?;
        let document = Html::parse_document(&body);

        Ok(document)
    }
    async fn fetch_url(&self, url: &str) -> Result<String, PhrackDownloaderError> {
        let body = reqwest::get(url).await?.text().await?;

        Ok(body)
    }

    fn issues_url(&self) -> String {
        let archive_url = self.config.phrack_archive_url();

        format!("{}/issues/", archive_url)
    }

    fn issue_url(&self, issue: &Issue) -> String {
        let archive_url = self.config.phrack_archive_url();

        format!("{}/issues/{}/", archive_url, issue.issue_number)
    }
    fn article_url(&self, article_url: &Article) -> String {
        let archive_url = self.config.phrack_archive_url();

        format!("{}{}", archive_url, article_url.article_uri_path)
    }
    fn article_path(&self, article: &Article) -> PathBuf {
        let download_path = self.config.download_path();

        download_path.join(format!(
            "{}/{}.txt",
            article.issue.issue_number, article.article_number
        ))
    }

    async fn download_articles(&self, articles: &[Article]) -> Result<(), PhrackDownloaderError> {
        let stream = stream::iter(articles.into_iter().map(|article| async move {
            let body = self.fetch_url(&self.article_url(article)).await?;
            Ok::<_, PhrackDownloaderError>((article, body))
        }))
        .buffer_unordered(3);

        let stream = tokio_stream::StreamExt::chunks_timeout(stream, 3, Duration::from_secs(20));

        let chunks: Vec<_> = stream.collect().await;
        let results = chunks.into_iter().flatten();
        for r in results {
            match r {
                Ok((article, body)) => {
                    let path = self.article_path(article);
                    let mut file = File::create(path)?;
                    file.write_all(body.as_bytes())?;
                }
                Err(e) => {
                    eprintln!("Error downloading article: {}", e);
                }
            }
        }
        Ok(())
    }
}
