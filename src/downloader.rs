// SPDX-License-Identifier: MIT
use crate::config::Config;
use crate::models::article::Article;
use crate::models::issue::Issue;
use crate::phrack::html_parser::{parse_articles, parse_issues};
use crate::phrack_downloader_error::PhrackDownloaderError;
use futures::stream::{self, StreamExt};
use reqwest;
use scraper::Html;
use std::vec;
use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
    time::Duration,
};

pub struct Downloader {
    config: Config,
}

#[derive(Debug, Clone)]
struct DownloadJob {
    source_url: String,
    destination_path: PathBuf,
}

impl Into<Vec<DownloadJob>> for DownloadJob {
    fn into(self) -> Vec<DownloadJob> {
        vec![self]
    }
}

impl Downloader {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub async fn download_all_issues(&self, refresh: bool) -> Result<(), PhrackDownloaderError> {
        println!("Checking available issues");
        println!("Fetching information for issues");

        let document = self.fetch_html(&self.issues_url()).await?;
        let issues = parse_issues(&document)?;

        println!("Found {} issues", issues.len());

        let mut download_jobs = vec![];

        for issue in issues {
            download_jobs.extend(self.issue_articles_to_dl_jobs(&issue, refresh).await?);
        }

        self.download_jobs(&download_jobs).await?;

        Ok(())
    }

    pub async fn download_issue(
        &self,
        issue: &Issue,
        refresh: bool,
    ) -> Result<(), PhrackDownloaderError> {
        println!("Fetching information for issue");

        let download_jobs = self.issue_articles_to_dl_jobs(&issue, refresh).await?;

        self.download_jobs(&download_jobs).await?;

        Ok(())
    }

    async fn fetch_issue(&self, issue: &Issue) -> Result<Issue, PhrackDownloaderError> {
        let issue_articles_html = self.fetch_html(&self.issue_url(issue)).await?;
        println!("Fetching metadata for issue {}", issue);

        Ok(parse_articles(&issue_articles_html, issue)?)
    }

    async fn fetch_url(&self, url: &str) -> Result<String, PhrackDownloaderError> {
        let body = reqwest::get(url).await?.text().await?;

        Ok(body)
    }

    async fn fetch_html(&self, url: &str) -> Result<Html, PhrackDownloaderError> {
        let body = self.fetch_url(url).await?;
        let document = Html::parse_document(&body);

        Ok(document)
    }

    fn issues_url(&self) -> String {
        format!("{}/issues/", self.config.phrack_archive_url())
    }

    fn issue_url(&self, issue: &Issue) -> String {
        format!(
            "{}/issues/{}/",
            self.config.phrack_archive_url(),
            issue.issue_number
        )
    }
    fn article_url(&self, article_url: &Article) -> String {
        format!(
            "{}{}",
            self.config.phrack_archive_url(),
            article_url.article_uri_path
        )
    }
    fn issue_path(&self, issue: &Issue) -> PathBuf {
        self.config
            .download_path()
            .join(issue.issue_number.to_string())
    }

    fn article_path(&self, article: &Article) -> PathBuf {
        self.issue_path(&article.issue)
            .join(format!("{}.txt", article.article_number))
    }

    async fn issue_articles_to_dl_jobs(
        &self,
        issue: &Issue,
        refresh: bool,
    ) -> Result<Vec<DownloadJob>, PhrackDownloaderError> {
        let mut jobs = vec![];

        if self.continue_issue_download(&issue, refresh)? {
            let issue = self.fetch_issue(issue).await?;

            issue.articles.iter().for_each(|article| {
                jobs.push(DownloadJob {
                    source_url: self.article_url(article),
                    destination_path: self.article_path(article),
                });
            });

            if let Some(phrack_pdf) = issue.phrack_pdf.clone() {
                jobs.push(DownloadJob {
                    source_url: format!("{}{}", self.issue_url(&issue), phrack_pdf.filename),
                    destination_path: self
                        .issue_path(&issue)
                        .join(phrack_pdf.filename.to_string()),
                });
            }
        } else {
            let issue_path = self.issue_path(issue);
            self.print_skip_message(&issue, &issue_path);
        }

        Ok(jobs)
    }
    fn continue_issue_download(
        &self,
        issue: &Issue,
        refresh: bool,
    ) -> Result<bool, PhrackDownloaderError> {
        let issue_path = self.issue_path(issue);

        if issue_path.exists() && !refresh {
            Ok(false)
        } else {
            if refresh && issue_path.exists() {
                fs::remove_dir_all(&issue_path)?;
            }
            fs::create_dir_all(&issue_path)?;

            Ok(true)
        }
    }

    fn print_skip_message(&self, issue: &Issue, issue_path: &PathBuf) {
        println!(
            "Issue {} already downloaded at {}, skipping (use --refresh to re-download)",
            issue,
            issue_path.display()
        );
    }

    async fn download_jobs(&self, jobs: &Vec<DownloadJob>) -> Result<(), PhrackDownloaderError> {
        println!("Starting to process {} download jobs", jobs.len());
        let start_time = std::time::Instant::now();

        let stream = stream::iter(jobs.into_iter().map(|job| async move {
            let body = self.fetch_url(&job.source_url).await?;
            Ok::<_, PhrackDownloaderError>((job, body))
        }))
        .buffer_unordered(3);

        let stream = tokio_stream::StreamExt::chunks_timeout(stream, 3, Duration::from_secs(20));

        let chunks: Vec<_> = stream.collect().await;
        let results = chunks.into_iter().flatten();
        for r in results {
            match r {
                Ok((job, body)) => {
                    let mut file = File::create(&job.destination_path)?;
                    file.write_all(body.as_bytes())?;
                }
                Err(e) => {
                    eprintln!("Error in download job: {}", e);
                }
            }
        }
        println!("Download done in {:.2?}", start_time.elapsed());
        Ok(())
    }
}
