use yew::prelude::*;

use crate::components::{
    article::article_preview::ArticlePreview, card::Card, container::AppContext,
};

#[function_component(Home)]
pub fn home() -> Html {
    // 通过 Callback 更改网页标题
    use_context::<AppContext>()
        .unwrap()
        .set_title
        .emit("Home".into());

    html! {
        <Card title={"文章"}>
            <ArticlePreview/>
        </Card>
    }
}
