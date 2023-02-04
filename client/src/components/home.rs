use yew::prelude::*;

use crate::components::{article::articles::ArticlePreview, card::Card};

#[function_component(Home)]
pub fn home() -> Html {
    // 通过 Callback 更改网页标题
    use_context::<Callback<String>>()
        .unwrap()
        .emit("Home".into());

    html! {
        <Card title={"文章"}>
            <ArticlePreview/>
        </Card>
    }
}
