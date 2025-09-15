// SPDX-License-Identifier: MIT

use crate::models::article::Article;
use crate::models::issue::Issue;
use crate::models::phrack_pdf::PhrackPdf;
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

pub fn parse_articles(document: &Html, issue: &Issue) -> Result<Issue, PhrackDownloaderError> {
    let selector = Selector::parse("a").unwrap();
    let mut articles = Vec::new();
    let re = Regex::new(&format!(
        r"/issues/{}/([\w-]+).(txt|pdf)",
        issue.issue_number
    ))
    .unwrap();
    let mut ready_made_pdf = None;

    for element in document.select(&selector) {
        if let Some(href) = element.value().attr("href") {
            if let Some(captures) = re.captures(href) {
                if &captures[2] == "txt" {
                    articles.push(Article {
                        issue: issue.clone(),
                        article_number: captures[1].to_string().parse()?,
                        article_uri_path: href.to_string().into(),
                    });
                }
                if &captures[2] == "pdf" {
                    if captures[1].starts_with("phrack-") && &captures[2] == "pdf" {
                        ready_made_pdf = Some(PhrackPdf {
                            filename: format!("{}.{}", &captures[1], &captures[2]),
                        });
                    } else {
                        println!("Unrecognized PDF file: {}", &captures[1]);
                    }
                }
            }
        }
    }

    Ok(Issue {
        issue_number: issue.issue_number,
        phrack_pdf: ready_made_pdf,
        articles: articles.clone(),
        ..issue.clone()
    })
}
