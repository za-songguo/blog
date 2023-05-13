use crate::{
    app::Route,
    components::{card::Card, container::AppContext, modal::Modal},
    fetch,
    models::user::{Login, User},
};

use gloo::net::http::Method;
use yew::prelude::*;
use yew_router::prelude::*;

/// Github 登录后携带 query 参数跳回来，我们在这里向服务端发起登录请求
#[function_component(OAuth)]
pub fn oauth() -> Html {
    let context = use_context::<AppContext>().unwrap();

    // 设置网页标题
    context.set_title.emit("稍等".into());

    let loading = use_state(|| true);
    let message = use_state(|| Err("".into()));

    // 登录后拿到的用户信息
    let user_info = use_state(|| Err("".into()));

    let location = use_location().unwrap();

    // 解析 query，获取登录所需的 code
    let login = location.query::<Login>();

    if login.is_err() {
        // 解析 query 时发生了错误
        return html! {
            <Modal title={"错误"}>
            { "无法解析请求参数，可能是提供了不正确的地址，请尝试重新登录" }
            </Modal>
        };
    }

    {
        let message = message.clone();
        let loading = loading.clone();

        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    message.set(
                        // 服务端会返回消息，所以用这个函数
                        fetch::fetch_without_deserialize(
                            "/api/user/login".into(),
                            Method::POST,
                            Some(serde_json::to_string(&login.unwrap()).unwrap()),
                            // 注意 Content-Type
                            Some("application/json".into()),
                        )
                        .await,
                    );

                    user_info.set(
                        fetch::fetch::<User>("/api/user/info".into(), Method::GET, None, None)
                            .await,
                    );

                    // 确保这个更新的操作只执行一次（挂载时）
                    if user_info.is_ok() {
                        // 设置 Context 里的 User
                        context.user.set((*user_info).clone());
                    }

                    loading.set(false);
                });
            },
            (),
        );
    }

    let navigator = use_navigator().unwrap();

    // 成功登录时展示的返回首页的按钮
    let footer = html! {
        <button onclick={Callback::from(move |_| {
            navigator.push(&Route::Home)
        })}>{ "回首页" }</button>
    };

    html! {
        if *loading {
            <Card title={"Loading..."}>{ "马上就好......" }</Card>
        } else {
            if let Ok(m) = &*message {
                // 服务端返回的问候语
                <Modal title={"登录成功"} {footer}>{ m }</Modal>
            } else if let Err(e) = &*message {
                // 错误弹窗
                <Modal title={"错误"}>{ e }</Modal>
            }
        }
    }
}
