use gloo::{console::log, net::http::Method};
use yew::prelude::*;

use crate::{fetch::fetch, models::article::ArticlePreview as Preview};

#[function_component(ArticlePreview)]
pub fn article_preview() -> Html {
    let loading = use_state(|| true);
    let articles = use_state(|| Err("".into()));

    {
        let loading = loading.clone();
        let articles = articles.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    articles.set(
                        fetch::<Vec<Preview>>("/api/articles".into(), Method::GET, None).await,
                    );
                    loading.set(false);
                });
            },
            (),
        );
    }

    html! {
        if *loading {
            <p>{ "Loading..." }</p>
        } else {
            { content((*articles).clone()) }
        }
    }
}

/// 生成 HTML
fn content(articles: Result<Vec<Preview>, String>) -> Html {
    let jump = |id| Callback::from(|_| log!("Clicked"));

    match articles {
        Ok(articles) => articles
            .iter()
            .map(|i| {
                html! {
                    <article class="card" onclick={jump(i.id)} key={i.id}>
                        <header>
                            <h3>{ &i.title }</h3>
                            <span style="color: grey;">{ &i.date }</span>
                        </header>
                    </article>
                }
            })
            .collect::<Html>(),
        Err(e) => html! {
            <p>{ e }</p>
        },
    }
}
