use gloo::net::http::Method;
use pulldown_cmark::{html, Options, Parser};
use web_sys::Node;
use yew::prelude::*;

use crate::{
    components::{card::Card, container::AppContext},
    fetch,
    models::article::Article,
};

#[derive(Debug, PartialEq, Eq, Properties)]
pub struct Props {
    pub article_id: u32,
}

/// 查看单篇文章，支持 Markdown
#[function_component(ArticleViewer)]
pub fn article_viewer(props: &Props) -> Html {
    let loading = use_state(|| true);
    let article = use_state(|| Err("".into()));

    // 这个变量的声明得放在 use_effect_with_deps 外面，否则就会遇到生命周期问题（闭包和future: 'static）
    let article_id = props.article_id;

    {
        let loading = loading.clone();
        let article = article.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    article.set(
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

    let title = if let Ok(article) = (*article).clone() {
        article.title
    } else {
        "文章".into()
    };

    // 设置网页标题
    use_context::<AppContext>()
        .unwrap()
        .set_title
        .emit(title.clone());

    html! {
        if *loading {
            <Card title={"Loading..."}>
                <p>{ "马上就好......" }</p>
            </Card>
        } else {
            <Card {title}>
                {
                    match &*article {
                        Ok(article) => convert_markdown_to_html(article),
                        Err(e) => html! { <p>{ e }</p> }
                    }
                }
            </Card>
        }
    }
}

/// 解析 Markdown，转为 HTML
fn convert_markdown_to_html(article: &Article) -> Html {
    let article_content = &article.content;

    // Set up options and parser
    let mut options = Options::empty();

    // Enable all the features
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(article_content, options);

    let mut markdown_html = String::new();
    html::push_html(&mut markdown_html, parser);

    let div_wrapper = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("div")
        .unwrap();

    // 把解析的 HTML 放进去
    div_wrapper.set_inner_html(&markdown_html);
    let node: Node = div_wrapper.into();

    Html::VRef(node)
}
