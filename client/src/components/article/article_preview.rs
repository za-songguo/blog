use gloo::net::http::Method;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{app::Route, fetch::fetch, models::article::ArticlePreview as Preview};

/// 查看所有文章（预览）
#[function_component(ArticlePreview)]
pub fn article_preview() -> Html {
    let loading = use_state(|| true);
    let articles = use_state(|| Err("".into()));

    // 用于跳转到其他路由（其他组件）
    let navigator = use_navigator().unwrap();

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
            { content(navigator,(*articles).clone()) }
        }
    }
}

/// 生成 HTML
fn content(navigator: Navigator, articles: Result<Vec<Preview>, String>) -> Html {
    let jump = |navigator: Navigator, article_id| {
        Callback::from(move |_| {
            // 查看对应的文章
            navigator.push(&Route::ArticleViewer { article_id })
        })
    };

    match articles {
        Ok(articles) => articles
            .iter()
            .map(|i| {
                html! {
                    // 因为 jump 会把 navigator 移动（move），这样就无法在迭代器中使用了（因为在上一次迭代中这个变量已经被 move 了，所以在接下来的迭代中就无法继续使用了），所以要 clone 一下
                    <article class="card" onclick={jump(navigator.clone(), i.id)} key={i.id}>
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
