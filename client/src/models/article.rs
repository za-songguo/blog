use serde::{Deserialize, Serialize};

/// 文章的详细信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Article {
    pub id: Option<u32>,
    pub title: String,
    pub content: String,
    pub date: Option<String>,
}

/// 文章预览
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct ArticlePreview {
    pub id: u32,
    pub title: String,
    pub date: String,
}
