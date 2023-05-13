use gloo::net::http::Method;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
    app::Route,
    components::{container::AppContext, markdown_editor::MarkdownEditor, modal::Modal},
    fetch,
    models::article::Article,
};

#[function_component(NewArticle)]
pub fn new_article() -> Html {
    let submit_response = use_state(|| Err("".into()));

    let submit = {
        let submit_response = submit_response.clone();

        Callback::from(move |article: Article| {
            let submit_response = submit_response.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let resp = fetch::fetch_without_deserialize(
                    "/api/article".to_string(),
                    Method::POST,
                    Some(serde_json::to_string(&article).unwrap()),
                    Some("application/json".to_string()),
                )
                .await;

                submit_response.set(resp);
            });
        })
    };

    use_context::<AppContext>()
        .unwrap()
        .set_title
        .emit("新增文章".into());

    let navigator = use_navigator().unwrap();

    // 返回首页按钮
    let footer = html! {
        <button onclick={Callback::from(move |_| {
            navigator.push(&Route::Home)
        })}>{ "回首页" }</button>
    };

    html! {
        <>
            <h1 style="margin-top: 5%; margin-left: 2%;">{ "新增文章" }</h1>
            <MarkdownEditor {submit}/>
            if let Ok(message) = &*submit_response {
                <Modal title={"服务器返回消息"} {footer}>{ message }</Modal>
            } else if let Err(e) = &*submit_response {
                if !e.is_empty() {
                    <Modal title={"错误"}>{ e }</Modal>
                }
            }
        </>
    }
}
