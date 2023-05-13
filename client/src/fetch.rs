use gloo::net::http::{Method, Request};
use serde::Deserialize;

/// 发送请求
pub async fn fetch<T: for<'a> Deserialize<'a>>(
    url: String,
    method: Method,
    body: Option<String>,
    content_type: Option<String>,
) -> Result<T, String> {
    let req = Request::new(&url).method(method);

    let req = match content_type {
        Some(content_type) => req.header("Content-Type", &content_type),
        None => req,
    };

    let resp = match body {
        Some(body) => req.body(body),
        None => req,
    }
    .send()
    .await;

    match resp {
        // HTTP status code 2xx (成功)
        Ok(r) if r.status().to_string().starts_with('2') => match r.json::<T>().await {
            Ok(r) => Ok(r),
            Err(e) => Err(format!("无法解析响应：{e}")),
        },
        // 服务器返回了错误
        Ok(r) => Err(format!(
            "{} ({} {})",
            r.text().await.unwrap(),
            r.status(),
            r.status_text()
        )),
        Err(e) => Err(format!("无法发送请求：{e}")),
    }
}

/// 发送请求
/// 不进行反序列化（用于服务端返回成功消息的场景）
pub async fn fetch_without_deserialize(
    url: String,
    method: Method,
    body: Option<String>,
    content_type: Option<String>,
) -> Result<String, String> {
    let req = Request::new(&url).method(method);

    let req = match content_type {
        Some(content_type) => req.header("Content-Type", &content_type),
        None => req,
    };

    let resp = match body {
        Some(body) => req.body(body),
        None => req,
    }
    .send()
    .await;

    match resp {
        // HTTP status code 2xx (成功)
        Ok(r) if r.status().to_string().starts_with('2') => Ok(r.text().await.unwrap()),
        // 服务器返回了错误
        Ok(r) => Err(format!(
            "{} ({} {})",
            r.text().await.unwrap(),
            r.status(),
            r.status_text()
        )),
        Err(e) => Err(format!("无法发送请求：{e}")),
    }
}
