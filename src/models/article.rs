// SPDX-License-Identifier: MIT
use crate::models::issue::Issue;

#[derive(Debug, Clone)]
pub struct Article {
    pub issue: Issue,
    pub article_number: u32,
    pub article_uri_path: String,
}
