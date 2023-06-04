use yew::prelude::*;

use crate::{
    components::{card::Card, container::AppContext},
    constants,
};

#[function_component(Login)]
pub fn login() -> Html {
    // 设置网页标题
    use_context::<AppContext>()
        .unwrap()
        .set_title
        .emit("登录".into());

    html! {
        <Card title={"登录"}>
            <a class="button mainButton" href={format!("https://github.com/login/oauth/authorize?client_id={}", constants::CLIENT_ID)}>{ "使用 Github 登录" }</a>
        </Card>
    }
}
