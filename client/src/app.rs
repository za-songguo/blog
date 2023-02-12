use crate::components::{
    article::article_viewer::ArticleViewer, container::Container, home::Home, not_found::NotFound,
};

use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

/// 主路由
#[derive(Debug, Clone, PartialEq, Eq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/article/:article_id")]
    ArticleViewer { article_id: u32 },
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(route: Route) -> Html {
    html! {
        <Container>
        {
            match route {
                Route::Home => html! { <Home/> },
                Route::ArticleViewer{article_id} => html! { <ArticleViewer {article_id}/> },
                Route::NotFound => html! { <NotFound/> },
            }
        }
        </Container>
    }
}
