use gloo::net::http::Method;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

use crate::{
    components::{container::AppContext, modal::Modal},
    fetch,
    models::comment::Comment,
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub article_id: u32,
    pub update_comments_list: Callback<()>,
}

#[function_component(NewComment)]
pub fn new_comment(props: &Props) -> Html {
    // 用来获取用户在文本框中输入的的评论内容
    let comment_content_ref = use_node_ref();

    let editor_is_visitable = use_state(|| false);

    let submit_response = use_state(|| Err("".into()));

    let show_editor = {
        let editor_is_visitable = editor_is_visitable.clone();
        Callback::from(move |_| {
            editor_is_visitable.set(!*editor_is_visitable);
        })
    };

    let article_id = props.article_id;
    let update_comments_list = props.update_comments_list.clone();

    let submit = {
        let submit_response = submit_response.clone();
        let comment_content_ref = comment_content_ref.clone();

        Callback::from(move |_| {
            let submit_response = submit_response.clone();
            let comment_content_ref = comment_content_ref.clone();

            // 由于这个 Callback 在 submit 外面的代码中不需要再用，所以只需要在这里 clone 一次
            let update_comments_list = update_comments_list.clone();

            let comment = Comment {
                id: None,
                date: None,
                user: None,
                article: Some(article_id),
                content: comment_content_ref
                    .cast::<HtmlTextAreaElement>()
                    .unwrap()
                    .value(),
            };
            wasm_bindgen_futures::spawn_local(async move {
                let resp = fetch::fetch_without_deserialize(
                    "/api/comment".to_string(),
                    Method::POST,
                    Some(serde_json::to_string(&comment).unwrap()),
                    Some("application/json".to_string()),
                )
                .await;

                // 服务端返回响应（成功或失败）时更新评论列表
                if resp.is_ok() {
                    update_comments_list.emit(());
                }

                submit_response.set(resp);
            });
        })
    };

    // 用户信息
    let user = use_context::<AppContext>().unwrap().user;

    html! {
        <>
            if (*user).is_ok() {
                <button onclick={show_editor} style="display: block; margin-bottom: 1%;">{ "写评论" }</button>
                if *editor_is_visitable {
                    <form style="margin-bottom: 1%;">
                        // comment_content_ref 就是对这个 textarea 的引用，可以通过这个 NodeRef 获取到这个元素，读取里面的值（只读访问）
                        <textarea placeholder=":)" ref={comment_content_ref}/>
                        <input type="button" value="提交" onclick={submit} class="button"/>
                    </form>
                }

                if let Ok(message) = &*submit_response {
                    <Modal title={"服务器返回消息"}>{ message }</Modal>
                } else if let Err(e) = &*submit_response {
                    if !e.is_empty() {
                        <Modal title={"错误"}>{ e }</Modal>
                    }
                }
            } else {
                <button disabled=true style="display: block; margin-bottom: 1%;">{ "登录后可以发表评论" }</button>
            }
        </>
    }
}
