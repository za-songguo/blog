use yew::prelude::*;

#[derive(Debug, PartialEq, Properties)]
pub struct Props {
    pub title: AttrValue,
    pub children: Children,
}

/// 卡片
#[function_component(Card)]
pub fn card(props: &Props) -> Html {
    html! {
        <article class="card" style="margin: auto; margin-top: 5%; width: 80%;">
            <header>
                <h3>{ &props.title }</h3>
            </header>
            <footer>
                { for props.children.iter() }
            </footer>
        </article>
    }
}
