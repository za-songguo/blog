use cookie::{time::Duration, Cookie};
use ntex::{
    http::Response,
    web::{
        types::{Json, State},
        Responder,
    },
};
use reqwest::Client;
use std::sync::Arc;

use crate::{
    constants,
    errors::CustomError,
    models::user::{AccessToken, GithubUserInfo, Login},
    AppState,
};

/// 接收传过来的 code，获取 access_token，得到用户数据并存进数据库
pub async fn github_login(
    code: Json<Login>,
    state: State<Arc<AppState>>,
) -> Result<impl Responder, CustomError> {
    let code = &code.code;

    // HTTP client
    let client = Client::new();

    // 获取 access_token
    // 把 Accept 设置为 json，让 Github 的 API 给我们返回 JSON 格式的数据
    let access_token = client
        .post(format!(
            "https://github.com/login/oauth/access_token?client_id={}&client_secret={}&code={code}",
            constants::CLIENT_ID,
            constants::CLIENT_SECRET
        ))
        .header("Accept", "application/json")
        .send()
        .await;

    let access_token = match access_token {
        Ok(r) => match r.json::<AccessToken>().await {
            Ok(r) => r.access_token,
            Err(_) => {
                return Err(CustomError::AuthFailed(
                    "code 是无效的（可能已经过期），请重新使用 Github 登录".into(),
                ))
            }
        },
        Err(_) => {
            return Err(CustomError::InternalServerError(
                "无法获取 access_token，请重试".into(),
            ));
        }
    };

    let user_info = client
        .get("https://api.github.com/user")
        .bearer_auth(access_token.clone())
        // Github 的 API 要求我们设置 UA
        .header("User-Agent", "za-songguo")
        .send()
        .await;

    let user_info = match user_info {
        Ok(r) => r.json::<GithubUserInfo>().await.unwrap(),
        Err(_) => {
            return Err(CustomError::InternalServerError(
                "无法获取 Github 用户信息，请重试".into(),
            ));
        }
    };

    // 设置 cookie，这样用户就不需要重复登录了
    let mut cookie = Cookie::new("ACCESS_TOKEN", access_token);
    cookie.set_path("/");
    cookie.set_max_age(Duration::days(7));
    cookie.set_http_only(true);

    // 把用户信息存进数据库
    let db_pool = &state.db_pool;

    // 如果已经有一条相同 ID 的记录就更新，否则就新增
    sqlx::query!(
        "INSERT INTO users (id, name, avatar_url) VALUES ($1, $2, $3) ON CONFLICT (id) DO UPDATE SET name = $2, avatar_url = $3",
        user_info.id as i32,
        user_info.login,
        user_info.avatar_url
    )
    .execute(db_pool)
    .await?;

    let mut response = Response::Ok().body(format!("Hi, {}!", user_info.login));

    // 忽略错误
    let _ = response.add_cookie(&cookie);

    Ok(response)
}
