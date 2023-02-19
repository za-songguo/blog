use yew::prelude::*;

use crate::components::{card::Card, container::AppContext};

#[function_component(Login)]
pub fn login() -> Html {
    // 设置网页标题
    use_context::<AppContext>()
        .unwrap()
        .set_title
        .emit("登录".into());

    html! {
        <Card title={"登录"}>
            <a class="button mainButton" href="https://github.com/login/oauth/authorize?client_id=1c509a9cd47635f8a78d">{ "使用 Github 登录" }</a>
        </Card>
    }
}
