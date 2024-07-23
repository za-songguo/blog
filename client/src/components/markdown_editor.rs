use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::{models::article::Article, utils};

#[derive(Debug, Properties, PartialEq)]
pub struct Props {
    // 可以往这个组件里传 title 和 content，修改文章的部分会用到
    #[prop_or("".into())]
    pub title: AttrValue,
    #[prop_or("".into())]
    pub content: AttrValue,
    pub submit: Callback<Article>,
}

#[function_component(MarkdownEditor)]
pub fn markdown_editor(props: &Props) -> Html {
    let title = use_state(|| props.title.clone());
    let content = use_state(|| props.content.clone());

    let update_title_preview = {
        let title = title.clone();
        Callback::from(move |event: InputEvent| {
            let value = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();

            title.set(value.into())
        })
    };

    let update_content_preview = {
        let content = content.clone();
        Callback::from(move |event: InputEvent| {
            let value = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();

            content.set(value.into())
        })
    };

    let call_submit = {
        let title = title.clone();
        let content = content.clone();
        let submit = props.submit.clone();

        Callback::from(move |_| {
            let article = Article {
                id: None,
                title: (*title).clone().to_string(),
                content: (*content).clone().to_string(),
                date: None,
            };

            submit.emit(article);
        })
    };

    html! {
        <>
        <form style="width: 45%; float: left; margin-left: 2%;">
            // value 的值必须用 state 的（由 props 提供默认值，修改 state 不会影响到 props），不能直接用 props 里的，否则用户输入的时候 value 发生变化，导致父组件和子组件的 props 之间形成双向数据流（原本是单向数据流：数据由父组件提供给子组件，从上往下流动，而子组件只能通过 props 获取数据，不能修改 props 里的数据（会影响到父组件，造成双向数据绑定，导致一些不可预期的问题））
            <input type="text" placeholder="文章标题" style="margin-bottom: 1%;" oninput={update_title_preview} value={&*title}/>
            // From ChatGPT:
            // 在HTML中，<textarea>元素不同于其他表单元素（如<input>），它本身就包含了文本内容。这意味着<textarea>标签内的任何文本都会被解释为该元素的初始值，并且无法通过插值语法来更新该元素的值。相反，我们需要使用 value 属性来控制<textarea>的文本内容。这个属性类似于<input>元素中的 value 属性，可以让我们在 Yew 中更新<textarea>的值，而不是像在HTML中那样直接插入文本
            <textarea style="margin-bottom: 1%;" placeholder="文章内容（支持使用 Markdown 编辑）" oninput={update_content_preview} value={&*content}/>
            <input type="button" value="提交" onclick={call_submit} class="button"/>
        </form>

        <div style="width: 45%; float: right; margin-left: 2%;">
            <article class="card" style="margin: auto; width: 80%;">
                <header>
                    <h3>{ (*title).clone() }</h3>
                </header>
                <footer>
                    { utils::convert_markdown_to_html((*content).to_string()) }
                </footer>
            </article>
        </div>
        </>
    }
}
