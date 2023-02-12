use gloo::net::http::{Method, Request};
use serde::Deserialize;

pub async fn fetch<T: for<'a> Deserialize<'a>>(
    url: String,
    method: Method,
    body: Option<String>,
) -> Result<T, String> {
    let resp = Request::new(&url).method(method);

    let resp = match body {
        Some(body) => resp.body(body),
        None => resp,
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
