use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PostMeta {
    pub title: String,
    pub date: String,
    pub slug: String,
    pub tags: Option<Vec<String>>,
    pub categories: Option<Vec<String>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Post {
    pub meta: PostMeta,
    pub content: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub title: String,
    pub base_url: String,
    pub description: Option<String>,
    pub posts_per_page: Option<usize>,
}
