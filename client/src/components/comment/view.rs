use gloo::net::http::Method;
use yew::prelude::*;

use crate::{
    components::{card::Card, comment::new::NewComment},
    fetch,
    models::comment::Comment,
};

#[derive(PartialEq, Properties)]
pub struct Props {
    pub article_id: u32,
}

/// 查看该文章的所有评论
#[function_component(Comments)]
pub fn comments(props: &Props) -> Html {
    let loading = use_state(|| true);
    let comments = use_state(|| Err("".into()));

    let article_id = props.article_id;

    let update_comments_list = {
        let loading = loading.clone();
        let comments = comments.clone();

        Callback::from(move |_| {
            let loading = loading.clone();
            let comments = comments.clone();

            wasm_bindgen_futures::spawn_local(async move {
                comments.set(
                    fetch::fetch::<Vec<Comment>>(
                        format!("/api/comment/{article_id}"),
                        Method::GET,
                        None,
                        None,
                    )
                    .await,
                );
                loading.set(false);
            })
        })
    };

    // 获取评论
    {
        let update_comments_list = update_comments_list.clone();

        use_effect_with_deps(move |_| update_comments_list.emit(()), ());
    }

    html! {
         if *loading {
            <Card title={"Loading..."}>
                <p>{ "马上就好......" }</p>
            </Card>
        } else {
            { content(&comments, props.article_id, update_comments_list) }
        }
    }
}

/// 遍历并显示评论列表
fn content(
    comments: &Result<Vec<Comment>, String>,
    article_id: u32,
    update_comments_list: Callback<()>,
) -> Html {
    html! {
        <Card title="评论">
            <NewComment {article_id} {update_comments_list}/>
            {
                match comments {
                    Ok(comments) if comments.is_empty() => html!{
                        <p>{ "似乎还没有评论" }</p>
                    },
                    Ok(comments) => html!{
                            {
                                comments
                                    .iter()
                                    .map(|i| {
                                        html!{
                                            // 这里用 unwrap 是因为服务端返回的数据是一定有 user 和 date 的
                                            // 你也可以选择把 Comment 拆成 ViewComment, DeleteComment 等
                                            <article class="card" key={i.id.unwrap()}>
                                                <header>
                                                    <img src={i.user.as_ref().unwrap().avatar_url.clone()} style="width: 2%; border-radius: 50%; vertical-align: middle; margin-right: 1%;"/>
                                                    <h3>{ format!("{} （{}）", i.user.as_ref().unwrap().login, i.date.as_ref().unwrap()) }</h3>
                                                </header>
                                                <footer>
                                                    { &i.content }
                                                </footer>
                                            </article>
                                        }
                                    })
                                    .collect::<Html>()
                            }

                    },
                    Err(e) => html! {
                        { e }
                    },
                }
            }
        </Card>
    }
}
