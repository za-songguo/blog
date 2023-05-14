use std::sync::Arc;

use ntex::web::types::{Json, State};

use crate::{
    errors::CustomError,
    models::user::{User, UserInfo},
    AppState,
};

/// 获取用户信息（需要用户在本站登录过）
/// 需要用户权限
pub async fn get_user_info(
    user: User,
    state: State<Arc<AppState>>,
) -> Result<Json<UserInfo>, CustomError> {
    let db_pool = &state.db_pool;

    let user_id = user.id;

    let user_info = sqlx::query!("SELECT * FROM users WHERE id = $1", user_id as i32)
        .fetch_one(db_pool)
        .await?;

    Ok(Json(UserInfo {
        id: user_info.id as u32,
        login: user_info.name,
        avatar_url: user_info.avatar_url,
        is_admin: user_info.id == 90502461,
    }))
}
