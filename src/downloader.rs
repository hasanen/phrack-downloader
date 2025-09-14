// SPDX-License-Identifier: MIT
use crate::config::Config;
use crate::phrack_downloader_error::PhrackDownloaderError;
use futures::stream::{self, StreamExt};
use regex::Regex;
use reqwest;
use scraper::{Html, Selector};
use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
    time::Duration,
};

pub struct Downloader {
    config: Config,
}
#[derive(Debug)]
struct ArticleUrl {
    issue_number: u32,
    article_number: u32,
    article_uri_path: String,
}
impl Downloader {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub async fn download_all_issues(&self, refresh: bool) -> Result<(), PhrackDownloaderError> {
        println!("Checking available issues");
        let issues_url = self.issues_url();
        let document = self.fetch_html(&issues_url).await?;
        let issues = self.parse_issues(&document)?;

        println!("Found {} issues", issues.len());
        for issue_number in issues {
            self.download_issue(issue_number.parse()?, refresh).await?
        }
        Ok(())
    }

    pub async fn download_issue(
        &self,
        issue_number: u32,
        refresh: bool,
    ) -> Result<(), PhrackDownloaderError> {
        // Placeholder for downloading a specific issue
        let download_path = PathBuf::from(
            self.config
                .get_value(&crate::config::ConfigKey::DownloadPath),
        )
        .join(issue_number.to_string());

        println!(
            "Downloading issue {} to {}",
            issue_number,
            download_path.display()
        );
        if download_path.exists() && !refresh {
            println!(
                "Issue {} already downloaded at {}, skipping (use --refresh to re-download)",
                issue_number,
                download_path.display()
            );
            return Ok(());
        } else {
            if refresh && download_path.exists() {
                fs::remove_dir_all(&download_path)?;
            }
            fs::create_dir_all(&download_path)?;
        }

        let issue_articles_html = self.fetch_html(&self.issue_url(issue_number)).await?;
        let issue_articles = self.parse_articles(&issue_articles_html, issue_number)?;

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

    fn parse_issues(&self, document: &Html) -> Result<Vec<String>, PhrackDownloaderError> {
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

    fn parse_articles(
        &self,
        document: &Html,
        issue_number: u32,
    ) -> Result<Vec<ArticleUrl>, PhrackDownloaderError> {
        let selector = Selector::parse("a").unwrap();
        let mut articles = Vec::new();
        let re = Regex::new(&format!(r"/issues/{}/([\w-]+).txt", issue_number)).unwrap();

        for element in document.select(&selector) {
            if let Some(href) = element.value().attr("href") {
                if let Some(captures) = re.captures(href) {
                    articles.push(ArticleUrl {
                        issue_number,
                        article_number: captures[1].to_string().parse()?,
                        article_uri_path: href.to_string(),
                    });
                }
            }
        }

        Ok(articles)
    }

    fn issues_url(&self) -> String {
        let archive_url = self
            .config
            .get_value(&crate::config::ConfigKey::PhrackArchiveUrl);

        format!("{}/issues/", archive_url)
    }

    fn issue_url(&self, issue_number: u32) -> String {
        let archive_url = self
            .config
            .get_value(&crate::config::ConfigKey::PhrackArchiveUrl);

        format!("{}/issues/{}/", archive_url, issue_number)
    }
    fn article_url(&self, article_url: &ArticleUrl) -> String {
        let archive_url = self
            .config
            .get_value(&crate::config::ConfigKey::PhrackArchiveUrl);

        format!("{}{}", archive_url, article_url.article_uri_path)
    }
    fn article_path(&self, article: &ArticleUrl) -> PathBuf {
        let download_path = PathBuf::from(
            self.config
                .get_value(&crate::config::ConfigKey::DownloadPath),
        );

        download_path.join(format!(
            "{}/{}.txt",
            article.issue_number, article.article_number
        ))
    }

    async fn download_articles(
        &self,
        articles: &[ArticleUrl],
    ) -> Result<(), PhrackDownloaderError> {
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
