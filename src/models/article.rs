// SPDX-License-Identifier: MIT
use crate::models::issue::Issue;
use crate::strict_string::PhrackArticleUrl;

#[derive(Debug, Clone)]
pub struct Article {
    pub issue: Issue,
    pub article_number: u32,
    pub article_uri_path: PhrackArticleUrl,
}
