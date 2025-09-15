// SPDX-License-Identifier: MIT

use crate::models::article::Article;
use crate::models::issue::Issue;
use crate::phrack_downloader_error::PhrackDownloaderError;
use regex::Regex;
use scraper::{Html, Selector};

pub fn parse_issues(document: &Html) -> Result<Vec<Issue>, PhrackDownloaderError> {
    let selector = Selector::parse("a").unwrap();
    let mut issues = Vec::new();
    let re = Regex::new(r"/issues/(\d+)/").unwrap();

    for element in document.select(&selector) {
        if let Some(href) = element.value().attr("href") {
            if let Some(captures) = re.captures(href) {
                issues.push(captures[1].to_string().try_into()?);
            }
        }
    }

    Ok(issues)
}

pub fn parse_articles(
    document: &Html,
    issue: &Issue,
) -> Result<Vec<Article>, PhrackDownloaderError> {
    let selector = Selector::parse("a").unwrap();
    let mut articles = Vec::new();
    let re = Regex::new(&format!(r"/issues/{}/([\w-]+).txt", issue.issue_number)).unwrap();

    for element in document.select(&selector) {
        if let Some(href) = element.value().attr("href") {
            if let Some(captures) = re.captures(href) {
                articles.push(Article {
                    issue: issue.clone(),
                    article_number: captures[1].to_string().parse()?,
                    article_uri_path: href.to_string(),
                });
            }
        }
    }

    Ok(articles)
}
