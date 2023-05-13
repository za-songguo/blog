use ntex::web::types::{Json, Path, State};
use std::sync::Arc;

use crate::{
    errors::CustomError,
    models::article::{Article, ArticlePreview},
    AppState,
};

/// 获取文章预览
pub async fn get_articles_preview(
    state: State<Arc<AppState>>,
) -> Result<Json<Vec<ArticlePreview>>, CustomError> {
    let db_pool = &state.db_pool;

    let articles = sqlx::query!("SELECT id, title, date FROM articles")
        .fetch_all(db_pool)
        .await?
        .iter()
        .map(|i| ArticlePreview {
            id: i.id as u32,
            title: i.title.clone(),
            date: i.date,
        })
        .collect();

    Ok(Json(articles))
}

/// 通过 ID 获取单篇文章
pub async fn get_article(
    id: Path<(u32,)>,
    state: State<Arc<AppState>>,
) -> Result<Json<Article>, CustomError> {
    let db_pool = &state.db_pool;

    let article = sqlx::query!(
        "SELECT title, content, date FROM articles WHERE id = $1",
        id.0 as i32
    )
    .fetch_one(db_pool)
    .await?;

    let article = Article {
        id: None,
        title: article.title.clone(),
        content: article.content.clone(),
        date: Some(article.date),
    };

    Ok(Json(article))
}
