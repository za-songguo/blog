use std::sync::Arc;

use ntex::web::{
    types::{Json, State},
    HttpResponse,
};

use crate::{
    errors::CustomError,
    models::{article::Article, user::Admin},
    AppState,
};

/// 新增文章
/// 需要管理员权限
pub async fn new_article(
    _: Admin,
    article: Json<Article>,
    state: State<Arc<AppState>>,
) -> Result<HttpResponse, CustomError> {
    let db_pool = &state.db_pool;

    sqlx::query!(
        "INSERT INTO articles (title, content) VALUES ($1, $2)",
        article.title,
        article.content
    )
    .execute(db_pool)
    .await?;

    Ok(HttpResponse::Created().body("新增文章成功！"))
}
