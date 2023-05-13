use gloo::net::http::Method;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
    app::Route,
    components::{
        card::Card, container::AppContext, markdown_editor::MarkdownEditor, modal::Modal,
    },
    fetch,
    models::article::Article,
};

#[derive(Debug, Properties, PartialEq)]
pub struct Props {
    pub article_id: u32,
}

#[function_component(EditArticle)]
pub fn edit_article(props: &Props) -> Html {
    use_context::<AppContext>()
        .unwrap()
        .set_title
        .emit("修改文章".into());

    let loading = use_state(|| true);
    let old_article = use_state(|| Err("".into()));

    // 获取之前写的文章数据
    let article_id = props.article_id;
    {
        let loading = loading.clone();
        let old_article = old_article.clone();

        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    old_article.set(
                        fetch::fetch::<Article>(
                            format!("/api/article/{article_id}"),
                            Method::GET,
                            None,
                            None,
                        )
                        .await,
                    );
                    loading.set(false);
                });
            },
            (),
        );
    }

    let submit_response = use_state(|| Err("".into()));

    let submit = {
        let submit_response = submit_response.clone();

        Callback::from(move |article: Article| {
            let submit_response = submit_response.clone();

            // MarkdownEditor 传上来的 Article 是没有 id 的，而修改文章需要 id，所以我们手动给它加上 id,其他部分不变
            let article = Article {
                id: Some(article_id),
                ..article
            };

            wasm_bindgen_futures::spawn_local(async move {
                let resp = fetch::fetch_without_deserialize(
                    "/api/article".to_string(),
                    Method::PUT,
                    Some(serde_json::to_string(&article).unwrap()),
                    Some("application/json".to_string()),
                )
                .await;

                submit_response.set(resp);
            });
        })
    };

    let navigator = use_navigator().unwrap();

    // 返回首页按钮
    let footer = html! {
        <button onclick={Callback::from(move |_| {
            navigator.push(&Route::Home)
        })}>{ "回首页" }</button>
    };

    html! {
        <>
            <h1 style="margin-top: 5%; margin-left: 2%;">{ "修改文章" }</h1>
            if *loading {
                <Card title={"Loading..."}>
                    <p>{ "马上就好......" }</p>
                </Card>
            } else if let Ok(article) = &*old_article {
                <MarkdownEditor {submit} title={article.title.clone()} content={article.content.clone()}/>
                if let Ok(message) = &*submit_response {
                    <Modal title={"服务器返回消息"} {footer}>{ message }</Modal>
                } else if let Err(e) = &*submit_response {
                    if !e.is_empty() {
                        <Modal title={"错误"}>{ e }</Modal>
                    }
                }
            } else if let Err(e) = &*old_article {
                <Modal title={"错误"}>{ e }</Modal>
            }
        </>
    }
}
