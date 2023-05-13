use gloo::net::http::Method;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
    app::Route,
    components::{article::preview::ArticlePreview, card::Card, container::AppContext},
    fetch,
    models::article::ArticlePreview as Preview,
};

#[function_component(Home)]
pub fn home() -> Html {
    // 通过 Callback 更改网页标题
    let context = use_context::<AppContext>().unwrap();

    let user = (*context.user).clone();

    context.set_title.emit("Home".into());

    let search_keyword = use_state(|| None);

    let loading = use_state(|| true);
    let articles = use_state(|| Err("".into()));

    // 搜索文章
    let search_article = {
        let search_keyword = search_keyword.clone();

        Callback::from(move |event: InputEvent| {
            // 获取输入框的值
            let keyword = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value()
                .trim()
                .to_owned();

            // 用户没有输入任何东西
            if keyword.is_empty() {
                // 展示所有文章
                search_keyword.set(None)
            } else {
                // 搜索
                search_keyword.set(Some(keyword))
            }
        })
    };

    {
        let search_keyword_cloned = search_keyword.clone();

        let loading = loading.clone();
        let articles = articles.clone();

        // 在 search_keyword 发生变化的时候重新获取数据
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let url = if let Some(keyword) = (*search_keyword).clone() {
                        format!("/api/article/search/{keyword}")
                    } else {
                        "/api/articles".into()
                    };

                    articles.set(fetch::fetch::<Vec<Preview>>(url, Method::GET, None, None).await);

                    loading.set(false);
                });
            },
            search_keyword_cloned,
        );
    }

    let navigator = use_navigator().unwrap();

    html! {
        <>
            <Card title={"文章"}>
                if let Ok(user) = user {
                    if user.is_admin {
                        <button style="margin-bottom: 1%;" onclick={Callback::from(move |_| navigator.push(&Route::NewArticle))}>{ "新增文章" }</button>
                    }
                }
                <input type="text" placeholder="搜索文章" oninput={search_article} style="margin-bottom: 1%;"/>
                if *loading {
                    <p>{ "Loading..." }</p>
                } else {
                    <ArticlePreview articles={(*articles).clone()}/>
                }
            </Card>
        </>
    }
}
