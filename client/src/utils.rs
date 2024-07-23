use pulldown_cmark::{html, Options, Parser};
use web_sys::Node;
use yew::prelude::*;

/// 解析 Markdown，转为 HTML
pub fn convert_markdown_to_html(markdown: String) -> Html {
    // Set up options and enable all flags
    let options = Options::all();

    let parser = Parser::new_ext(&markdown, options);

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
