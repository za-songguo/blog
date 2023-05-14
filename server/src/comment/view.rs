use std::sync::Arc;

use ntex::web::types::{Json, Path, State};

use crate::{
    errors::CustomError,
    models::{comment::Comment, user::UserInfo},
    AppState,
};
/// 通过文章 ID 获取该文章的所有评论（包含发表评论的用户的信息）
pub async fn get_comments_for_article(
    article_id: Path<(u32,)>,
    state: State<Arc<AppState>>,
) -> Result<Json<Vec<Comment>>, CustomError> {
    let db_pool = &state.db_pool;

    let article_id = article_id.0;

    // 查找对应文章的所有评论，拿到它们的 user_id, content, date 和 users 表里相同 user_id(对应的是 users 表里的 id) 的记录的 name, avatar_url
    let comments = sqlx::query!(
        "SELECT comments.id, comments.user_id, comments.content, comments.date, users.name, users.avatar_url FROM comments JOIN users ON comments.user_id = users.id WHERE comments.article = $1", article_id as i32
    )
    .fetch_all(db_pool)
    .await?
    .iter()
    .map(|i| Comment {
        id: Some(i.id as u32),
        user: Some(UserInfo {
            id: i.user_id as u32,
            login: i.name.clone(),
            avatar_url: i.avatar_url.clone(),is_admin: i.user_id == 90502461,
        }),
        content: i.content.clone(),
        date: Some(i.date),
        article: None,
    })
    .collect::<Vec<Comment>>();

    Ok(Json(comments))
}
