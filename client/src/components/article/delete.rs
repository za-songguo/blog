use gloo::net::http::Method;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
    app::Route,
    components::{container::AppContext, modal::Modal},
    fetch,
};

#[derive(Debug, Properties, PartialEq)]
pub struct Props {
    pub article_id: u32,
}

#[function_component(DeleteArticle)]
pub fn delete_article(props: &Props) -> Html {
    use_context::<AppContext>()
        .unwrap()
        .set_title
        .emit("删除文章".into());

    let navigator = use_navigator().unwrap();

    let server_response = use_state(|| Err("".into()));

    let article_id = props.article_id;
    let delete_article = {
        let server_response = server_response.clone();

        Callback::from(move |_| {
            let server_response = server_response.clone();

            wasm_bindgen_futures::spawn_local(async move {
                server_response.set(
                    fetch::fetch_without_deserialize(
                        format!("/api/article/{}", article_id),
                        Method::DELETE,
                        None,
                        None,
                    )
                    .await,
                );
            });
        })
    };

    let footer_delete = {
        let navigator = navigator.clone();

        html! {
            <>
                <button onclick={delete_article} class="error">{ "确定删除" }</button>
                <button style="margin-left: 1%;" onclick={Callback::from(move |_| {
                    navigator.push(&Route::Home)
                })}>{ "取消" }</button>
            </>
        }
    };

    let footer_home = html! {
        <button onclick={Callback::from(move |_| {
            navigator.push(&Route::Home)
        })}>{ "回首页" }</button>
    };

    html! {
        <>
            <Modal title={"删除文章"} footer={footer_delete}>{ format!("确定要删除 ID 为 {} 的文章吗？", &props.article_id) }</Modal>

            if let Ok(message) = &*server_response {
                <Modal title={"服务器返回消息"} footer={footer_home}>{ message }</Modal>
            } else if let Err(e) = &*server_response {
                if !e.is_empty() {
                    <Modal title={"错误"}>{ e }</Modal>
                }
            }
        </>
    }
}
